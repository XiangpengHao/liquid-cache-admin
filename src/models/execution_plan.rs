use chrono::{Local, TimeZone, Utc};
use leptos::logging;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct ExecutionPlanNode {
    pub name: String,
    pub schema: Vec<SchemaField>,
    pub statistics: Option<Statistics>,
    pub metrics: HashMap<String, String>,
    #[serde(default, deserialize_with = "deserialize_children")]
    pub children: Vec<ExecutionPlanNode>,
}

fn deserialize_children<'de, D>(deserializer: D) -> Result<Vec<ExecutionPlanNode>, D::Error>
where
    D: Deserializer<'de>,
{
    let children_strings: Vec<String> = Vec::deserialize(deserializer)?;
    let mut children = Vec::new();

    for child_str in children_strings {
        match serde_json::from_str::<ExecutionPlanNode>(&child_str) {
            Ok(child) => children.push(child),
            Err(e) => {
                logging::error!("Failed to parse child execution plan: {}", e);
            }
        }
    }

    Ok(children)
}

#[derive(Debug, Clone, Deserialize)]
pub struct SchemaField {
    pub name: String,
    pub data_type: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Statistics {
    pub num_rows: Option<String>,
    pub total_byte_size: Option<String>,
    pub columns: Vec<ColumnStatistic>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ColumnStatistic {
    pub name: String,
    pub null: Option<String>,
    pub max: Option<String>,
    pub min: Option<String>,
    pub sum: Option<String>,
    pub distinct: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExecutionStats {
    #[allow(unused)]
    pub plan_id: String,
    pub display_name: String,
    pub flamegraph_svg: Option<String>,
    pub network_traffic_bytes: u64,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone)]
pub struct ExecutionPlan {
    pub id: String,
    pub plan: ExecutionPlanNode,
    pub created_at: u64,
    pub formatted_time: String,
    pub stats: Option<ExecutionStats>,
}

impl ExecutionPlan {
    pub fn display_name(&self) -> String {
        if let Some(stats) = &self.stats {
            if !stats.display_name.is_empty() {
                format!("{} ({})", stats.display_name, self.formatted_time)
            } else {
                self.fallback_display_name()
            }
        } else {
            self.fallback_display_name()
        }
    }

    fn fallback_display_name(&self) -> String {
        let short_id = if self.id.len() > 8 {
            format!("{}...", &self.id[0..8])
        } else {
            self.id.clone()
        };
        format!("{} ({})", short_id, self.formatted_time)
    }

    pub fn flamegraph_svg(&self) -> Option<&String> {
        self.stats.as_ref().and_then(|s| s.flamegraph_svg.as_ref())
    }
}

// Backend response structures
#[derive(Deserialize, Clone)]
struct ExecutionPlanResponse {
    plan: String,
    #[allow(unused)]
    id: String,
    created_at: u64,
    stats: Option<String>,
}

pub fn parse_execution_plans(
    response: Vec<(String, String)>,
) -> Result<Vec<ExecutionPlan>, String> {
    let mut plans = Vec::new();

    for (key, value) in response {
        match serde_json::from_str::<ExecutionPlanResponse>(&value) {
            Ok(plan_response) => {
                match serde_json::from_str::<ExecutionPlanNode>(&plan_response.plan) {
                    Ok(plan_node) => {
                        let datetime = Utc
                            .timestamp_opt(plan_response.created_at as i64, 0)
                            .single()
                            .ok_or_else(|| format!("Invalid timestamp for plan {key}"))?;
                        let local_datetime = datetime.with_timezone(&Local);
                        let formatted_time = local_datetime.format("%H:%M:%S").to_string();

                        // Parse execution stats if available
                        let execution_stats = if let Some(stats_json) = &plan_response.stats {
                            match serde_json::from_str::<ExecutionStats>(stats_json) {
                                Ok(stats) => Some(stats),
                                Err(e) => {
                                    logging::error!(
                                        "Failed to parse execution stats for key {}: {}",
                                        key,
                                        e
                                    );
                                    None
                                }
                            }
                        } else {
                            None
                        };

                        plans.push(ExecutionPlan {
                            id: key,
                            plan: plan_node,
                            created_at: plan_response.created_at,
                            formatted_time,
                            stats: execution_stats,
                        });
                    }
                    Err(e) => {
                        logging::error!("Failed to parse execution plan for key {key}: {e}");
                        logging::error!("Raw plan JSON: {}", plan_response.plan);
                        return Err(format!("Failed to parse execution plan for key {key}: {e}"));
                    }
                }
            }
            Err(e) => {
                logging::error!("Failed to parse execution plan response for key {key}: {e}");
                logging::error!("Raw JSON value: {}", value);
                return Err(format!(
                    "Failed to parse execution plan response for key {key}: {e}"
                ));
            }
        }
    }

    // Sort by creation time (newest first)
    plans.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    Ok(plans)
}
