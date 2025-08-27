use async_trait::async_trait;
use mockall::automock;
use sqlx::PgPool;

use crate::{
    errors::Result,
    models::device_metrics::{CreateDeviceMetric, DeviceMetric, DeviceMetricFilters},
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

        if let Some(aggregate) = filters.aggregate {
            let mut aggregated_query =
                sqlx::QueryBuilder::new("SELECT device_id, metric_type, unit, ");

            match aggregate {
                crate::models::device_metrics::Aggregation::Avg => {
                    aggregated_query.push("AVG(metric_value) as metric_value");
                }
                crate::models::device_metrics::Aggregation::Sum => {
                    aggregated_query.push("SUM(metric_value) as metric_value");
                }
                crate::models::device_metrics::Aggregation::Min => {
                    aggregated_query.push("MIN(metric_value) as metric_value");
                }
                crate::models::device_metrics::Aggregation::Max => {
                    aggregated_query.push("MAX(metric_value) as metric_value");
                }
            }

            aggregated_query.push(" FROM (");
            aggregated_query.push(query.sql());
            aggregated_query.push(") as sub");
            aggregated_query.push(" GROUP BY device_id, metric_type, unit");

            let metrics = aggregated_query
                .build_query_as::<DeviceMetric>()
                .fetch_all(&self.pool)
                .await?;
            return Ok(metrics);
        }

        let metrics = query
            .build_query_as::<DeviceMetric>()
            .fetch_all(&self.pool)
            .await?;

        Ok(metrics)
    }
}
