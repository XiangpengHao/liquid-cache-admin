use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

use crate::models::execution_plan::{ExecutionPlan, ExecutionPlanNode};
use crate::utils::format_bytes;

type RefreshCallback = Box<dyn Fn() + 'static>;

fn format_duration(duration_str: &str) -> String {
    if duration_str.ends_with("ms") {
        duration_str.to_string()
    } else if duration_str.ends_with("ns") {
        if let Ok(ns) = duration_str.trim_end_matches("ns").parse::<f64>() {
            if ns >= 1_000_000_000.0 {
                format!("{:.2}s", ns / 1_000_000_000.0)
            } else if ns >= 1_000_000.0 {
                format!("{:.2}ms", ns / 1_000_000.0)
            } else if ns >= 1_000.0 {
                format!("{:.2}μs", ns / 1_000.0)
            } else {
                format!("{}ns", ns as u64)
            }
        } else {
            duration_str.to_string()
        }
    } else {
        duration_str.to_string()
    }
}

fn format_number(num_str: &str) -> String {
    if let Ok(num) = num_str.parse::<u64>() {
        if num >= 1_000_000_000 {
            format!("{:.2}B", num as f64 / 1_000_000_000.0)
        } else if num >= 1_000_000 {
            format!("{:.2}M", num as f64 / 1_000_000.0)
        } else if num >= 1_000 {
            format!("{:.2}K", num as f64 / 1_000.0)
        } else {
            num.to_string()
        }
    } else {
        num_str.to_string()
    }
}

#[component]
fn Flamegraph(svg_content: String, plan_id: String) -> impl IntoView {
    let svg_for_download = svg_content.clone();
    let plan_id_for_download = plan_id.clone();

    let download_svg = move |_| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Ok(element) = document.create_element("a") {
                    let anchor = element.unchecked_into::<web_sys::HtmlAnchorElement>();

                    let data_url = format!(
                        "data:image/svg+xml;charset=utf-8,{}",
                        urlencoding::encode(&svg_for_download)
                    );

                    anchor.set_href(&data_url);
                    anchor.set_download(&format!("flamegraph-{plan_id_for_download}.svg"));

                    if let Some(body) = document.body() {
                        let _ = body.append_child(&anchor);
                        anchor.click();
                        let _ = body.remove_child(&anchor);
                    }
                }
            }
        }
    };

    view! {
        <div class="flex justify-between items-center mb-3">
            <h3 class="text-sm font-medium text-gray-700">"Flamegraph"</h3>
            <div class="flex gap-2">
                <button
                    class="px-3 py-1 border border-gray-200 rounded text-gray-600 hover:bg-gray-50 transition-colors text-xs flex items-center gap-1"
                    on:click=download_svg
                >
                    <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                        ></path>
                    </svg>
                    "Download SVG"
                </button>
            </div>
        </div>
        <div class="bg-white rounded overflow-auto">
            <iframe
                srcdoc=format!(
                    "<!DOCTYPE html><html><head><style>body{{margin:0;padding:0;}} svg{{width:100%;height:auto;}}</style></head><body>{}</body></html>",
                    svg_content,
                )
                class="w-full h-[600px] border-0"
                sandbox="allow-scripts allow-same-origin"
            ></iframe>
        </div>
    }
}

#[component]
fn ExecutionPlanNodeComponent(node: ExecutionPlanNode) -> impl IntoView {
    let (expanded, set_expanded) = signal(true);

    let has_children = !node.children.is_empty();

    // Display all metrics from the backend
    let mut all_metrics: Vec<(String, String)> = node
        .metrics
        .iter()
        .map(|(key, value)| {
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

                // All Metrics Grid
                {if !all_metrics.is_empty() {
                    view! {
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
                    }
                        .into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}

                // Statistics
                {if let Some(statistics) = &node.statistics {
                    let stats = statistics.clone();
                    view! {
                        <div class="text-xs text-gray-500 bg-gray-50 p-2 rounded mb-3">
                            <div class="font-medium mb-1">"Statistics"</div>
                            <div class="flex gap-4 mb-2">
                                {if let Some(num_rows) = &stats.num_rows {
                                    view! {
                                        <span>
                                            <span class="text-gray-500">"Rows: "</span>
                                            <span class="text-gray-800 font-mono">
                                                {num_rows.clone()}
                                            </span>
                                        </span>
                                    }
                                        .into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }}
                                {if let Some(total_byte_size) = &stats.total_byte_size {
                                    view! {
                                        <span>
                                            <span class="text-gray-500">"Size: "</span>
                                            <span class="text-gray-800 font-mono">
                                                {total_byte_size.clone()}
                                            </span>
                                        </span>
                                    }
                                        .into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }}
                                {if !stats.columns.is_empty() {
                                    let columns = stats.columns.clone();
                                    let total_columns = columns.len();
                                    view! {
                                        <div class="mt-2">
                                            <div class="font-medium mb-1">"Column Statistics:"</div>
                                            <div class="space-y-1 max-h-32 overflow-y-auto">
                                                {columns
                                                    .into_iter()
                                                    .take(5)
                                                    .map(|col| {
                                                        view! {
                                                            <div class="text-xs bg-white border border-gray-100 rounded p-1">
                                                                <div class="font-medium text-gray-700">{col.name}</div>
                                                                <div class="grid grid-cols-4 gap-1 text-xs">
                                                                    {if let Some(min) = &col.min {
                                                                        view! {
                                                                            <div class="truncate">
                                                                                <span class="text-gray-500">"Min: "</span>
                                                                                <span class="text-gray-800">{min.clone()}</span>
                                                                            </div>
                                                                        }
                                                                            .into_any()
                                                                    } else {
                                                                        view! { <div></div> }.into_any()
                                                                    }}
                                                                    {if let Some(max) = &col.max {
                                                                        view! {
                                                                            <div class="truncate">
                                                                                <span class="text-gray-500">"Max: "</span>
                                                                                <span class="text-gray-800">{max.clone()}</span>
                                                                            </div>
                                                                        }
                                                                            .into_any()
                                                                    } else {
                                                                        view! { <div></div> }.into_any()
                                                                    }}
                                                                    {if let Some(sum) = &col.sum {
                                                                        view! {
                                                                            <div class="truncate">
                                                                                <span class="text-gray-500">"Sum: "</span>
                                                                                <span class="text-gray-800">{sum.clone()}</span>
                                                                            </div>
                                                                        }
                                                                            .into_any()
                                                                    } else {
                                                                        view! { <div></div> }.into_any()
                                                                    }}
                                                                    {if let Some(null) = &col.null {
                                                                        view! {
                                                                            <div class="truncate">
                                                                                <span class="text-gray-500">"Null: "</span>
                                                                                <span class="text-gray-800">{null.clone()}</span>
                                                                            </div>
                                                                        }
                                                                            .into_any()
                                                                    } else {
                                                                        view! { <div></div> }.into_any()
                                                                    }}
                                                                    {if let Some(distinct) = &col.distinct {
                                                                        view! {
                                                                            <div class="truncate">
                                                                                <span class="text-gray-500">"Distinct: "</span>
                                                                                <span class="text-gray-800">{distinct.clone()}</span>
                                                                            </div>
                                                                        }
                                                                            .into_any()
                                                                    } else {
                                                                        view! { <div></div> }.into_any()
                                                                    }}
                                                                </div>
                                                            </div>
                                                        }
                                                    })
                                                    .collect_view()}
                                                {if total_columns > 5 {
                                                    view! {
                                                        <div class="text-xs text-gray-400 italic">
                                                            {format!("... and {} more columns", total_columns - 5)}
                                                        </div>
                                                    }
                                                        .into_any()
                                                } else {
                                                    view! { <div></div> }.into_any()
                                                }}
                                            </div>
                                        </div>
                                    }
                                        .into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }}
                            </div>
                        </div>
                    }
                        .into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}

                // Schema toggle
                {if !node.schema.is_empty() {
                    view! {
                        <div class="border-t border-gray-100 pt-3">
                            <button
                                class="flex items-center gap-1 text-xs text-gray-600 hover:text-gray-800 transition-colors"
                                on:click=move |_| set_expanded.update(|e| *e = !*e)
                            >
                                <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                                    <path
                                        fill-rule="evenodd"
                                        d=move || {
                                            if expanded.get() {
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

                            {move || {
                                if expanded.get() {
                                    view! {
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
                                    }
                                        .into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }
                            }}
                        </div>
                    }
                        .into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
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
pub fn ExecutionPlans(
    execution_plans: ReadSignal<Option<Vec<ExecutionPlan>>>,
    on_refresh: RefreshCallback,
) -> impl IntoView {
    let (selected_plan_id, set_selected_plan_id) = signal(String::new());

    // Auto-select the first plan when execution plans are loaded
    Effect::new(move || {
        if let Some(plans) = execution_plans.get() {
            if !plans.is_empty() {
                let current_selected = selected_plan_id.get();
                // If no plan is selected, or if selected plan no longer exists, select the first one
                if current_selected.is_empty()
                    || !plans.iter().any(|plan| plan.id == current_selected)
                {
                    set_selected_plan_id.set(plans[0].id.clone());
                }
            }
        }
    });

    view! {
        <div class="border border-gray-200 rounded-lg bg-white p-4">
            <div class="flex justify-between items-center mb-3">
                <h2 class="text-base font-medium text-gray-700">"Execution Plans"</h2>
                <div class="flex items-center space-x-2">
                    <select
                        class="px-2 py-1 border border-gray-200 rounded focus:outline-none focus:border-gray-400 text-xs text-gray-700"
                        on:change=move |ev| {
                            set_selected_plan_id.set(event_target_value(&ev));
                        }
                        prop:value=selected_plan_id
                    >
                        <option value="">"Select a plan..."</option>
                        {move || {
                            execution_plans
                                .get()
                                .map(|plans| {
                                    plans
                                        .into_iter()
                                        .map(|plan| {
                                            let display_name = plan.display_name();
                                            view! {
                                                <option value=plan.id.clone()>{display_name}</option>
                                            }
                                        })
                                        .collect_view()
                                })
                                .unwrap_or_default()
                        }}
                    </select>
                    <button
                        class="px-2 py-1 border border-gray-200 rounded text-gray-600 hover:bg-gray-50 transition-colors text-xs"
                        on:click=move |_| {
                            on_refresh();
                            set_selected_plan_id.set(String::new());
                        }
                    >
                        "Refresh"
                    </button>
                </div>
            </div>

            <div class="space-y-3">
                {move || {
                    if let (Some(plans), selected_id) = (
                        execution_plans.get(),
                        selected_plan_id.get(),
                    ) {
                        if !selected_id.is_empty() {
                            if let Some(selected_plan) = plans
                                .iter()
                                .find(|plan| plan.id == selected_id)
                            {
                                view! {
                                    <div class="space-y-2">
                                        <div class="flex items-center justify-between p-2 rounded">
                                            <div class="flex items-center gap-4 text-xs">
                                                <span class="text-gray-500">
                                                    "ID: "
                                                    <span class="text-gray-800 font-mono">
                                                        {selected_plan.id.clone()}
                                                    </span>
                                                </span>
                                                <span class="text-gray-500">
                                                    "Created: "
                                                    <span class="text-gray-800">
                                                        {selected_plan.formatted_time.clone()}
                                                    </span>
                                                </span>
                                                {if let Some(stats) = &selected_plan.stats {
                                                    view! {
                                                        <>
                                                            {if !stats.display_name.is_empty() {
                                                                view! {
                                                                    <span class="text-gray-500">
                                                                        "Name: "
                                                                        <span class="text-gray-800">
                                                                            {stats.display_name.clone()}
                                                                        </span>
                                                                    </span>
                                                                }
                                                                    .into_any()
                                                            } else {
                                                                ().into_any()
                                                            }}
                                                            <span class="text-gray-500">
                                                                "Query execution time: "
                                                                <span class="text-gray-800">
                                                                    {format!("{}ms", stats.execution_time_ms)}
                                                                </span>
                                                            </span>
                                                            <span class="text-gray-500">
                                                                "Network: "
                                                                <span class="text-gray-800">
                                                                    {format_bytes(stats.network_traffic_bytes)}
                                                                </span>
                                                            </span>
                                                        </>
                                                    }
                                                        .into_any()
                                                } else {
                                                    ().into_any()
                                                }}
                                            </div>
                                        </div>
                                        <div class="overflow-x-auto mb-8">
                                            <div class="flex justify-center">
                                                <ExecutionPlanNodeComponent node=selected_plan
                                                    .plan
                                                    .clone() />
                                            </div>
                                        </div>

                                        // Flamegraph SVG display - placed after execution plan
                                        {if let Some(svg) = selected_plan.flamegraph_svg() {
                                            view! {
                                                <Flamegraph
                                                    svg_content=svg.clone()
                                                    plan_id=selected_plan.id.clone()
                                                />
                                            }
                                                .into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }}
                                    </div>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <div class="text-gray-400 text-sm italic mt-2">
                                        "Selected plan not found"
                                    </div>
                                }
                                    .into_any()
                            }
                        } else {
                            view! {
                                <div class="text-gray-400 text-sm italic mt-2">
                                    "Select a plan to view its details"
                                </div>
                            }
                                .into_any()
                        }
                    } else {
                        ().into_any()
                    }
                }}
            </div>
        </div>
    }
}
