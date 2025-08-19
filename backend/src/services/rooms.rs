use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;

use crate::{
    errors::Result,
    models::rooms::{NewRoom, Room},
    repositories::rooms_repository::RoomsRepositoryTrait,
};

#[automock]
#[async_trait]
pub trait RoomsServiceTrait {
    async fn get_house_rooms(&self, house_id: i64) -> Result<Vec<Room>>;
    async fn create_house_room(&self, house_id: i64, room: NewRoom) -> Result<Room>;
    async fn delete_room(&self, room_id: i64) -> Result<()>;
}

#[derive(Clone)]
pub struct RoomsService {
    rooms_repository: Arc<dyn RoomsRepositoryTrait + Send + Sync>,
}

impl RoomsService {
    pub fn new(rooms_repository: Arc<dyn RoomsRepositoryTrait + Send + Sync>) -> Self {
        Self { rooms_repository }
    }
}

#[async_trait]
impl RoomsServiceTrait for RoomsService {
    async fn get_house_rooms(&self, house_id: i64) -> Result<Vec<Room>> {
        let rooms = self.rooms_repository.get_house_rooms(house_id).await?;

        Ok(rooms)
    }

    async fn create_house_room(&self, house_id: i64, room: NewRoom) -> Result<Room> {
        let room = self
            .rooms_repository
            .create_house_room(house_id, room)
            .await?;

        Ok(room)
    }

    async fn delete_room(&self, room_id: i64) -> Result<()> {
        self.rooms_repository.delete_room(room_id).await
    }
}
