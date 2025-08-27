use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow, ToSchema)]
pub struct DeviceMetric {
    pub id: i64,
    pub device_id: i64,
    pub metric_type: String,
    pub metric_value: f64,
    pub unit: String,
    pub measured_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateDeviceMetric {
    #[serde(default)]
    pub device_id: i64,
    #[validate(length(min = 1, message = "Metric type cannot be empty"))]
    #[serde(default)]
    pub metric_type: String,
    #[serde(default)]
    pub metric_value: f64,
    #[validate(length(min = 1, message = "Unit cannot be empty"))]
    #[serde(default)]
    pub unit: String,
    #[serde(default)]
    pub measured_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum Aggregation {
    Avg,
    Sum,
    Min,
    Max,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct DeviceMetricFilters {
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
    pub unit: Option<String>,
    pub metric_type: Option<String>,
    pub aggregate: Option<Aggregation>,
}
