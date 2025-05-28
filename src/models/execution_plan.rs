use serde::Deserialize;

/// Parameters for the set_execution_stats endpoint
#[derive(Deserialize, Clone, Debug)]
pub struct ExecutionStats {
    /// Plan ID for the execution plan
    #[allow(dead_code)]
    pub plan_ids: Vec<String>,
    /// Display name for the execution plan
    pub display_name: String,
    /// Flamegraph SVG for the execution plan
    pub flamegraph_svg: Option<String>,
    /// Network traffic bytes for the execution plan
    pub network_traffic_bytes: u64,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Execution stats with plan
#[derive(Deserialize, Clone)]
pub struct ExecutionStatsWithPlan {
    /// Execution stats
    pub execution_stats: ExecutionStats,
    /// Plan info
    pub plans: Vec<PlanInfo>,
}

/// Schema field
#[derive(Deserialize, Clone, Debug)]
pub struct SchemaField {
    /// Field name
    pub name: String,
    /// Field data type
    pub data_type: String,
}

/// Column statistics
#[derive(Deserialize, Clone, Debug)]
pub struct ColumnStatistics {
    /// Column name
    pub name: String,
    /// Null count
    pub null: Option<String>,
    /// Max value
    pub max: Option<String>,
    /// Min value
    pub min: Option<String>,
    /// Sum value
    pub sum: Option<String>,
    /// Distinct count
    pub distinct_count: Option<String>,
}

/// Statistics
#[derive(Deserialize, Clone, Debug)]
pub struct Statistics {
    /// Number of rows
    pub num_rows: String,
    /// Total byte size
    pub total_byte_size: String,
    /// Column statistics
    pub column_statistics: Vec<ColumnStatistics>,
}

/// Metric
#[derive(Deserialize, Clone)]
pub struct MetricValues {
    /// Metric name
    pub name: String,
    /// Metric value
    pub value: String,
}

/// Execution plan with stats
#[derive(Deserialize, Clone)]
pub struct ExecutionPlanWithStats {
    /// Execution plan name
    pub name: String,
    /// Schema fields
    pub schema: Vec<SchemaField>,
    /// Statistics
    pub statistics: Statistics,
    /// Metrics
    pub metrics: Vec<MetricValues>,
    /// Children
    pub children: Vec<ExecutionPlanWithStats>,
}

/// Plan info
#[derive(Deserialize, Clone)]
pub struct PlanInfo {
    /// Created at
    pub created_at: u64,
    /// Execution plan
    pub plan: ExecutionPlanWithStats,
    /// ID
    pub id: String,
}
