use std::sync::Arc;

use async_trait::async_trait;
use validator::Validate;

use crate::{
    errors::Result,
    models::device_metrics::{
        AggregatedDeviceMetric, CreateDeviceMetric, DeviceMetric, DeviceMetricFilters,
    },
    repositories::device_metrics_repository::DeviceMetricsRepositoryTrait,
    services::access_control_service::AccessControlServiceTrait,
};

#[async_trait]
pub trait DeviceMetricsServiceTrait {
    async fn create_metric(
        &self,
        user_id: i64,
        new_metric: CreateDeviceMetric,
    ) -> Result<DeviceMetric>;
    async fn get_metrics(
        &self,
        user_id: i64,
        device_id: i64,
        filters: DeviceMetricFilters,
    ) -> Result<Vec<DeviceMetric>>;
    async fn get_metrics_for_room(
        &self,
        user_id: i64,
        room_id: i64,
        filters: DeviceMetricFilters,
    ) -> Result<Vec<DeviceMetric>>;
    async fn get_metrics_for_house(
        &self,
        user_id: i64,
        house_id: i64,
        filters: DeviceMetricFilters,
    ) -> Result<Vec<DeviceMetric>>;
    async fn get_aggregated_metrics_for_room(
        &self,
        user_id: i64,
        room_id: i64,
        filters: DeviceMetricFilters,
    ) -> Result<Vec<AggregatedDeviceMetric>>;
    async fn get_aggregated_metrics_for_house(
        &self,
        user_id: i64,
        house_id: i64,
        filters: DeviceMetricFilters,
    ) -> Result<Vec<AggregatedDeviceMetric>>;
}

#[derive(Clone)]
pub struct DeviceMetricsService {
    device_metrics_repository: Arc<dyn DeviceMetricsRepositoryTrait + Send + Sync>,
    access_control_service: Arc<dyn AccessControlServiceTrait + Send + Sync>,
}

impl DeviceMetricsService {
    pub fn new(
        device_metrics_repository: Arc<dyn DeviceMetricsRepositoryTrait + Send + Sync>,
        access_control_service: Arc<dyn AccessControlServiceTrait + Send + Sync>,
    ) -> Self {
        Self {
            device_metrics_repository,
            access_control_service,
        }
    }
}

#[async_trait]
impl DeviceMetricsServiceTrait for DeviceMetricsService {
    async fn create_metric(
        &self,
        user_id: i64,
        new_metric: CreateDeviceMetric,
    ) -> Result<DeviceMetric> {
        new_metric.validate()?;
        self.access_control_service
            .can_access_device(user_id, new_metric.device_id)
            .await?;
        self.device_metrics_repository
            .create_metric(new_metric)
            .await
    }

    async fn get_metrics(
        &self,
        user_id: i64,
        device_id: i64,
        filters: DeviceMetricFilters,
    ) -> Result<Vec<DeviceMetric>> {
        self.access_control_service
            .can_access_device(user_id, device_id)
            .await?;
        self.device_metrics_repository
            .get_metrics(device_id, filters)
            .await
    }

    async fn get_metrics_for_room(
        &self,
        user_id: i64,
        room_id: i64,
        filters: DeviceMetricFilters,
    ) -> Result<Vec<DeviceMetric>> {
        self.access_control_service
            .can_access_room(user_id, room_id)
            .await?;
        self.device_metrics_repository
            .get_metrics_for_room(room_id, filters)
            .await
    }

    async fn get_metrics_for_house(
        &self,
        user_id: i64,
        house_id: i64,
        filters: DeviceMetricFilters,
    ) -> Result<Vec<DeviceMetric>> {
        self.access_control_service
            .can_access_house(user_id, house_id)
            .await?;
        self.device_metrics_repository
            .get_metrics_for_house(house_id, filters)
            .await
    }

    async fn get_aggregated_metrics_for_room(
        &self,
        user_id: i64,
        room_id: i64,
        filters: DeviceMetricFilters,
    ) -> Result<Vec<AggregatedDeviceMetric>> {
        self.access_control_service
            .can_access_room(user_id, room_id)
            .await?;
        self.device_metrics_repository
            .get_aggregated_metrics_for_room(room_id, filters)
            .await
    }

    async fn get_aggregated_metrics_for_house(
        &self,
        user_id: i64,
        house_id: i64,
        filters: DeviceMetricFilters,
    ) -> Result<Vec<AggregatedDeviceMetric>> {
        self.access_control_service
            .can_access_house(user_id, house_id)
            .await?;
        self.device_metrics_repository
            .get_aggregated_metrics_for_house(house_id, filters)
            .await
    }
}
