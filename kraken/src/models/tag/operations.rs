use rorm::conditions::DynamicCollection;
use rorm::db::Executor;
use rorm::prelude::*;
use rorm::{and, insert, query};
use thiserror::Error;
use uuid::Uuid;

use super::{GlobalTag, Workspace, WorkspaceTag};
use crate::api::handler::common::error::ApiError;
use crate::models::Color;

impl From<Color> for i32 {
    fn from(value: Color) -> Self {
        i32::from_le_bytes([value.r, value.g, value.b, value.a])
    }
}

impl From<i32> for Color {
    fn from(value: i32) -> Self {
        let [r, g, b, a] = value.to_le_bytes();
        Self { r, g, b, a }
    }
}

#[derive(Patch)]
#[rorm(model = "GlobalTag")]
struct GlobalTagInsert {
    uuid: Uuid,
    name: String,
    color: i32,
}

#[derive(Patch)]
#[rorm(model = "WorkspaceTag")]
struct WorkspaceTagInsert {
    uuid: Uuid,
    name: String,
    color: i32,
    workspace: ForeignModel<Workspace>,
}

/// The errors that can occur while inserting a [GlobalTag]
#[derive(Debug, Error)]
pub enum InsertGlobalTagError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] rorm::Error),
    #[error("Invalid name")]
    InvalidName,
    #[error("Name already occupied")]
    NameAlreadyExists,
}

impl From<InsertGlobalTagError> for ApiError {
    fn from(value: InsertGlobalTagError) -> Self {
        match value {
            InsertGlobalTagError::DatabaseError(x) => ApiError::DatabaseError(x),
            InsertGlobalTagError::InvalidName => ApiError::InvalidName,
            InsertGlobalTagError::NameAlreadyExists => ApiError::NameAlreadyExists,
        }
    }
}

impl GlobalTag {
    /// Check whether a global tag exists by querying its name
    pub async fn exists_by_name(
        executor: impl Executor<'_>,
        name: &str,
    ) -> Result<bool, rorm::Error> {
        Ok(query!(executor, (GlobalTag::F.uuid,))
            .condition(GlobalTag::F.name.equals(name))
            .optional()
            .await?
            .is_some())
    }

    /// Check whether all global tags in a list exist by quering their uuids
    ///
    /// This function returns a `Option<()>` instead of a `bool` to allow easier error propagation:
    /// ```norun
    /// fn example(db: &Database, global_tags: Vec<Uuid>) -> Result<(), ApiError> {
    ///     GlobalTag::exist_all(db, global_tags.iter().copied())
    ///         .await?
    ///         .ok_or(ApiError::InvalidUuid)?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn exist_all(
        executor: impl Executor<'_>,
        uuids: impl IntoIterator<Item = Uuid>,
    ) -> Result<Option<()>, rorm::Error> {
        let tags: Vec<_> = uuids
            .into_iter()
            .map(|uuid| GlobalTag::F.uuid.equals(uuid))
            .collect();

        // Short circuit if the there are no uuids to check
        if tags.is_empty() {
            return Ok(Some(()));
        }

        let search = tags.len();
        let (found,) = query!(executor, (GlobalTag::F.uuid.count(),))
            .condition(DynamicCollection::or(tags))
            .one()
            .await?;
        Ok((found == search as i64).then_some(()))
    }

    /// Insert a [GlobalTag]
    pub async fn insert(
        executor: impl Executor<'_>,
        name: String,
        color: Color,
    ) -> Result<Uuid, InsertGlobalTagError> {
        let mut guard = executor.ensure_transaction().await?;

        if name.is_empty() {
            return Err(InsertGlobalTagError::InvalidName);
        }

        if GlobalTag::exists_by_name(guard.get_transaction(), &name).await? {
            return Err(InsertGlobalTagError::NameAlreadyExists);
        }

        let uuid = Uuid::new_v4();
        insert!(guard.get_transaction(), GlobalTagInsert)
            .return_nothing()
            .single(&GlobalTagInsert {
                uuid,
                name,
                color: color.into(),
            })
            .await?;

        guard.commit().await?;

        Ok(uuid)
    }
}

/// The errors that can occur while inserting a [WorkspaceTag]
#[derive(Debug, Error)]
pub enum InsertWorkspaceTagError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] rorm::Error),
    #[error("Invalid name")]
    InvalidName,
    #[error("Name already occupied")]
    NameAlreadyExists,
    #[error("Workspace does not exist")]
    WorkspaceDoesNotExist,
}

impl From<InsertWorkspaceTagError> for ApiError {
    fn from(value: InsertWorkspaceTagError) -> Self {
        match value {
            InsertWorkspaceTagError::DatabaseError(x) => ApiError::DatabaseError(x),
            InsertWorkspaceTagError::InvalidName => ApiError::InvalidName,
            InsertWorkspaceTagError::NameAlreadyExists => ApiError::NameAlreadyExists,
            InsertWorkspaceTagError::WorkspaceDoesNotExist => ApiError::InvalidUuid,
        }
    }
}

impl WorkspaceTag {
    /// Check whether a workspace tag exists by querying its name
    pub async fn exists_by_name(
        executor: impl Executor<'_>,
        name: &str,
        workspace_uuid: Uuid,
    ) -> Result<bool, rorm::Error> {
        Ok(query!(executor, (WorkspaceTag::F.uuid,))
            .condition(and!(
                WorkspaceTag::F.name.equals(name),
                WorkspaceTag::F.workspace.equals(workspace_uuid)
            ))
            .optional()
            .await?
            .is_some())
    }

    /// Check whether all workspace tags in a list exist by quering their uuids
    ///
    /// This function returns a `Option<()>` instead of a `bool` to allow easier error propagation:
    /// ```norun
    /// fn example(db: &Database, workspace_tags: Vec<Uuid>) -> Result<(), ApiError> {
    ///     WorkspaceTag::exist_all(db, workspace_tags.iter().copied())
    ///         .await
    ///         .ok_or(ApiError::InvalidUuid)?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn exist_all(
        executor: impl Executor<'_>,
        uuids: impl IntoIterator<Item = Uuid>,
    ) -> Result<Option<()>, rorm::Error> {
        let tags: Vec<_> = uuids
            .into_iter()
            .map(|uuid| WorkspaceTag::F.uuid.equals(uuid))
            .collect();

        // Short circuit if the there are no uuids to check
        if tags.is_empty() {
            return Ok(Some(()));
        }

        let search = tags.len();
        let (found,) = query!(executor, (WorkspaceTag::F.uuid.count(),))
            .condition(DynamicCollection::or(tags))
            .one()
            .await?;
        Ok((found == search as i64).then_some(()))
    }

    /// Insert a [WorkspaceTag]
    pub async fn insert(
        executor: impl Executor<'_>,
        name: String,
        color: Color,
        workspace_uuid: Uuid,
    ) -> Result<Uuid, InsertWorkspaceTagError> {
        let mut guard = executor.ensure_transaction().await?;

        if name.is_empty() {
            return Err(InsertWorkspaceTagError::InvalidName);
        }

        if GlobalTag::exists_by_name(guard.get_transaction(), &name).await? {
            return Err(InsertWorkspaceTagError::NameAlreadyExists);
        }

        if !Workspace::exists(guard.get_transaction(), workspace_uuid).await? {
            return Err(InsertWorkspaceTagError::WorkspaceDoesNotExist);
        }

        let uuid = Uuid::new_v4();
        insert!(guard.get_transaction(), WorkspaceTagInsert)
            .return_nothing()
            .single(&WorkspaceTagInsert {
                uuid,
                name,
                color: color.into(),
                workspace: ForeignModelByField::Key(workspace_uuid),
            })
            .await?;

        guard.commit().await?;

        Ok(uuid)
    }
}
