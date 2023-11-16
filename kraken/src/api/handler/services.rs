//! This module holds the aggregated data of services

use std::collections::HashMap;

use actix_toolbox::tb_middleware::Session;
use actix_web::web::{Data, Json, Path, Query};
use actix_web::{get, put, HttpResponse};
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use rorm::conditions::{BoxedCondition, Condition, DynamicCollection};
use rorm::prelude::ForeignModelByField;
use rorm::{and, insert, query, update, Database, FieldAccess, Model};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::api::extractors::SessionUser;
use crate::api::handler::hosts::SimpleHost;
use crate::api::handler::ports::SimplePort;
use crate::api::handler::{
    get_page_params, ApiError, ApiResult, PageParams, PathUuid, ServiceResultsPage,
    SimpleAggregationSource, SimpleTag, TagType,
};
use crate::models::{
    AggregationSource, AggregationTable, GlobalTag, Host, Port, Service, ServiceCertainty,
    ServiceGlobalTag, ServiceWorkspaceTag, Workspace, WorkspaceTag,
};
use crate::query_tags;

/// Query parameters for filtering the services to get
#[derive(Deserialize, IntoParams)]
pub struct GetAllServicesQuery {
    /// Only get services associated with a specific host
    pub host: Option<Uuid>,
}

/// A simple representation of a service
#[derive(Serialize, ToSchema)]
pub struct SimpleService {
    pub(crate) uuid: Uuid,
    #[schema(example = "postgresql")]
    pub(crate) name: String,
    #[schema(example = "13.0.1")]
    pub(crate) version: Option<String>,
    pub(crate) host: Uuid,
    pub(crate) port: Option<Uuid>,
    #[schema(example = "Holds all relevant information")]
    pub(crate) comment: String,
    pub(crate) workspace: Uuid,
    /// The point in time, the record was created
    pub created_at: DateTime<Utc>,
}

/// A full representation of a service
#[derive(Serialize, ToSchema)]
pub struct FullService {
    /// Uuid of the service
    pub uuid: Uuid,
    /// The service's name
    #[schema(example = "postgresql")]
    pub name: String,
    /// An optional version of the running service
    #[schema(example = "13.0.1")]
    pub version: Option<String>,
    /// The certainty of the detection
    pub certainty: ServiceCertainty,
    /// The host this service is assigned to
    pub host: SimpleHost,
    /// An optional port this service listens on
    pub port: Option<SimplePort>,
    /// A comment to the service
    #[schema(example = "Holds all relevant information")]
    pub comment: String,
    /// The workspace this service is linked to
    pub workspace: Uuid,
    /// The tags this service is linked to
    pub tags: Vec<SimpleTag>,
    /// The number of attacks which found this host
    pub sources: SimpleAggregationSource,
    /// The point in time, the record was created
    pub created_at: DateTime<Utc>,
}

/// List the services of a workspace
#[utoipa::path(
    tag = "Services",
    context_path = "/api/v1",
    responses(
        (status = 200, description = "Retrieve all services of a workspace", body = ServiceResultsPage),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse),
    ),
    params(PathUuid, PageParams, GetAllServicesQuery),
    security(("api_key" = []))
)]
#[get("/workspaces/{uuid}/services")]
pub async fn get_all_services(
    path: Path<PathUuid>,
    page_params: Query<PageParams>,
    filter_params: Query<GetAllServicesQuery>,
    db: Data<Database>,
    session: Session,
) -> ApiResult<Json<ServiceResultsPage>> {
    let path = path.into_inner();
    let user_uuid: Uuid = session.get("uuid")?.ok_or(ApiError::SessionCorrupt)?;

    let mut tx = db.start_transaction().await?;

    if !Workspace::is_user_member_or_owner(&mut tx, path.uuid, user_uuid).await? {
        return Err(ApiError::MissingPrivileges);
    }

    let (limit, offset) = get_page_params(page_params).await?;

    fn build_condition(workspace: Uuid, filter_params: &GetAllServicesQuery) -> BoxedCondition<'_> {
        match filter_params {
            GetAllServicesQuery { host: Some(host) } => and![
                Service::F.workspace.equals(workspace),
                Service::F.host.equals(*host)
            ]
            .boxed(),
            GetAllServicesQuery { host: None } => Service::F.workspace.equals(workspace).boxed(),
        }
    }

    let (total,) = query!(&mut tx, (Service::F.uuid.count()))
        .condition(build_condition(path.uuid, &filter_params))
        .one()
        .await?;

    let services = query!(
        &mut tx,
        (
            Service::F.uuid,
            Service::F.name,
            Service::F.version,
            Service::F.certainty,
            Service::F.comment,
            Service::F.created_at,
            Service::F.host as Host,
            Service::F.port,
            Service::F.workspace,
        )
    )
    .condition(build_condition(path.uuid, &filter_params))
    .order_desc(Service::F.created_at)
    .limit(limit)
    .offset(offset)
    .all()
    .await?;

    let mut ports = HashMap::new();
    let p: Vec<_> = services
        .iter()
        .filter_map(|x| x.7.as_ref().map(|y| Port::F.uuid.equals(*y.key())))
        .collect();

    if !p.is_empty() {
        let mut port_stream = query!(&mut tx, Port)
            .condition(DynamicCollection::or(p))
            .stream();

        while let Some(port) = port_stream.try_next().await? {
            ports.insert(
                port.uuid,
                SimplePort {
                    uuid: port.uuid,
                    port: u16::from_ne_bytes(port.port.to_ne_bytes()),
                    protocol: port.protocol,
                    comment: port.comment,
                    created_at: port.created_at,
                    workspace: *port.workspace.key(),
                    host: *port.host.key(),
                },
            );
        }
    }

    let mut tags = HashMap::new();

    query_tags!(
        tags,
        tx,
        (
            ServiceWorkspaceTag::F.workspace_tag as WorkspaceTag,
            ServiceWorkspaceTag::F.service
        ),
        ServiceWorkspaceTag::F.service,
        (
            ServiceGlobalTag::F.global_tag as GlobalTag,
            ServiceGlobalTag::F.service
        ),
        ServiceGlobalTag::F.service,
        services.iter().map(|x| x.0)
    );

    let mut sources = SimpleAggregationSource::query(
        &mut tx,
        path.uuid,
        AggregationTable::Service,
        services.iter().map(|x| x.0),
    )
    .await?;

    tx.commit().await?;

    let items = services
        .into_iter()
        .map(
            |(uuid, name, version, certainty, comment, created_at, host, port, workspace)| {
                FullService {
                    uuid,
                    name,
                    version,
                    certainty,
                    comment,
                    host: SimpleHost {
                        uuid: host.uuid,
                        ip_addr: host.ip_addr.ip().to_string(),
                        os_type: host.os_type,
                        comment: host.comment,
                        workspace: *host.workspace.key(),
                        created_at: host.created_at,
                    },
                    port: port.map(|y| ports.remove(y.key()).unwrap()),
                    workspace: *workspace.key(),
                    tags: tags.remove(&uuid).unwrap_or_default(),
                    sources: sources.remove(&uuid).unwrap_or_default(),
                    created_at,
                }
            },
        )
        .collect();

    Ok(Json(ServiceResultsPage {
        items,
        limit,
        offset,
        total: total as u64,
    }))
}

/// The path parameter of a service
#[derive(Deserialize, IntoParams)]
pub struct PathService {
    /// The workspace's uuid
    pub w_uuid: Uuid,
    /// The service's uuid
    pub s_uuid: Uuid,
}

/// Retrieve all information about a single service
#[utoipa::path(
    tag = "Services",
    context_path = "/api/v1",
    responses(
        (status = 200, description = "Retrieved the selected service", body = FullService),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse),
    ),
    params(PathService),
    security(("api_key" = []))
)]
#[get("/workspaces/{w_uuid}/services/{s_uuid}")]
pub async fn get_service(
    path: Path<PathService>,
    db: Data<Database>,
    SessionUser(user_uuid): SessionUser,
) -> ApiResult<Json<FullService>> {
    let mut tx = db.start_transaction().await?;

    if !Workspace::is_user_member_or_owner(&mut tx, path.w_uuid, user_uuid).await? {
        return Err(ApiError::MissingPrivileges)?;
    }

    let service = query!(&mut tx, Service)
        .condition(and!(
            Service::F.workspace.equals(path.w_uuid),
            Service::F.uuid.equals(path.s_uuid)
        ))
        .optional()
        .await?
        .ok_or(ApiError::InvalidUuid)?;

    let host = query!(&mut tx, Host)
        .condition(Host::F.uuid.equals(*service.host.key()))
        .one()
        .await?;

    let port = if let Some(port) = service.port.as_ref() {
        Some(
            query!(&mut tx, Port)
                .condition(and!(
                    Port::F.workspace.equals(path.w_uuid),
                    Port::F.uuid.equals(*port.key())
                ))
                .one()
                .await?,
        )
    } else {
        None
    };

    let mut tags: Vec<_> = query!(&mut tx, (ServiceGlobalTag::F.global_tag as GlobalTag,))
        .condition(ServiceGlobalTag::F.service.equals(path.s_uuid))
        .stream()
        .map_ok(|(x,)| SimpleTag {
            uuid: x.uuid,
            name: x.name,
            color: x.color.into(),
            tag_type: TagType::Global,
        })
        .try_collect()
        .await?;

    let global_tags: Vec<_> = query!(
        &mut tx,
        (ServiceWorkspaceTag::F.workspace_tag as WorkspaceTag,)
    )
    .condition(ServiceWorkspaceTag::F.service.equals(path.s_uuid))
    .stream()
    .map_ok(|(x,)| SimpleTag {
        uuid: x.uuid,
        name: x.name,
        color: x.color.into(),
        tag_type: TagType::Workspace,
    })
    .try_collect()
    .await?;

    tags.extend(global_tags);

    let sources = query!(&mut tx, (AggregationSource::F.source_type,))
        .condition(AggregationSource::F.aggregated_uuid.equals(host.uuid))
        .stream()
        .map_ok(|(x,)| x)
        .try_collect()
        .await?;

    tx.commit().await?;

    Ok(Json(FullService {
        uuid: path.s_uuid,
        name: service.name,
        version: service.version,
        certainty: service.certainty,
        host: SimpleHost {
            uuid: host.uuid,
            ip_addr: host.ip_addr.ip().to_string(),
            os_type: host.os_type,
            comment: host.comment,
            workspace: path.w_uuid,
            created_at: host.created_at,
        },
        port: port.map(|port| SimplePort {
            uuid: port.uuid,
            port: u16::from_ne_bytes(port.port.to_ne_bytes()),
            protocol: port.protocol,
            host: host.uuid,
            comment: port.comment,
            workspace: path.w_uuid,
            created_at: port.created_at,
        }),
        comment: service.comment,
        workspace: path.w_uuid,
        tags,
        sources,
        created_at: service.created_at,
    }))
}

/// The request to update a service
#[derive(Deserialize, ToSchema)]
pub struct UpdateServiceRequest {
    comment: Option<String>,
    global_tags: Option<Vec<Uuid>>,
    workspace_tags: Option<Vec<Uuid>>,
}

/// Update a service
///
/// You must include at least on parameter
#[utoipa::path(
    tag = "Services",
    context_path = "/api/v1",
    responses(
        (status = 200, description = "Service was updated"),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse),
    ),
    request_body = UpdateServiceRequest,
    params(PathService),
    security(("api_key" = []))
)]
#[put("/workspaces/{w_uuid}/services/{s_uuid}")]
pub async fn update_service(
    req: Json<UpdateServiceRequest>,
    path: Path<PathService>,
    db: Data<Database>,
    SessionUser(user_uuid): SessionUser,
) -> ApiResult<HttpResponse> {
    let req = req.into_inner();

    if req.workspace_tags.is_none() && req.global_tags.is_none() && req.comment.is_none() {
        return Err(ApiError::EmptyJson);
    }

    let mut tx = db.start_transaction().await?;

    if !Workspace::is_user_member_or_owner(&mut tx, path.w_uuid, user_uuid).await? {
        return Err(ApiError::MissingPrivileges);
    }

    query!(&mut tx, (Service::F.uuid,))
        .condition(Service::F.uuid.equals(path.s_uuid))
        .optional()
        .await?
        .ok_or(ApiError::InvalidUuid)?;

    if let Some(global_tags) = req.global_tags {
        GlobalTag::exist_all(&mut tx, global_tags.iter().copied())
            .await?
            .ok_or(ApiError::InvalidUuid)?;

        rorm::delete!(&mut tx, ServiceGlobalTag)
            .condition(ServiceGlobalTag::F.service.equals(path.s_uuid))
            .await?;

        if !global_tags.is_empty() {
            insert!(&mut tx, ServiceGlobalTag)
                .return_nothing()
                .bulk(
                    &global_tags
                        .into_iter()
                        .map(|x| ServiceGlobalTag {
                            uuid: Uuid::new_v4(),
                            service: ForeignModelByField::Key(path.s_uuid),
                            global_tag: ForeignModelByField::Key(x),
                        })
                        .collect::<Vec<_>>(),
                )
                .await?;
        }
    }

    if let Some(workspace_tags) = req.workspace_tags {
        WorkspaceTag::exist_all(&mut tx, workspace_tags.iter().copied())
            .await?
            .ok_or(ApiError::InvalidUuid)?;

        rorm::delete!(&mut tx, ServiceWorkspaceTag)
            .condition(ServiceWorkspaceTag::F.service.equals(path.s_uuid))
            .await?;

        if !workspace_tags.is_empty() {
            insert!(&mut tx, ServiceWorkspaceTag)
                .return_nothing()
                .bulk(
                    &workspace_tags
                        .into_iter()
                        .map(|x| ServiceWorkspaceTag {
                            uuid: Uuid::new_v4(),
                            service: ForeignModelByField::Key(path.s_uuid),
                            workspace_tag: ForeignModelByField::Key(x),
                        })
                        .collect::<Vec<_>>(),
                )
                .await?;
        }
    }

    if let Some(comment) = req.comment {
        update!(&mut tx, Service)
            .condition(Service::F.uuid.equals(path.s_uuid))
            .set(Service::F.comment, comment)
            .exec()
            .await?;
    }

    tx.commit().await?;

    Ok(HttpResponse::Ok().finish())
}
