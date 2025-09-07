use async_trait::async_trait;
use mockall::automock;
use sqlx::PgPool;

use crate::{
    errors::Result,
    models::device_metrics::{
        AggregatedDeviceMetric, Aggregation, CreateDeviceMetric, DeviceMetric, DeviceMetricFilters,
    },
};

#[automock]
#[async_trait]
pub trait DeviceMetricsRepositoryTrait {
    async fn create_metric(&self, new_metric: CreateDeviceMetric) -> Result<DeviceMetric>;
    async fn get_metrics(
        &self,
        device_id: i64,
        filters: DeviceMetricFilters,
    ) -> Result<Vec<DeviceMetric>>;
    async fn get_last_metrics_for_room(&self, room_id: i64) -> Result<Vec<DeviceMetric>>;
    async fn get_metrics_for_room(
        &self,
        room_id: i64,
        filters: DeviceMetricFilters,
    ) -> Result<Vec<DeviceMetric>>;
    async fn get_metrics_for_house(
        &self,
        house_id: i64,
        filters: DeviceMetricFilters,
    ) -> Result<Vec<DeviceMetric>>;
    async fn get_aggregated_metrics_for_room(
        &self,
        room_id: i64,
        filters: DeviceMetricFilters,
    ) -> Result<Vec<AggregatedDeviceMetric>>;
    async fn get_aggregated_metrics_for_house(
        &self,
        house_id: i64,
        filters: DeviceMetricFilters,
    ) -> Result<Vec<AggregatedDeviceMetric>>;
}

#[derive(Clone)]
pub struct DeviceMetricsRepository {
    pool: PgPool,
}

impl DeviceMetricsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DeviceMetricsRepositoryTrait for DeviceMetricsRepository {
    async fn create_metric(&self, new_metric: CreateDeviceMetric) -> Result<DeviceMetric> {
        let metric = sqlx::query_as!(
            DeviceMetric,
            r#"
            INSERT INTO device_metrics (device_id, metric_type, metric_value, unit, measured_at)
            VALUES ($1, $2, $3, $4, COALESCE($5, NOW()))
            RETURNING id, device_id, metric_type, metric_value, unit, measured_at, created_at
            "#,
            new_metric.device_id,
            new_metric.metric_type,
            new_metric.metric_value,
            new_metric.unit,
            new_metric.measured_at,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(metric)
    }

    async fn get_metrics(
        &self,
        device_id: i64,
        filters: DeviceMetricFilters,
    ) -> Result<Vec<DeviceMetric>> {
        let mut query = sqlx::QueryBuilder::new(
            "SELECT id, device_id, metric_type, metric_value, unit, measured_at, created_at FROM device_metrics WHERE device_id = ",
        );
        query.push_bind(device_id);

        if let Some(from) = filters.from {
            query.push(" AND measured_at >= ");
            query.push_bind(from);
        }

        if let Some(to) = filters.to {
            query.push(" AND measured_at <= ");
            query.push_bind(to);
        }

        if let Some(unit) = filters.unit {
            query.push(" AND unit = ");
            query.push_bind(unit);
        }

        if let Some(metric_type) = filters.metric_type {
            query.push(" AND metric_type = ");
            query.push_bind(metric_type);
        }

        let metrics = query
            .build_query_as::<DeviceMetric>()
            .fetch_all(&self.pool)
            .await?;

        Ok(metrics)
    }

    async fn get_last_metrics_for_room(&self, room_id: i64) -> Result<Vec<DeviceMetric>> {
        let metrics = sqlx::query_as!(
            DeviceMetric,
            r#"
            SELECT dm.*
            FROM device_metrics dm
            INNER JOIN (
                SELECT device_id, MAX(measured_at) as max_measured_at
                FROM device_metrics
                GROUP BY device_id
            ) last_metrics ON dm.device_id = last_metrics.device_id AND dm.measured_at = last_metrics.max_measured_at
            WHERE dm.device_id IN (SELECT id FROM devices WHERE room_id = $1)
            "#,
            room_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(metrics)
    }

    async fn get_metrics_for_room(
        &self,
        room_id: i64,
        filters: DeviceMetricFilters,
    ) -> Result<Vec<DeviceMetric>> {
        let mut query = sqlx::QueryBuilder::new(
            "SELECT dm.* FROM device_metrics dm WHERE dm.device_id IN (SELECT id FROM devices WHERE room_id = ",
        );
        query.push_bind(room_id);
        query.push(")");

        if let Some(from) = filters.from {
            query.push(" AND measured_at >= ");
            query.push_bind(from);
        }

        if let Some(to) = filters.to {
            query.push(" AND measured_at <= ");
            query.push_bind(to);
        }

        if let Some(unit) = filters.unit {
            query.push(" AND unit = ");
            query.push_bind(unit);
        }

        if let Some(metric_type) = filters.metric_type {
            query.push(" AND metric_type = ");
            query.push_bind(metric_type);
        }

        let metrics = query
            .build_query_as::<DeviceMetric>()
            .fetch_all(&self.pool)
            .await?;

        Ok(metrics)
    }

    async fn get_metrics_for_house(
        &self,
        house_id: i64,
        filters: DeviceMetricFilters,
    ) -> Result<Vec<DeviceMetric>> {
        let mut query = sqlx::QueryBuilder::new(
            "SELECT dm.* FROM device_metrics dm WHERE dm.device_id IN (SELECT d.id FROM devices d JOIN rooms r ON d.room_id = r.id WHERE r.house_id = ",
        );
        query.push_bind(house_id);
        query.push(")");

        if let Some(from) = filters.from {
            query.push(" AND measured_at >= ");
            query.push_bind(from);
        }

        if let Some(to) = filters.to {
            query.push(" AND measured_at <= ");
            query.push_bind(to);
        }

        if let Some(unit) = filters.unit {
            query.push(" AND unit = ");
            query.push_bind(unit);
        }

        if let Some(metric_type) = filters.metric_type {
            query.push(" AND metric_type = ");
            query.push_bind(metric_type);
        }

        let metrics = query
            .build_query_as::<DeviceMetric>()
            .fetch_all(&self.pool)
            .await?;

        Ok(metrics)
    }

    async fn get_aggregated_metrics_for_room(
        &self,
        room_id: i64,
        filters: DeviceMetricFilters,
    ) -> Result<Vec<AggregatedDeviceMetric>> {
        let mut query = sqlx::QueryBuilder::new("");

        if let Some(aggregations) = filters.aggregate {
            for (i, aggregation) in aggregations.iter().enumerate() {
                if i > 0 {
                    query.push(" UNION ALL ");
                }
                let agg_str = match aggregation.aggregate {
                    Aggregation::Avg => "AVG(metric_value)",
                    Aggregation::Sum => "SUM(metric_value)",
                    Aggregation::Min => "MIN(metric_value)",
                    Aggregation::Max => "MAX(metric_value)",
                };
                query.push("SELECT metric_type, unit, ");
                query.push(agg_str);
                query.push(" as metric_value FROM device_metrics WHERE device_id IN (SELECT id FROM devices WHERE room_id = ");
                query.push_bind(room_id);
                query.push(") AND metric_type = ");
                query.push_bind(aggregation.metric_type.clone());

                if let Some(from) = filters.from {
                    query.push(" AND measured_at >= ");
                    query.push_bind(from);
                }

                if let Some(to) = filters.to {
                    query.push(" AND measured_at <= ");
                    query.push_bind(to);
                }

                if let Some(unit) = &filters.unit {
                    query.push(" AND unit = ");
                    query.push_bind(unit.clone());
                }

                query.push(" GROUP BY metric_type, unit");
            }
        }

        let metrics = query
            .build_query_as::<AggregatedDeviceMetric>()
            .fetch_all(&self.pool)
            .await?;

        Ok(metrics)
    }

    async fn get_aggregated_metrics_for_house(
        &self,
        house_id: i64,
        filters: DeviceMetricFilters,
    ) -> Result<Vec<AggregatedDeviceMetric>> {
        let mut query = sqlx::QueryBuilder::new("");

        if let Some(aggregations) = filters.aggregate {
            for (i, aggregation) in aggregations.iter().enumerate() {
                if i > 0 {
                    query.push(" UNION ALL ");
                }
                let agg_str = match aggregation.aggregate {
                    Aggregation::Avg => "AVG(metric_value)",
                    Aggregation::Sum => "SUM(metric_value)",
                    Aggregation::Min => "MIN(metric_value)",
                    Aggregation::Max => "MAX(metric_value)",
                };
                query.push("SELECT metric_type, unit, ");
                query.push(agg_str);
                query.push(" as metric_value FROM device_metrics WHERE device_id IN (SELECT d.id FROM devices d JOIN rooms r ON d.room_id = r.id WHERE r.house_id = ");
                query.push_bind(house_id);
                query.push(") AND metric_type = ");
                query.push_bind(aggregation.metric_type.clone());

                if let Some(from) = filters.from {
                    query.push(" AND measured_at >= ");
                    query.push_bind(from);
                }

                if let Some(to) = filters.to {
                    query.push(" AND measured_at <= ");
                    query.push_bind(to);
                }

                if let Some(unit) = &filters.unit {
                    query.push(" AND unit = ");
                    query.push_bind(unit.clone());
                }

                query.push(" GROUP BY metric_type, unit");
            }
        }

        let metrics = query
            .build_query_as::<AggregatedDeviceMetric>()
            .fetch_all(&self.pool)
            .await?;

        Ok(metrics)
    }
}
