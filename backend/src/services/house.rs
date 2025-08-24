use std::{borrow::Cow, sync::Arc};

use async_trait::async_trait;
use mockall::automock;
use validator::ValidationErrors;

use crate::{
    errors::{AppError, Result},
    models::houses::{House, NewHouse},
    repositories::{user_houses_repository::UserHousesRepositoryTrait, HouseRepositoryTrait},
};

#[automock]
#[async_trait]
pub trait HouseServiceTrait {
    async fn get_user_houses(&self, user_id: i64) -> Result<Vec<House>>;
    async fn get_house_by_id(&self, id: i64) -> Result<House>;
    async fn create_house(&self, user_id: i64, new_house: NewHouse) -> Result<House>;
    async fn delete_house(&self, id: i64) -> Result<()>;
}

#[derive(Clone)]
pub struct HouseService {
    house_repository: Arc<dyn HouseRepositoryTrait + Send + Sync>,
    user_house_repository: Arc<dyn UserHousesRepositoryTrait + Send + Sync>,
}

impl HouseService {
    pub fn new(
        house_repository: Arc<dyn HouseRepositoryTrait + Send + Sync>,
        user_house_repository: Arc<dyn UserHousesRepositoryTrait + Send + Sync>,
    ) -> Self {
        Self {
            house_repository,
            user_house_repository,
        }
    }
}

#[async_trait]
impl HouseServiceTrait for HouseService {
    async fn get_house_by_id(&self, id: i64) -> Result<House> {
        let house = self.house_repository.get_house_by_id(id).await?;

        Ok(house)
    }

    async fn get_user_houses(&self, user_id: i64) -> Result<Vec<House>> {
        let houses = self.house_repository.get_user_houses(user_id).await?;

        Ok(houses)
    }

    async fn create_house(&self, user_id: i64, new_house: NewHouse) -> Result<House> {
        let house = self
            .house_repository
            .find_house_by_address(new_house.address.clone().unwrap())
            .await?;
        if house.is_some() {
            let mut errors = ValidationErrors::new();
            errors.add(
                "address",
                validator::ValidationError::new("already_exists")
                    .with_message(Cow::from("House with this address already exists")),
            );
            return Err(AppError::ValidationError(errors));
        }
        let house = self.house_repository.create_house(new_house).await?;
        let _ = self
            .user_house_repository
            .add_house_to_user(user_id, house.id)
            .await?;

        Ok(house)
    }

    async fn delete_house(&self, id: i64) -> Result<()> {
        self.house_repository.delete_house(id).await?;

        Ok(())
    }
}
