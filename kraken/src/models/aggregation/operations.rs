use rorm::db::Executor;
use rorm::prelude::*;
use rorm::{and, insert, query, update};
use uuid::Uuid;

use crate::models::{DomainDomainRelation, DomainHostRelation, PortProtocol, ServiceProtocols};

impl DomainDomainRelation {
    /// Insert a [`CnameRelation`] if it doesn't exist yet.
    pub async fn insert_if_missing(
        executor: impl Executor<'_>,
        workspace: Uuid,
        source: Uuid,
        destination: Uuid,
    ) -> Result<(), rorm::Error> {
        let mut guard = executor.ensure_transaction().await?;
        let tx = guard.get_transaction();

        if query!(&mut *tx, (DomainDomainRelation::F.uuid,))
            .condition(and![
                DomainDomainRelation::F.source.equals(source),
                DomainDomainRelation::F.destination.equals(destination)
            ])
            .optional()
            .await?
            .is_none()
        {
            insert!(&mut *tx, DomainDomainRelation)
                .return_nothing()
                .single(&DomainDomainRelation {
                    uuid: Uuid::new_v4(),
                    source: ForeignModelByField::Key(source),
                    destination: ForeignModelByField::Key(destination),
                    workspace: ForeignModelByField::Key(workspace),
                })
                .await?;

            // Create direct domain -> host relations
            for (host,) in query!(&mut *tx, (DomainHostRelation::F.host,))
                .condition(DomainHostRelation::F.domain.equals(destination))
                .all()
                .await?
            {
                DomainHostRelation::insert_if_missing(
                    &mut *tx,
                    workspace,
                    source,
                    *host.key(),
                    false,
                )
                .await?;
            }
        }

        guard.commit().await?;
        Ok(())
    }
}

impl DomainHostRelation {
    /// Insert a [`DomainHostRelation`] if it doesn't exist yet.
    ///
    /// Indirect relations are created implicitly by [`CnameRelation::insert_if_missing`].
    pub async fn insert_if_missing(
        executor: impl Executor<'_>,
        workspace: Uuid,
        domain: Uuid,
        host: Uuid,
        is_direct: bool,
    ) -> Result<(), rorm::Error> {
        let mut guard = executor.ensure_transaction().await?;
        let tx = guard.get_transaction();

        match query!(
            &mut *tx,
            (DomainHostRelation::F.uuid, DomainHostRelation::F.is_direct)
        )
        .condition(and![
            DomainHostRelation::F.domain.equals(domain),
            DomainHostRelation::F.host.equals(host)
        ])
        .optional()
        .await?
        {
            None => {
                insert!(&mut *tx, DomainHostRelation)
                    .return_nothing()
                    .single(&DomainHostRelation {
                        uuid: Uuid::new_v4(),
                        domain: ForeignModelByField::Key(domain),
                        host: ForeignModelByField::Key(host),
                        workspace: ForeignModelByField::Key(workspace),
                        is_direct: true,
                    })
                    .await?;
            }
            Some((uuid, false)) if is_direct => {
                update!(&mut *tx, DomainHostRelation)
                    .set(DomainHostRelation::F.is_direct, true)
                    .condition(DomainHostRelation::F.uuid.equals(uuid))
                    .await?;
            }
            _ => {}
        }

        guard.commit().await?;
        Ok(())
    }
}

const RAW: usize = 0;
const TLS_TCP: usize = 1;

impl PortProtocol {
    /// Decodes [`ServiceProtocols`] based on the service's port's [`PortProtocol`].
    pub fn decode_service(&self, value: i16) -> ServiceProtocols {
        let read_bit = |bit| (value & (1i16 << bit)) > 0;
        match self {
            PortProtocol::Unknown => ServiceProtocols::Unknown {},
            PortProtocol::Tcp => ServiceProtocols::Tcp {
                raw: read_bit(RAW),
                tls: read_bit(TLS_TCP),
            },
            PortProtocol::Udp => ServiceProtocols::Udp { raw: read_bit(RAW) },
            PortProtocol::Sctp => ServiceProtocols::Sctp { raw: read_bit(RAW) },
        }
    }
}

impl ServiceProtocols {
    /// Encodes the [`ServiceProtocols`] into its database format.
    ///
    /// Use [`PortProtocol::decode_service`] for decoding.
    pub fn encode(self) -> i16 {
        let mut result = 0;
        let mut set_bit = |bit| result |= 1i16 << bit;
        match self {
            ServiceProtocols::Unknown {} => {}
            ServiceProtocols::Tcp { raw, tls } => {
                if raw {
                    set_bit(RAW);
                }
                if tls {
                    set_bit(TLS_TCP);
                }
            }
            ServiceProtocols::Udp { raw } => {
                if raw {
                    set_bit(RAW);
                }
            }
            ServiceProtocols::Sctp { raw } => {
                if raw {
                    set_bit(RAW);
                }
            }
        }
        result
    }
}
