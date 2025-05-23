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
        <div class="border border-gray-200 rounded-lg bg-white p-6">
            <div class="flex justify-between items-center mb-4">
                <h2 class="text-lg font-medium text-gray-700">"System Information"</h2>
                <button
                    class="text-xs text-gray-500 hover:text-gray-700"
                    on:click=move |_| on_refresh()
                >
                    "Refresh"
                </button>
            </div>
            {move || match system_info.get() {
                Some(info) => {
                    view! {
                        <div class="grid grid-cols-2 gap-y-2 text-sm">
                            <span class="text-gray-500">"Host Name"</span>
                            <span class="text-gray-800">{info.host_name.clone()}</span>

                            <span class="text-gray-500">"OS"</span>
                            <span class="text-gray-800">
                                {format!("{} ({})", info.name, info.os)}
                            </span>

                            <span class="text-gray-500">"Kernel"</span>
                            <span class="text-gray-800">{info.kernel.clone()}</span>

                            <span class="text-gray-500">"CPU Cores"</span>
                            <span class="text-gray-800">{info.cpu_cores}</span>

                            <span class="text-gray-500">"Memory"</span>
                            <span class="text-gray-800">
                                {format!(
                                    "{} / {} used",
                                    format_bytes(info.used_memory_bytes),
                                    format_bytes(info.total_memory_bytes),
                                )}
                            </span>

                            <span class="text-gray-500">"Server Resident Memory"</span>
                            <span class="text-gray-800">
                                {format_bytes(info.server_resident_memory_bytes)}
                            </span>

                            <span class="text-gray-500">"Server Virtual Memory"</span>
                            <span class="text-gray-800">
                                {format_bytes(info.server_virtual_memory_bytes)}
                            </span>
                        </div>
                    }
                        .into_any()
                }
                None => {
                    view! {
                        <div class="text-gray-400 text-sm italic">
                            "Connect to view system information"
                        </div>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}
