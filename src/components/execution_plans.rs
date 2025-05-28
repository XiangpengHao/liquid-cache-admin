use leptos::prelude::*;
use std::sync::Arc;

use crate::components::flamegraph::Flamegraph;
use crate::components::statistics::StatisticsComponent;
use crate::models::execution_plan::{ExecutionPlanWithStats, ExecutionStatsWithPlan};
use crate::utils::{format_bytes, format_duration, format_number, format_timestamp};

type RefreshCallback = Box<dyn Fn() + 'static>;
#[component]
fn ExecutionPlanNodeComponent(node: ExecutionPlanWithStats) -> impl IntoView {
    let (expand_schema, set_expanded) = signal(true);

    let has_children = !node.children.is_empty();

    // Display all metrics from the backend
    let mut all_metrics: Vec<(String, String)> = node
        .metrics
        .iter()
        .map(|metric| {
            let key = &metric.name;
            let value = &metric.value;
            let formatted_value = if key.contains("time") || key.contains("elapsed") {
                format_duration(value)
            } else if key.contains("bytes") {
                format_bytes(value.parse::<u64>().unwrap_or(0))
            } else if key.contains("rows") {
                format_number(value)
            } else {
                value.clone()
            };
            (key.clone(), formatted_value)
        })
        .collect();
    all_metrics.sort_by(|a, b| a.0.cmp(&b.0));

    let stats = node.statistics.clone();

    view! {
        <div class="flex flex-col items-center">
            // Node Card
            <div class="relative bg-white border-2 border-gray-200 rounded-lg p-4 shadow-sm hover:shadow-md transition-shadow min-w-64 max-w-80">
                // Node Header
                <div class="flex items-center justify-between mb-3">
                    <div class="flex items-center gap-2">
                        <h4 class="font-semibold text-gray-800 text-sm">{node.name.clone()}</h4>
                    </div>
                </div>

                <div class="grid grid-cols-4 gap-2 mb-3">
                    {all_metrics
                        .into_iter()
                        .map(|(label, value)| {
                            view! {
                                <div class="bg-gray-50 rounded p-2">
                                    <div class="text-xs text-gray-500">{label}</div>
                                    <div
                                        class="text-xs font-mono text-gray-800 truncate"
                                        title=value.clone()
                                    >
                                        {value.clone()}
                                    </div>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>

                <StatisticsComponent stats=stats />

                <div>
                    <button
                        class="flex items-center gap-1 text-xs text-gray-600"
                        on:click=move |_| set_expanded.update(|e| *e = !*e)
                    >
                        <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                            <path
                                fill-rule="evenodd"
                                d=move || {
                                    if expand_schema.get() {
                                        "M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
                                    } else {
                                        "M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
                                    }
                                }
                                clip-rule="evenodd"
                            />
                        </svg>
                        "Schema"
                    </button>
                    <Show when=move || expand_schema.get()>
                        <div class="mt-2 grid grid-cols-3 gap-1">
                            {node
                                .schema
                                .clone()
                                .into_iter()
                                .map(|field| {
                                    view! {
                                        <div class="text-xs bg-white border border-gray-100 rounded p-1">
                                            <div class="text-gray-700 truncate font-medium">
                                                {field.name}
                                            </div>
                                            <div class="text-gray-500 font-mono text-xs truncate">
                                                {field.data_type}
                                            </div>
                                        </div>
                                    }
                                })
                                .collect_view()}
                        </div>
                    </Show>
                </div>
            </div>

            // Connection line and children
            {if has_children {
                view! {
                    <div class="flex flex-col items-center">
                        // Vertical line down
                        <div class="w-0.5 h-8 bg-gray-300"></div>

                        // Children container
                        <div class="flex flex-col gap-8">
                            {node
                                .children
                                .into_iter()
                                .map(|child| {
                                    view! {
                                        <div class="flex flex-col items-center">
                                            // Horizontal line to child
                                            <div class="flex items-center">
                                                <div class="w-8 h-0.5 bg-gray-300"></div>
                                                <div class="w-2 h-2 bg-gray-300 rounded-full"></div>
                                                <div class="w-8 h-0.5 bg-gray-300"></div>
                                            </div>
                                            // Child node
                                            <div class="mt-2">
                                                <ExecutionPlanNodeComponent node=child />
                                            </div>
                                        </div>
                                    }
                                })
                                .collect_view()}
                        </div>
                    </div>
                }
                    .into_any()
            } else {
                ().into_any()
            }}
        </div>
    }
}

#[component]
fn OneExecutionStat(stats: ExecutionStatsWithPlan) -> impl IntoView {
    let plans = stats.plans.clone();
    let execution_stats = stats.execution_stats.clone();
    let (selected_plan_index, set_selected_plan_index) = signal(0);

    view! {
        <div class="border border-gray-200 rounded-lg bg-white">
            <div class="p-4 border-b border-gray-100">
                <h3 class="text-sm font-medium text-gray-800 mb-2">
                    {execution_stats.display_name}
                </h3>
                <div class="grid grid-cols-4 gap-4 text-xs">
                    <div class="bg-gray-50 p-2 rounded">
                        <div class="text-gray-500">"Execution Time"</div>
                        <div class="font-mono text-gray-800">
                            {format!("{}ms", execution_stats.execution_time_ms)}
                        </div>
                    </div>
                    <div class="bg-gray-50 p-2 rounded">
                        <div class="text-gray-500">"Network Traffic"</div>
                        <div class="font-mono text-gray-800">
                            {format_bytes(execution_stats.network_traffic_bytes)}
                        </div>
                    </div>
                    <div class="bg-gray-50 p-2 rounded">
                        <div class="text-gray-500">"Plan Count"</div>
                        <div class="font-mono text-gray-800">{plans.len()}</div>
                    </div>
                    <div class="bg-gray-50 p-2 rounded">
                        <div class="text-gray-500">"Created at"</div>
                        <div class="font-mono text-gray-800">
                            {format_timestamp(plans.first().unwrap().created_at)}
                        </div>
                    </div>
                </div>

                <div class="mt-4">
                    <div class="bg-gray-50 rounded p-3 border max-h-48 overflow-y-auto">
                        <pre class="text-xs font-mono text-gray-800 whitespace-pre-wrap overflow-x-auto">
                            {execution_stats.user_sql.clone()}
                        </pre>
                    </div>
                </div>
            </div>

            // Plan tabs
            {if plans.len() > 1 {
                view! {
                    <div class="border-b border-gray-100">
                        <div class="flex">
                            {plans
                                .iter()
                                .enumerate()
                                .map(|(index, plan)| {
                                    let is_selected = move || selected_plan_index.get() == index;
                                    view! {
                                        <button
                                            class=move || {
                                                format!(
                                                    "px-4 py-2 text-xs font-medium transition-colors border-b-2 {}",
                                                    if is_selected() {
                                                        "text-blue-600 border-blue-600 bg-blue-50"
                                                    } else {
                                                        "text-gray-500 border-transparent hover:text-gray-700 hover:border-gray-300"
                                                    },
                                                )
                                            }
                                            on:click=move |_| set_selected_plan_index.set(index)
                                        >
                                            {format!("Plan {} (ID: {})", index + 1, plan.id)}
                                        </button>
                                    }
                                })
                                .collect_view()}
                        </div>
                    </div>
                }
                    .into_any()
            } else {
                view! { <div></div> }.into_any()
            }}

            // Selected plan content
            <div class="p-4">
                {move || {
                    let selected_index = selected_plan_index.get();
                    if let Some(plan_info) = plans.get(selected_index) {
                        view! {
                            <div class="space-y-6">
                                <div>
                                    <h4 class="text-sm font-medium text-gray-700 mb-3">
                                        "Execution Plan"
                                    </h4>
                                    <div class="flex justify-center">
                                        <ExecutionPlanNodeComponent node=plan_info.plan.clone() />
                                    </div>
                                </div>

                                {if let Some(flamegraph_svg) = execution_stats
                                    .flamegraph_svg
                                    .clone()
                                {
                                    view! {
                                        <Flamegraph
                                            svg_content=flamegraph_svg
                                            plan_id=plan_info.id.clone()
                                        />
                                    }
                                        .into_any()
                                } else {
                                    ().into_any()
                                }}
                            </div>
                        }
                            .into_any()
                    } else {
                        view! {
                            <div class="text-center text-gray-500 py-8">"No plan selected"</div>
                        }
                            .into_any()
                    }
                }}
            </div>
        </div>
    }
}

#[component]
pub fn ExecutionStats(
    execution_stats: Arc<Vec<ExecutionStatsWithPlan>>,
    on_refresh: RefreshCallback,
) -> impl IntoView {
    let (selected_plan_id, set_selected_plan_id) = signal(
        execution_stats
            .first()
            .map(|plan| plan.execution_stats.display_name.clone())
            .unwrap_or_default(),
    );
    let (selected_plan, set_selected_plan) = signal(execution_stats.first().cloned());
    let display_names = execution_stats
        .iter()
        .map(|plan| plan.execution_stats.display_name.clone())
        .collect::<Vec<_>>();

    let execution_stats_clone = execution_stats.clone();

    Effect::new(move |_| {
        if !execution_stats_clone.is_empty() && selected_plan_id.get().is_empty() {
            if let Some(first_plan) = execution_stats_clone.first() {
                set_selected_plan_id.set(first_plan.execution_stats.display_name.clone());
                set_selected_plan.set(Some(first_plan.clone()));
            }
        }
    });

    view! {
        <div class="space-y-4">
            <div class="bg-white border border-gray-200 rounded-lg p-4">
                <div class="flex justify-between items-center mb-4">
                    <h2 class="text-lg font-semibold text-gray-800">"Execution Plans"</h2>
                    <div class="flex items-center space-x-3">
                        <select
                            class="px-3 py-2 border border-gray-200 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-sm text-gray-700 bg-white"
                            on:change=move |ev| {
                                let display_name = event_target_value(&ev);
                                if let Some(plan) = execution_stats
                                    .iter()
                                    .find(|plan| plan.execution_stats.display_name == display_name)
                                {
                                    set_selected_plan.set(Some(plan.clone()));
                                    set_selected_plan_id.set(display_name);
                                }
                            }
                            prop:value=move || selected_plan_id.get()
                        >
                            {move || {
                                display_names
                                    .iter()
                                    .map(|display_name| {
                                        view! {
                                            <option value=display_name
                                                .clone()>{display_name.clone()}</option>
                                        }
                                    })
                                    .collect_view()
                            }}
                        </select>
                        <button
                            class="px-3 py-2 bg-gray-100 border border-gray-200 rounded-md text-gray-700 hover:bg-gray-200 transition-colors text-sm flex items-center gap-2"
                            on:click=move |_| {
                                on_refresh();
                            }
                        >
                            <svg
                                class="w-4 h-4"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
                                ></path>
                            </svg>
                            "Refresh"
                        </button>
                    </div>
                </div>
                {move || {
                    if let Some(selected_plan) = selected_plan.get() {
                        view! { <OneExecutionStat stats=selected_plan /> }.into_any()
                    } else {
                        ().into_any()
                    }
                }}
            </div>
        </div>
    }
}
