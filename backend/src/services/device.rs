use std::sync::Arc;

use async_trait::async_trait;
use validator::Validate;

use crate::{
    errors::Result,
    models::devices::{CreateDevice, Device, UpdateDevice},
    repositories::device_repository::DeviceRepositoryTrait,
};

#[async_trait]
pub trait DeviceServiceTrait {
    async fn create_device(&self, new_device: CreateDevice) -> Result<Device>;
    async fn get_device_by_id(&self, id: i64) -> Result<Device>;
    async fn update_device(&self, id: i64, updated_device: UpdateDevice) -> Result<Device>;
    async fn delete_device(&self, id: i64) -> Result<()>;
    async fn get_devices_by_room_id(&self, room_id: i64) -> Result<Vec<Device>>;
    async fn get_devices_by_house_id(&self, house_id: i64) -> Result<Vec<Device>>;
}

#[derive(Clone)]
pub struct DeviceService {
    device_repository: Arc<dyn DeviceRepositoryTrait + Send + Sync>,
}

impl DeviceService {
    pub fn new(device_repository: Arc<dyn DeviceRepositoryTrait + Send + Sync>) -> Self {
        Self { device_repository }
    }
}

#[async_trait]
impl DeviceServiceTrait for DeviceService {
    async fn create_device(&self, new_device: CreateDevice) -> Result<Device> {
        new_device.validate()?;
        self.device_repository.create_device(new_device).await
    }

    async fn get_device_by_id(&self, id: i64) -> Result<Device> {
        self.device_repository.get_device_by_id(id).await
    }

    async fn update_device(&self, id: i64, updated_device: UpdateDevice) -> Result<Device> {
        updated_device.validate()?;
        self.device_repository
            .update_device(id, updated_device)
            .await
    }

    async fn delete_device(&self, id: i64) -> Result<()> {
        self.device_repository.delete_device(id).await
    }

    async fn get_devices_by_room_id(&self, room_id: i64) -> Result<Vec<Device>> {
        self.device_repository.get_devices_by_room_id(room_id).await
    }

    async fn get_devices_by_house_id(&self, house_id: i64) -> Result<Vec<Device>> {
        self.device_repository
            .get_devices_by_house_id(house_id)
            .await
    }
}
