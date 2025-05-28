use std::sync::Arc;

use crate::components::cache_info::{
    CacheInfo as CacheInfoComponent, CacheInfo as CacheInfoData, ParquetCacheUsage,
};
use crate::components::execution_plans::ExecutionStats as ExecutionPlansComponent;
use crate::components::system_info::{
    SystemInfo as SystemInfoComponent, SystemInfo as SystemInfoData,
};
use crate::components::toast::use_toast;
use crate::models::execution_plan::ExecutionStatsWithPlan;
use crate::utils::fetch_api;
use leptos::{logging, prelude::*};
use leptos_router::{hooks::use_navigate, hooks::use_query_map};
use serde::Deserialize;

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

    let (execution_stats, set_execution_stats) = signal(None::<Arc<Vec<ExecutionStatsWithPlan>>>);

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
                match fetch_api::<Vec<ExecutionStatsWithPlan>>(&format!(
                    "{address}/execution_plans"
                ))
                .await
                {
                    Ok(response) => {
                        set_execution_stats.set(Some(Arc::new(response)));
                    }
                    Err(e) => {
                        toast.show_error(format!("Failed to fetch execution plans: {e}"));
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

                        {move || {
                            if let Some(plans) = execution_stats.get() {
                                view! {
                                    <ExecutionPlansComponent
                                        execution_stats=plans
                                        on_refresh=Box::new(move || {
                                            fetch_execution_plans.dispatch(());
                                        })
                                    />
                                }
                                    .into_any()
                            } else {
                                view! { <div class="text-gray-500">"No execution found"</div> }
                                    .into_any()
                            }
                        }}
                    </div>

                </div>
            </ErrorBoundary>
        </div>
    }
}
