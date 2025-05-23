use leptos::prelude::*;
use serde::Deserialize;

use crate::utils::format_bytes;

#[derive(Deserialize, Clone)]
pub struct SystemInfo {
    pub total_memory_bytes: u64,
    pub used_memory_bytes: u64,
    pub name: String,
    pub kernel: String,
    pub os: String,
    pub host_name: String,
    pub cpu_cores: usize,
    pub server_resident_memory_bytes: u64,
    pub server_virtual_memory_bytes: u64,
}

type RefreshCallback = Box<dyn Fn() + 'static>;

#[component]
pub fn SystemInfo(
    system_info: ReadSignal<Option<SystemInfo>>,
    on_refresh: RefreshCallback,
) -> impl IntoView {
    view! {
        <div class="border border-gray-200 rounded-lg bg-white p-4">
            <div class="flex justify-between items-center mb-3">
                <h2 class="text-base font-medium text-gray-700">"System Information"</h2>
                <button
                    class="text-xs text-gray-500 hover:text-gray-700 px-2 py-1 rounded hover:bg-gray-50"
                    on:click=move |_| on_refresh()
                >
                    "Refresh"
                </button>
            </div>
            {move || match system_info.get() {
                Some(info) => {
                    view! {
                        <div class="grid grid-cols-2 gap-y-1 gap-x-4 text-sm">
                            <span class="text-gray-500 text-xs">"Host Name"</span>
                            <span class="text-gray-800 text-xs truncate">
                                {info.host_name.clone()}
                            </span>

                            <span class="text-gray-500 text-xs">"OS"</span>
                            <span class="text-gray-800 text-xs truncate">
                                {format!("{} ({})", info.name, info.os)}
                            </span>

                            <span class="text-gray-500 text-xs">"Kernel"</span>
                            <span class="text-gray-800 text-xs truncate">
                                {info.kernel.clone()}
                            </span>

                            <span class="text-gray-500 text-xs">"CPU Cores"</span>
                            <span class="text-gray-800 text-xs">{info.cpu_cores}</span>

                            <span class="text-gray-500 text-xs">"Memory"</span>
                            <span class="text-gray-800 text-xs">
                                {format!(
                                    "{} / {} used",
                                    format_bytes(info.used_memory_bytes),
                                    format_bytes(info.total_memory_bytes),
                                )}
                            </span>

                            <span class="text-gray-500 text-xs">"Server Resident"</span>
                            <span class="text-gray-800 text-xs">
                                {format_bytes(info.server_resident_memory_bytes)}
                            </span>

                            <span class="text-gray-500 text-xs">"Server Virtual"</span>
                            <span class="text-gray-800 text-xs">
                                {format_bytes(info.server_virtual_memory_bytes)}
                            </span>
                        </div>
                    }
                        .into_any()
                }
                None => {
                    view! {
                        <div class="text-gray-400 text-xs italic">
                            "Connect to view system information"
                        </div>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}
