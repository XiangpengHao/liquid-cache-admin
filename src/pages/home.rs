use crate::components::cache_info::{
    CacheInfo as CacheInfoComponent, CacheInfo as CacheInfoData, ParquetCacheUsage,
};
use crate::components::execution_plans::{
    ExecutionPlanNode, ExecutionPlans as ExecutionPlansComponent,
};
use crate::components::system_info::{
    SystemInfo as SystemInfoComponent, SystemInfo as SystemInfoData,
};
use crate::components::toast::use_toast;
use crate::utils::{fetch_api, ApiResponse};
use chrono::{Local, TimeZone, Utc};
use leptos::{logging, prelude::*};
use leptos_router::{hooks::use_navigate, hooks::use_query_map};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
struct ExecutionMetricsResponse {
    plan_id: String,
    execution_time_ms: u64,
    cache_hits: u64,
    cache_misses: u64,
}

// New struct to handle the backend response format
#[derive(Deserialize, Clone)]
struct ExecutionPlanResponse {
    plan: String,
    #[allow(dead_code)]
    id: String,
    created_at: u64,
    flamegraph_svg: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
struct TraceParams {
    path: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
struct ExecutionMetricsParams {
    plan_id: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
struct CacheStatsParams {
    path: String,
}



/// Default Home Page - LiquidCache Server Monitoring Dashboard
#[component]
pub fn Home() -> impl IntoView {
    let toast = use_toast();

    // Read query parameters
    let query_map = use_query_map();
    let host_param = move || query_map.read().get("host");

    let (server_address, set_server_address) = signal("http://localhost:53703".to_string());
    let (cache_usage, set_cache_usage) = signal(None::<ParquetCacheUsage>);
    let (cache_info, set_cache_info) = signal(None::<CacheInfoData>);
    let (system_info, set_system_info) = signal(None);

    let (trace_active, set_trace_active) = signal(false);
    let (trace_path, set_trace_path) = signal("/tmp".to_string());
    let (metrics_plan_id, set_metrics_plan_id) = signal(String::new());
    let (execution_metrics, set_execution_metrics) = signal(None);
    let (stats_path, set_stats_path) = signal("/tmp".to_string());

    let (execution_plans, set_execution_plans) =
        signal(None::<Vec<(String, ExecutionPlanNode, String, Option<String>)>>);

    let fetch_cache_usage = {
        let toast = toast.clone();
        Action::new(move |_: &()| {
            let address = server_address.get();
            let toast = toast.clone();

            async move {
                match fetch_api::<ParquetCacheUsage>(&format!("{address}/parquet_cache_usage"))
                    .await
                {
                    Ok(response) => {
                        set_cache_usage.set(Some(response));
                    }
                    Err(e) => {
                        toast.show_error(format!("Failed to fetch cache usage: {e}"));
                    }
                }
            }
        })
    };

    let fetch_cache_info = {
        let toast = toast.clone();
        Action::new(move |_: &()| {
            let address = server_address.get();
            let toast = toast.clone();

            async move {
                match fetch_api::<CacheInfoData>(&format!("{address}/cache_info")).await {
                    Ok(response) => {
                        logging::log!("Cache info: {:?}", response);
                        set_cache_info.set(Some(response));
                    }
                    Err(e) => {
                        logging::error!("Failed to fetch cache info: {}", e);
                        toast.show_error(format!("Failed to fetch cache info: {e}"));
                    }
                }
            }
        })
    };

    let fetch_system_info = {
        let toast = toast.clone();
        Action::new(move |_: &()| {
            let address = server_address.get();
            let toast = toast.clone();

            async move {
                match fetch_api::<SystemInfoData>(&format!("{address}/system_info")).await {
                    Ok(response) => {
                        set_system_info.set(Some(response));
                    }
                    Err(e) => {
                        toast.show_error(format!("Failed to fetch system info: {e}"));
                    }
                }
            }
        })
    };

    let fetch_execution_plans = {
        let toast = toast.clone();
        Action::new(move |_: &()| {
            let address = server_address.get();
            let toast = toast.clone();

            async move {
                match fetch_api::<Vec<(String, String)>>(&format!("{address}/execution_plans"))
                    .await
                {
                    Ok(response) => {
                        let mut plans = Vec::new();
                        for (key, value) in response {
                            match serde_json::from_str::<ExecutionPlanResponse>(&value) {
                                Ok(plan_response) => {
                                    match serde_json::from_str::<ExecutionPlanNode>(&plan_response.plan) {
                                        Ok(plan) => {
                                            let datetime = Utc.timestamp_opt(plan_response.created_at as i64, 0).unwrap();
                                            let local_datetime = datetime.with_timezone(&Local);
                                            let formatted_time =
                                                local_datetime.format("%H:%M:%S").to_string();
                                            plans.push((key, plan, formatted_time, plan_response.flamegraph_svg, plan_response.created_at));
                                        }
                                        Err(e) => {
                                            logging::error!(
                                                "Failed to parse execution plan for key {key}: {e}"
                                            );
                                            logging::error!("Raw plan JSON: {}", plan_response.plan);
                                            toast.show_error(format!(
                                                "Failed to parse execution plan for key {key}: {e}"
                                            ));
                                        }
                                    }
                                }
                                Err(e) => {
                                    logging::error!(
                                        "Failed to parse execution plan response for key {key}: {e}"
                                    );
                                    logging::error!("Raw JSON value: {value}");
                                    toast.show_error(format!(
                                        "Failed to parse execution plan response for key {key}: {e}"
                                    ));
                                }
                            }
                        }
                        plans.sort_by(|a, b| b.4.cmp(&a.4));
                        let sorted_plans: Vec<(String, ExecutionPlanNode, String, Option<String>)> = plans
                            .into_iter()
                            .map(|(key, plan, formatted_time, flamegraph_svg, _)| (key, plan, formatted_time, flamegraph_svg))
                            .collect();
                        set_execution_plans.set(Some(sorted_plans));
                    }
                    Err(e) => {
                        toast.show_error(format!("Failed to fetch execution plans: {e}"));
                    }
                }
            }
        })
    };

    // New action for starting trace collection
    let start_trace = {
        let toast = toast.clone();
        Action::new(move |_: &()| {
            let address = server_address.get();
            let toast = toast.clone();

            async move {
                match fetch_api::<ApiResponse>(&format!("{address}/start_trace")).await {
                    Ok(response) => {
                        toast.show_success(response.message);
                        set_trace_active.set(true);
                    }
                    Err(e) => {
                        toast.show_error(format!("Failed to start trace: {e}"));
                    }
                }
            }
        })
    };

    // Action for stopping trace collection
    let stop_trace = {
        let toast = toast.clone();
        Action::new(move |_: &()| {
            let address = server_address.get();
            let path = trace_path.get();
            let toast = toast.clone();

            async move {
                match fetch_api::<ApiResponse>(&format!("{address}/stop_trace?path={path}")).await {
                    Ok(response) => {
                        toast.show_success(response.message);
                        set_trace_active.set(false);
                    }
                    Err(e) => {
                        toast.show_error(format!("Failed to stop trace: {e}"));
                    }
                }
            }
        })
    };

    // Action for getting execution metrics
    let get_execution_metrics = {
        let toast = toast.clone();
        Action::new(move |_: &()| {
            let address = server_address.get();
            let plan_id = metrics_plan_id.get();
            let toast = toast.clone();

            async move {
                if plan_id.is_empty() {
                    toast.show_error("Plan ID cannot be empty".to_string());
                    return;
                }

                match fetch_api::<Option<ExecutionMetricsResponse>>(&format!(
                    "{address}/execution_metrics?plan_id={plan_id}"
                ))
                .await
                {
                    Ok(Some(response)) => {
                        set_execution_metrics.set(Some(response));
                    }
                    Ok(None) => {
                        toast.show_error(format!("No metrics found for plan ID: {plan_id}"));
                        set_execution_metrics.set(None);
                    }
                    Err(e) => {
                        toast.show_error(format!("Failed to fetch execution metrics: {e}"));
                    }
                }
            }
        })
    };

    // Action for getting cache stats
    let get_cache_stats = {
        let toast = toast.clone();
        Action::new(move |_: &()| {
            let address = server_address.get();
            let path = stats_path.get();
            let toast = toast.clone();

            async move {
                match fetch_api::<ApiResponse>(&format!("{address}/cache_stats?path={path}")).await
                {
                    Ok(response) => {
                        toast.show_success(response.message);
                    }
                    Err(e) => {
                        toast.show_error(format!("Failed to get cache stats: {e}"));
                    }
                }
            }
        })
    };



    let navigate = use_navigate();

    let fetch_all_data = move |_| {
        fetch_cache_usage.dispatch(());
        fetch_cache_info.dispatch(());
        fetch_system_info.dispatch(());
        fetch_execution_plans.dispatch(());
    };

    // Initialize server address from URL parameter on mount (runs only once)
    let host = host_param();
    if let Some(host) = host {
        logging::log!("Found host parameter on initial load: {}", host);
        set_server_address.set(host);
        // Automatically fetch data when loading from URL parameter
        fetch_all_data(());
    }

    let connect_and_update_url = move |_| {
        let current_address = server_address.get();
        // Update URL with the current server address (simple encoding)
        let encoded_address = current_address
            .replace("://", "%3A%2F%2F")
            .replace("/", "%2F");
        let query_string = format!("?host={encoded_address}");
        navigate(&query_string, Default::default());
        // Fetch data
        fetch_all_data(());
    };

    view! {
        <div class="min-h-screen bg-gray-50">
            <ErrorBoundary fallback=|errors| {
                view! {
                    <h1 class="text-2xl text-gray-700 mb-4">"Something went wrong"</h1>
                    <ul class="text-sm text-gray-600">
                        {move || {
                            errors
                                .get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                .collect_view()
                        }}
                    </ul>
                }
            }>
                <div class="container mx-auto px-6 py-6 max-w-7xl">
                    <h1 class="text-2xl font-medium text-gray-800 mb-6 border-b border-gray-200 pb-3">
                        "LiquidCache Monitor"
                    </h1>

                    // Connection section
                    <div class="mb-6">
                        <div class="flex items-center space-x-2 mb-4">
                            <input
                                type="text"
                                placeholder="Server address"
                                class="flex-1 px-3 py-2 border border-gray-200 rounded focus:outline-none focus:border-gray-400 text-sm text-gray-700"
                                prop:value=server_address
                                on:input=move |ev| {
                                    set_server_address.set(event_target_value(&ev));
                                }
                            />
                            <button
                                class="px-4 py-2 border border-gray-200 rounded text-gray-700 hover:bg-gray-100 transition-colors text-sm"
                                on:click=connect_and_update_url
                            >
                                "Connect"
                            </button>
                        </div>
                    </div>

                    // Dashboard Grid Layout
                    <div class="space-y-4 mb-6">
                        // Top row - System Info and Cache Info
                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
                            <SystemInfoComponent
                                system_info=system_info
                                on_refresh=Box::new(move || {
                                    let _ = fetch_system_info.dispatch(());
                                })
                            />

                            <CacheInfoComponent
                                cache_info=cache_info
                                cache_usage=cache_usage
                                server_address=server_address
                                on_refresh=Box::new(move || {
                                    fetch_cache_info.dispatch(());
                                    fetch_cache_usage.dispatch(());
                                })
                            />
                        </div>

                        // Execution Plans - Full width
                        <ExecutionPlansComponent
                            execution_plans=execution_plans
                            on_refresh=Box::new(move || {
                                fetch_execution_plans.dispatch(());
                            })
                        />
                    </div>

                    // Tools Grid Layout
                    <div class="space-y-4">
                        <h2 class="text-lg font-medium text-gray-800 border-b border-gray-200 pb-2">
                            "Profiling Tools"
                        </h2>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            // Trace Collection Tool
                            <div class="border border-gray-200 rounded-lg bg-white p-4">
                                <div class="flex justify-between items-center mb-3">
                                    <h3 class="text-base font-medium text-gray-700">
                                        "Trace Collection"
                                    </h3>
                                </div>
                                <div class="text-sm space-y-3">
                                    <div class="space-y-2">
                                        <label class="block text-gray-600 text-xs">
                                            "Save Path"
                                        </label>
                                        <input
                                            type="text"
                                            placeholder="Path to save trace data"
                                            class="w-full px-2 py-2 border border-gray-200 rounded focus:outline-none focus:border-gray-400 text-sm text-gray-700"
                                            prop:value=trace_path
                                            on:input=move |ev| {
                                                set_trace_path.set(event_target_value(&ev));
                                            }
                                            disabled=move || trace_active.get()
                                        />
                                    </div>
                                    <div>
                                        {move || {
                                            if trace_active.get() {
                                                view! {
                                                    <button
                                                        class="w-full px-3 py-2 border border-red-100 rounded text-red-500 hover:bg-red-50 transition-colors text-sm"
                                                        on:click=move |_| {
                                                            stop_trace.dispatch(());
                                                        }
                                                    >
                                                        "Stop Trace"
                                                    </button>
                                                }
                                                    .into_any()
                                            } else {
                                                view! {
                                                    <button
                                                        class="w-full px-3 py-2 border border-gray-200 rounded text-gray-600 hover:bg-gray-50 transition-colors text-sm"
                                                        on:click=move |_| {
                                                            start_trace.dispatch(());
                                                        }
                                                    >
                                                        "Start Trace"
                                                    </button>
                                                }
                                                    .into_any()
                                            }
                                        }}
                                    </div>
                                    <div class="text-xs text-gray-500">
                                        {move || {
                                            if trace_active.get() {
                                                "Trace collection is active. Stop to save trace data."
                                            } else {
                                                "Start trace collection to capture cache operations."
                                            }
                                        }}
                                    </div>
                                </div>
                            </div>

                            // Cache Statistics Tool
                            <div class="border border-gray-200 rounded-lg bg-white p-4">
                                <div class="flex justify-between items-center mb-3">
                                    <h3 class="text-base font-medium text-gray-700">
                                        "Cache Statistics"
                                    </h3>
                                </div>
                                <div class="text-sm space-y-3">
                                    <div class="space-y-2">
                                        <label class="block text-gray-600 text-xs">
                                            "Save Path"
                                        </label>
                                        <input
                                            type="text"
                                            placeholder="Path to save cache statistics"
                                            class="w-full px-2 py-2 border border-gray-200 rounded focus:outline-none focus:border-gray-400 text-sm text-gray-700"
                                            prop:value=stats_path
                                            on:input=move |ev| {
                                                set_stats_path.set(event_target_value(&ev));
                                            }
                                        />
                                    </div>
                                    <div>
                                        <button
                                            class="w-full px-3 py-2 border border-gray-200 rounded text-gray-600 hover:bg-gray-50 transition-colors text-sm"
                                            on:click=move |_| {
                                                get_cache_stats.dispatch(());
                                            }
                                        >
                                            "Export Statistics"
                                        </button>
                                    </div>
                                </div>
                            </div>

                            // Execution Metrics Tool
                            <div class="border border-gray-200 rounded-lg bg-white p-4">
                                <div class="flex justify-between items-center mb-3">
                                    <h3 class="text-base font-medium text-gray-700">
                                        "Execution Metrics"
                                    </h3>
                                </div>
                                <div class="text-sm space-y-3">
                                    <div class="space-y-2">
                                        <label class="block text-gray-600 text-xs">"Plan ID"</label>
                                        <input
                                            type="text"
                                            placeholder="Enter plan UUID"
                                            class="w-full px-2 py-2 border border-gray-200 rounded focus:outline-none focus:border-gray-400 text-sm text-gray-700"
                                            prop:value=metrics_plan_id
                                            on:input=move |ev| {
                                                set_metrics_plan_id.set(event_target_value(&ev));
                                            }
                                        />
                                    </div>
                                    <div>
                                        <button
                                            class="w-full px-3 py-2 border border-gray-200 rounded text-gray-600 hover:bg-gray-50 transition-colors text-sm"
                                            on:click=move |_| {
                                                get_execution_metrics.dispatch(());
                                            }
                                        >
                                            "Get Metrics"
                                        </button>
                                    </div>

                                    {move || match execution_metrics.get() {
                                        Some(metrics) => {
                                            view! {
                                                <div class="bg-gray-50 p-3 rounded border border-gray-100">
                                                    <h4 class="text-xs font-medium text-gray-600 mb-2">
                                                        "Metrics for Plan: " {metrics.plan_id.clone()}
                                                    </h4>
                                                    <div class="grid grid-cols-2 gap-y-1 text-xs">
                                                        <span class="text-gray-500">"Execution Time"</span>
                                                        <span class="text-gray-800">
                                                            {format!("{} ms", metrics.execution_time_ms)}
                                                        </span>

                                                        <span class="text-gray-500">"Cache Hits"</span>
                                                        <span class="text-gray-800">{metrics.cache_hits}</span>

                                                        <span class="text-gray-500">"Cache Misses"</span>
                                                        <span class="text-gray-800">{metrics.cache_misses}</span>
                                                    </div>
                                                </div>
                                            }
                                                .into_any()
                                        }
                                        None => {
                                            view! {
                                                <div class="text-gray-400 text-xs italic">
                                                    "Enter a plan ID and click 'Get Metrics' to view execution metrics"
                                                </div>
                                            }
                                                .into_any()
                                        }
                                    }}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </ErrorBoundary>
        </div>
    }
}
