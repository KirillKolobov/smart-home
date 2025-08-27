use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;

use crate::{
    errors::{AppError, Result},
    models::users::User,
    repositories::user_houses_repository::UserHousesRepositoryTrait,
};

#[automock]
#[async_trait]
pub trait AccessControlServiceTrait {
    async fn validate_house_access(&self, house_id: i64, user_id: i64) -> Result<bool>;
    async fn can_access_device(&self, user: &User, device_id: i64) -> Result<()>;
}

#[derive(Clone)]
pub struct AccessControlService {
    user_houses_repo: Arc<dyn UserHousesRepositoryTrait + Send + Sync>,
}

impl AccessControlService {
    pub fn new(user_houses_repo: Arc<dyn UserHousesRepositoryTrait + Send + Sync>) -> Self {
        Self { user_houses_repo }
    }
}

#[async_trait]
impl AccessControlServiceTrait for AccessControlService {
    async fn validate_house_access(&self, house_id: i64, user_id: i64) -> Result<bool> {
        let result = self
            .user_houses_repo
            .user_has_access_to_house(house_id, user_id)
            .await?;
        if !result {
            return Err(AppError::AuthenticationError("Access denied".to_string()));
        }

        Ok(result)
    }

    async fn can_access_device(&self, user: &User, device_id: i64) -> Result<()> {
        let device_house = self
            .user_houses_repo
            .get_house_by_device_id(device_id)
            .await?;
        self.validate_house_access(device_house.id, user.id).await?;
        Ok(())
    }
}
