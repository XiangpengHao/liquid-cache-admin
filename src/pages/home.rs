use leptos::{logging, prelude::*};
use serde::Deserialize;
use crate::components::system_info::{SystemInfo as SystemInfoComponent, SystemInfo as SystemInfoData};
use crate::components::cache_info::{CacheInfo as CacheInfoComponent, CacheInfo as CacheInfoData, ParquetCacheUsage};
use crate::components::toast::use_toast;
use crate::utils::{fetch_api, ApiResponse};


#[derive(Deserialize, Clone)]
struct ExecutionMetricsResponse {
    plan_id: String,
    execution_time_ms: u64,
    cache_hits: u64,
    cache_misses: u64,
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

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
struct FlameGraphParams {
    output_dir: String,
}

/// Default Home Page - LiquidCache Server Monitoring Dashboard
#[component]
pub fn Home() -> impl IntoView {
    let toast = use_toast();
    let (server_address, set_server_address) = signal("http://localhost:53703".to_string());
    let (cache_usage, set_cache_usage) = signal(None::<ParquetCacheUsage>);
    let (cache_info, set_cache_info) = signal(None::<CacheInfoData>);
    let (system_info, set_system_info) = signal(None);
    
    // New signals for the additional features
    let (trace_active, set_trace_active) = signal(false);
    let (trace_path, set_trace_path) = signal("/tmp".to_string());
    let (metrics_plan_id, set_metrics_plan_id) = signal(String::new());
    let (execution_metrics, set_execution_metrics) = signal(None);
    let (stats_path, set_stats_path) = signal("/tmp".to_string());
    let (flamegraph_active, set_flamegraph_active) = signal(false);
    let (flamegraph_output_dir, set_flamegraph_output_dir) = signal("/tmp".to_string());

    let fetch_cache_usage = {
        let toast = toast.clone();
        Action::new(move |_: &()| {
            let address = server_address.get();
            let toast = toast.clone();

            async move {
                match fetch_api::<ParquetCacheUsage>(&format!("{}/parquet_cache_usage", address)).await
                {
                    Ok(response) => {
                        set_cache_usage.set(Some(response));
                    }
                    Err(e) => {
                        toast.show_error(format!("Failed to fetch cache usage: {}", e));
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
                match fetch_api::<CacheInfoData>(&format!("{}/cache_info", address)).await {
                    Ok(response) => {
                        logging::log!("Cache info: {:?}", response);
                        set_cache_info.set(Some(response));
                    }
                    Err(e) => {
                        logging::error!("Failed to fetch cache info: {}", e);
                        toast.show_error(format!("Failed to fetch cache info: {}", e));
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
                match fetch_api::<SystemInfoData>(&format!("{}/system_info", address)).await {
                    Ok(response) => {
                        set_system_info.set(Some(response));
                    }
                    Err(e) => {
                        toast.show_error(format!("Failed to fetch system info: {}", e));
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
                match fetch_api::<ApiResponse>(&format!("{}/start_trace", address)).await {
                    Ok(response) => {
                        toast.show_success(response.message);
                        set_trace_active.set(true);
                    }
                    Err(e) => {
                        toast.show_error(format!("Failed to start trace: {}", e));
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
                match fetch_api::<ApiResponse>(&format!("{}/stop_trace?path={}", address, path)).await {
                    Ok(response) => {
                        toast.show_success(response.message);
                        set_trace_active.set(false);
                    }
                    Err(e) => {
                        toast.show_error(format!("Failed to stop trace: {}", e));
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
                    "{}/execution_metrics?plan_id={}", 
                    address, 
                    plan_id
                )).await {
                    Ok(Some(response)) => {
                        set_execution_metrics.set(Some(response));
                    }
                    Ok(None) => {
                        toast.show_error(format!("No metrics found for plan ID: {}", plan_id));
                        set_execution_metrics.set(None);
                    }
                    Err(e) => {
                        toast.show_error(format!("Failed to fetch execution metrics: {}", e));
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
                match fetch_api::<ApiResponse>(&format!(
                    "{}/cache_stats?path={}", 
                    address, 
                    path
                )).await {
                    Ok(response) => {
                        toast.show_success(response.message);
                    }
                    Err(e) => {
                        toast.show_error(format!("Failed to get cache stats: {}", e));
                    }
                }
            }
        })
    };

    // Action for starting flamegraph collection
    let start_flamegraph = {
        let toast = toast.clone();
        Action::new(move |_: &()| {
            let address = server_address.get();
            let toast = toast.clone();

            async move {
                match fetch_api::<ApiResponse>(&format!("{}/start_flamegraph", address)).await {
                    Ok(response) => {
                        toast.show_success(response.message);
                        set_flamegraph_active.set(true);
                    }
                    Err(e) => {
                        toast.show_error(format!("Failed to start flamegraph: {}", e));
                    }
                }
            }
        })
    };

    // Action for stopping flamegraph collection
    let stop_flamegraph = {
        let toast = toast.clone();
        Action::new(move |_: &()| {
            let address = server_address.get();
            let output_dir = flamegraph_output_dir.get();
            let toast = toast.clone();

            async move {
                match fetch_api::<ApiResponse>(&format!(
                    "{}/stop_flamegraph?output_dir={}", 
                    address, 
                    output_dir
                )).await {
                    Ok(response) => {
                        toast.show_success(response.message);
                        set_flamegraph_active.set(false);
                    }
                    Err(e) => {
                        toast.show_error(format!("Failed to stop flamegraph: {}", e));
                    }
                }
            }
        })
    };

    let fetch_all_data = move |_| {
        fetch_cache_usage.dispatch(());
        fetch_cache_info.dispatch(());
        fetch_system_info.dispatch(());
    };

    view! {
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
            <div class="container mx-auto px-4 py-8 max-w-4xl bg-gray-50 min-h-screen">
                <h1 class="text-2xl font-medium text-gray-800 mb-8 border-b border-gray-200 pb-3">
                    "LiquidCache Monitor"
                </h1>

                <div class="mb-8">
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
                            on:click=fetch_all_data
                        >
                            "Connect"
                        </button>
                    </div>
                </div>

                <div class="space-y-6 mb-8">
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

                    <div class="border border-gray-200 rounded-lg bg-white p-6">
                        <div class="flex justify-between items-center mb-4">
                            <h2 class="text-lg font-medium text-gray-700">"Trace Collection"</h2>
                        </div>
                        <div class="text-sm space-y-4">
                            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-end">
                                <div class="col-span-2">
                                    <label class="block text-gray-600 mb-1">"Save Path"</label>
                                    <input
                                        type="text"
                                        placeholder="Path to save trace data"
                                        class="w-full px-3 py-2 border border-gray-200 rounded focus:outline-none focus:border-gray-400 text-sm text-gray-700"
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
                            </div>

                            <div class="text-xs text-gray-500 mt-2">
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

                    <div class="border border-gray-200 rounded-lg bg-white p-6">
                        <div class="flex justify-between items-center mb-4">
                            <h2 class="text-lg font-medium text-gray-700">"Cache Statistics"</h2>
                        </div>
                        <div class="text-sm space-y-4">
                            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-end">
                                <div class="col-span-2">
                                    <label class="block text-gray-600 mb-1">"Save Path"</label>
                                    <input
                                        type="text"
                                        placeholder="Path to save cache statistics"
                                        class="w-full px-3 py-2 border border-gray-200 rounded focus:outline-none focus:border-gray-400 text-sm text-gray-700"
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
                    </div>

                    <div class="border border-gray-200 rounded-lg bg-white p-6">
                        <div class="flex justify-between items-center mb-4">
                            <h2 class="text-lg font-medium text-gray-700">"Execution Metrics"</h2>
                        </div>
                        <div class="text-sm space-y-4">
                            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-end">
                                <div class="col-span-2">
                                    <label class="block text-gray-600 mb-1">"Plan ID"</label>
                                    <input
                                        type="text"
                                        placeholder="Enter plan UUID"
                                        class="w-full px-3 py-2 border border-gray-200 rounded focus:outline-none focus:border-gray-400 text-sm text-gray-700"
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
                            </div>

                            {move || match execution_metrics.get() {
                                Some(metrics) => {
                                    view! {
                                        <div class="mt-4 bg-gray-50 p-4 rounded border border-gray-100">
                                            <h3 class="text-sm font-medium text-gray-600 mb-2">
                                                "Metrics for Plan: " {metrics.plan_id.clone()}
                                            </h3>
                                            <div class="grid grid-cols-2 gap-y-2">
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
                                        <div class="text-gray-400 text-sm italic mt-2">
                                            "Enter a plan ID and click 'Get Metrics' to view execution metrics"
                                        </div>
                                    }
                                        .into_any()
                                }
                            }}
                        </div>
                    </div>

                    <div class="border border-gray-200 rounded-lg bg-white p-6">
                        <div class="flex justify-between items-center mb-4">
                            <h2 class="text-lg font-medium text-gray-700">"Flamegraph"</h2>
                        </div>
                        <div class="text-sm space-y-4">
                            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-end">
                                <div class="col-span-2">
                                    <label class="block text-gray-600 mb-1">
                                        "Output Directory"
                                    </label>
                                    <input
                                        type="text"
                                        placeholder="Path to save flamegraph"
                                        class="w-full px-3 py-2 border border-gray-200 rounded focus:outline-none focus:border-gray-400 text-sm text-gray-700"
                                        prop:value=flamegraph_output_dir
                                        on:input=move |ev| {
                                            set_flamegraph_output_dir.set(event_target_value(&ev));
                                        }
                                        disabled=move || flamegraph_active.get()
                                    />
                                </div>
                                <div>
                                    {move || {
                                        if flamegraph_active.get() {
                                            view! {
                                                <button
                                                    class="w-full px-3 py-2 border border-red-100 rounded text-red-500 hover:bg-red-50 transition-colors text-sm"
                                                    on:click=move |_| {
                                                        stop_flamegraph.dispatch(());
                                                    }
                                                >
                                                    "Stop Flamegraph"
                                                </button>
                                            }
                                                .into_any()
                                        } else {
                                            view! {
                                                <button
                                                    class="w-full px-3 py-2 border border-gray-200 rounded text-gray-600 hover:bg-gray-50 transition-colors text-sm"
                                                    on:click=move |_| {
                                                        start_flamegraph.dispatch(());
                                                    }
                                                >
                                                    "Start Flamegraph"
                                                </button>
                                            }
                                                .into_any()
                                        }
                                    }}
                                </div>
                            </div>

                            <div class="text-xs text-gray-500 mt-2">
                                {move || {
                                    if flamegraph_active.get() {
                                        "Flamegraph collection is active. Stop to generate the flamegraph."
                                    } else {
                                        "Start flamegraph collection to profile server performance."
                                    }
                                }}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </ErrorBoundary>
    }
}
