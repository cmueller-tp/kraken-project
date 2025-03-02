use std::net::IpAddr;

use ipnetwork::IpNetwork;
use kraken_proto::{shared, PortOrRange, ServiceDetectionRequest, ServiceDetectionResponse};
use rorm::insert;
use rorm::prelude::ForeignModelByField;
use uuid::Uuid;

use crate::api::handler::attacks::schema::DomainOrNetwork;
use crate::chan::global::GLOBAL;
use crate::chan::leech_manager::LeechClient;
use crate::models::{
    AggregationSource, AggregationTable, HostCertainty, PortCertainty, PortProtocol,
    ServiceCertainty, ServiceDetectionName, ServiceDetectionResultInsert, ServiceProtocols,
    SourceType,
};
use crate::modules::attacks::{
    AttackContext, AttackError, HandleAttackResponse, ServiceDetectionParams,
};

impl AttackContext {
    /// Executes the "service detection" attack
    pub async fn service_detection(
        &self,
        mut leech: LeechClient,
        params: ServiceDetectionParams,
    ) -> Result<(), AttackError> {
        let targets =
            DomainOrNetwork::resolve(self.workspace.uuid, self.user.uuid, &leech, &params.targets)
                .await?;
        let request = ServiceDetectionRequest {
            attack_uuid: self.attack_uuid.to_string(),
            targets: targets
                .into_iter()
                .map(shared::NetOrAddress::from)
                .collect(),
            ports: params.ports.into_iter().map(PortOrRange::from).collect(),
            connect_timeout: params.connect_timeout,
            receive_timeout: params.receive_timeout,
            concurrent_limit: params.concurrent_limit,
            max_retries: params.max_retries,
            retry_interval: params.retry_interval,
            skip_icmp_check: params.skip_icmp_check,
        };
        self.handle_streamed_response(leech.service_detection(request))
            .await
    }
}

impl HandleAttackResponse<ServiceDetectionResponse> for AttackContext {
    async fn handle_response(&self, response: ServiceDetectionResponse) -> Result<(), AttackError> {
        let ServiceDetectionResponse {
            services,
            response_type,
            address: Some(address),
            port,
        } = response
        else {
            return Err(AttackError::Malformed("Missing `address`"));
        };
        let address = IpAddr::try_from(address)?;
        let host = IpNetwork::from(address);

        let certainty = match response_type {
            1 => ServiceCertainty::MaybeVerified,
            2 => ServiceCertainty::DefinitelyVerified,
            _ => {
                return Err(AttackError::Custom("Retrieved certainty Unknown".into()));
            }
        };

        let mut tx = GLOBAL.db.start_transaction().await?;

        let result_uuid = insert!(&mut tx, ServiceDetectionResultInsert)
            .return_primary_key()
            .single(&ServiceDetectionResultInsert {
                uuid: Uuid::new_v4(),
                attack: ForeignModelByField::Key(self.attack_uuid),
                certainty,
                host,
                port: port as i32,
            })
            .await?;
        insert!(&mut tx, ServiceDetectionName)
            .return_nothing()
            .bulk(services.iter().map(|x| ServiceDetectionName {
                uuid: Uuid::new_v4(),
                name: x.name.to_string(),
                result: ForeignModelByField::Key(result_uuid),
            }))
            .await?;

        let host_uuid = GLOBAL
            .aggregator
            .aggregate_host(self.workspace.uuid, host, HostCertainty::Verified)
            .await?;
        let port_uuid = GLOBAL
            .aggregator
            .aggregate_port(
                self.workspace.uuid,
                host_uuid,
                port as u16,
                PortProtocol::Tcp,
                PortCertainty::Verified,
            )
            .await?;

        let mut service_uuids = Vec::new();
        for service in services {
            service_uuids.push(
                GLOBAL
                    .aggregator
                    .aggregate_service(
                        self.workspace.uuid,
                        host_uuid,
                        Some(port_uuid),
                        Some(ServiceProtocols::Tcp {
                            raw: service.tcp,
                            tls: service.tls,
                        }),
                        &service.name,
                        certainty,
                    )
                    .await?,
            );
        }

        insert!(&mut tx, AggregationSource)
            .return_nothing()
            .bulk(
                [
                    AggregationSource {
                        uuid: Uuid::new_v4(),
                        workspace: ForeignModelByField::Key(self.workspace.uuid),
                        source_type: SourceType::ServiceDetection,
                        source_uuid: result_uuid,
                        aggregated_table: AggregationTable::Host,
                        aggregated_uuid: host_uuid,
                    },
                    AggregationSource {
                        uuid: Uuid::new_v4(),
                        workspace: ForeignModelByField::Key(self.workspace.uuid),
                        source_type: SourceType::ServiceDetection,
                        source_uuid: result_uuid,
                        aggregated_table: AggregationTable::Port,
                        aggregated_uuid: port_uuid,
                    },
                ]
                .into_iter()
                .chain(
                    service_uuids
                        .into_iter()
                        .map(|service_uuid| AggregationSource {
                            uuid: Uuid::new_v4(),
                            workspace: ForeignModelByField::Key(self.workspace.uuid),
                            source_type: SourceType::ServiceDetection,
                            source_uuid: result_uuid,
                            aggregated_table: AggregationTable::Service,
                            aggregated_uuid: service_uuid,
                        }),
                ),
            )
            .await?;

        tx.commit().await?;

        Ok(())
    }
}
