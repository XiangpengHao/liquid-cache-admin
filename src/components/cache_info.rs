use leptos::prelude::*;
use serde::Deserialize;

use crate::utils::format_bytes;

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
type ActionCallback = Box<dyn Fn() + 'static>;

#[component]
pub fn CacheInfo(
    cache_info: ReadSignal<Option<CacheInfo>>,
    cache_usage: ReadSignal<Option<ParquetCacheUsage>>,
    on_refresh: RefreshCallback,
    on_reset_cache: ActionCallback,
    on_shutdown_server: ActionCallback,
) -> impl IntoView {
    view! {
        <div class="border border-gray-200 rounded-lg bg-white p-6">
            <div class="flex justify-between items-center mb-4">
                <h2 class="text-lg font-medium text-gray-700">"Cache Information"</h2>
                <button
                    class="text-xs text-gray-500 hover:text-gray-700"
                    on:click=move |_| on_refresh()
                >
                    "Refresh"
                </button>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                {move || match cache_info.get() {
                    Some(info) => {
                        view! {
                            <div class="text-sm space-y-2">
                                <h3 class="text-gray-500 mb-2">"Configuration"</h3>
                                <div class="grid grid-cols-2 gap-y-2">
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
                                <div class="mt-3 pt-2 border-t border-gray-100">
                                    <div class="w-full bg-gray-100 rounded-full h-1.5 mb-1">
                                        <div
                                            class="bg-gray-400 h-1.5 rounded-full"
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
                            <div class="text-gray-400 text-sm italic">
                                "Connect to view cache configuration"
                            </div>
                        }
                            .into_any()
                    }
                }}
                {move || match cache_usage.get() {
                    Some(usage) => {
                        view! {
                            <div class="text-sm space-y-2">
                                <h3 class="text-gray-500 mb-2">"Storage"</h3>
                                <div class="grid grid-cols-2 gap-y-2">
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
                            <div class="text-gray-400 text-sm italic">
                                "Connect to view cache usage"
                            </div>
                        }
                            .into_any()
                    }
                }}
            </div>
            <div class="flex gap-3 mt-4 pt-4 border-t border-gray-100">
                <button
                    class="px-3 py-1.5 border border-gray-200 rounded text-gray-600 hover:bg-gray-50 transition-colors text-xs"
                    on:click=move |_| on_reset_cache()
                >
                    "Reset Cache"
                </button>
                <button
                    class="px-3 py-1.5 border border-red-100 rounded text-red-500 hover:bg-red-50 transition-colors text-xs"
                    on:click=move |_| on_shutdown_server()
                >
                    "Shutdown Server"
                </button>
            </div>
        </div>
    }
}
