use leptos::prelude::*;
use serde::Deserialize;

use crate::{
    components::toast::use_toast,
    utils::{fetch_api, format_bytes, ApiResponse},
};

#[derive(Deserialize, Clone)]
pub struct ParquetCacheUsage {
    pub directory: String,
    pub file_count: usize,
    pub total_size_bytes: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CacheInfo {
    pub batch_size: usize,
    pub max_cache_bytes: u64,
    pub memory_usage_bytes: u64,
    pub disk_usage_bytes: u64,
}

type RefreshCallback = Box<dyn Fn() + 'static>;

#[component]
pub fn CacheInfo(
    cache_info: ReadSignal<Option<CacheInfo>>,
    cache_usage: ReadSignal<Option<ParquetCacheUsage>>,
    on_refresh: RefreshCallback,
    server_address: ReadSignal<String>,
) -> impl IntoView {
    let toast = use_toast();
    let reset_cache = {
        let toast = toast.clone();
        Action::new(move |_: &()| {
            let toast = toast.clone();
            let server_address = server_address.get();

            async move {
                match fetch_api::<ApiResponse>(&format!("{server_address}/reset_cache")).await {
                    Ok(response) => {
                        toast.show_success(response.message);
                    }
                    Err(e) => {
                        toast.show_error(format!("Failed to reset cache: {e}"));
                    }
                }
            }
        })
    };

    let shutdown_server = {
        let toast = toast.clone();
        Action::new(move |_: &()| {
            let address = server_address.get();
            let toast = toast.clone();

            async move {
                match fetch_api::<ApiResponse>(&format!("{address}/shutdown")).await {
                    Ok(response) => {
                        toast.show_success(response.message);
                    }
                    Err(e) => {
                        toast.show_error(format!("Failed to shutdown server: {e}"));
                    }
                }
            }
        })
    };

    view! {
        <div class="border border-gray-200 rounded-lg bg-white p-4">
            <div class="flex justify-between items-center mb-3">
                <h2 class="text-base font-medium text-gray-700">"Cache Information"</h2>
                <button
                    class="text-xs text-gray-500 hover:text-gray-700 px-2 py-1 rounded hover:bg-gray-50"
                    on:click=move |_| on_refresh()
                >
                    "Refresh"
                </button>
            </div>
            <div class="space-y-3">
                {move || match cache_info.get() {
                    Some(info) => {
                        view! {
                            <div class="text-sm">
                                <h3 class="text-gray-500 mb-2 text-xs font-medium">
                                    "Configuration"
                                </h3>
                                <div class="grid grid-cols-2 gap-y-1 gap-x-3 text-xs">
                                    <span class="text-gray-500">"Batch Size"</span>
                                    <span class="text-gray-800">{info.batch_size}</span>

                                    <span class="text-gray-500">"Max Cache"</span>
                                    <span class="text-gray-800">
                                        {format_bytes(info.max_cache_bytes)}
                                    </span>

                                    <span class="text-gray-500">"Memory Usage"</span>
                                    <span class="text-gray-800">
                                        {format_bytes(info.memory_usage_bytes)}
                                    </span>

                                    <span class="text-gray-500">"Disk Usage"</span>
                                    <span class="text-gray-800">
                                        {format_bytes(info.disk_usage_bytes)}
                                    </span>
                                </div>
                                <div class="mt-2 pt-2 border-t border-gray-100">
                                    <div class="w-full bg-gray-100 rounded-full h-1 mb-1">
                                        <div
                                            class="bg-gray-400 h-1 rounded-full"
                                            style=format!(
                                                "width: {}%",
                                                if info.max_cache_bytes > 0 {
                                                    info.memory_usage_bytes as f64 / info.max_cache_bytes as f64
                                                        * 100.0
                                                } else {
                                                    0.0
                                                },
                                            )
                                        ></div>
                                    </div>
                                    <div class="text-xs text-gray-500 text-right">
                                        {format!(
                                            "{}% utilized",
                                            if info.max_cache_bytes > 0 {
                                                format!(
                                                    "{:.1}",
                                                    info.memory_usage_bytes as f64 / info.max_cache_bytes as f64
                                                        * 100.0,
                                                )
                                            } else {
                                                "0.0".to_string()
                                            },
                                        )}
                                    </div>
                                </div>
                            </div>
                        }
                            .into_any()
                    }
                    None => {
                        view! {
                            <div class="text-gray-400 text-xs italic">
                                "Connect to view cache configuration"
                            </div>
                        }
                            .into_any()
                    }
                }}
                {move || match cache_usage.get() {
                    Some(usage) => {
                        view! {
                            <div class="text-sm border-t border-gray-100 pt-3">
                                <h3 class="text-gray-500 mb-2 text-xs font-medium">"Storage"</h3>
                                <div class="grid grid-cols-2 gap-y-1 gap-x-3 text-xs">
                                    <span class="text-gray-500">"Directory"</span>
                                    <span
                                        class="text-gray-800 truncate"
                                        title=usage.directory.clone()
                                    >
                                        {usage.directory.clone()}
                                    </span>

                                    <span class="text-gray-500">"File Count"</span>
                                    <span class="text-gray-800">{usage.file_count}</span>

                                    <span class="text-gray-500">"Total Size"</span>
                                    <span class="text-gray-800">
                                        {format_bytes(usage.total_size_bytes)}
                                    </span>
                                </div>
                            </div>
                        }
                            .into_any()
                    }
                    None => {
                        view! {
                            <div class="text-gray-400 text-xs italic border-t border-gray-100 pt-3">
                                "Connect to view cache usage"
                            </div>
                        }
                            .into_any()
                    }
                }}
            </div>
            <div class="flex gap-2 mt-3 pt-3 border-t border-gray-100">
                <button
                    class="px-2 py-1 border border-gray-200 rounded text-gray-600 hover:bg-gray-50 transition-colors text-xs"
                    on:click=move |_| {
                        reset_cache.dispatch(());
                    }
                >
                    "Reset Cache"
                </button>
                <button
                    class="px-2 py-1 border border-red-100 rounded text-red-500 hover:bg-red-50 transition-colors text-xs"
                    on:click=move |_| {
                        shutdown_server.dispatch(());
                    }
                >
                    "Shutdown Server"
                </button>
            </div>
        </div>
    }
}
