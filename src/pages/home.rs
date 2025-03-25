use leptos::{logging, prelude::*};
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Deserialize, Clone)]
struct ApiResponse {
    message: String,
}

#[derive(Deserialize, Clone)]
struct TableInfo {
    name: String,
    path: String,
    cache_mode: String,
}

#[derive(Deserialize, Clone)]
struct TablesResponse {
    tables: Vec<TableInfo>,
}

#[derive(Deserialize, Clone)]
struct ParquetCacheUsage {
    directory: String,
    file_count: usize,
    total_size_bytes: u64,
}

#[derive(Deserialize, Clone, Debug)]
struct CacheInfo {
    batch_size: usize,
    max_cache_bytes: u64,
    memory_usage_bytes: u64,
    disk_usage_bytes: u64,
}

#[derive(Deserialize, Clone)]
struct SystemInfo {
    total_memory_bytes: u64,
    used_memory_bytes: u64,
    available_memory_bytes: u64,
    name: String,
    kernel: String,
    os: String,
    host_name: String,
    cpu_cores: usize,
}

pub fn fetch_api<T>(
    path: &str,
) -> impl std::future::Future<Output = Result<T, gloo_net::Error>> + Send + '_
where
    T: DeserializeOwned,
{
    use leptos::prelude::on_cleanup;
    use send_wrapper::SendWrapper;

    SendWrapper::new(async move {
        let abort_controller =
            SendWrapper::new(web_sys::AbortController::new().ok());
        let abort_signal = abort_controller.as_ref().map(|a| a.signal());

        // abort in-flight requests if, e.g., we've navigated away from this page
        on_cleanup(move || {
            if let Some(abort_controller) = abort_controller.take() {
                abort_controller.abort()
            }
        });

        logging::log!("Fetching data from {}", path);

        let response = gloo_net::http::Request::get(path)
            .abort_signal(abort_signal.as_ref())
            .send()
            .await?;
        response.json().await
    })
}

// Helper function to format bytes to human-readable format
fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

/// Default Home Page - LiquidCache Server Monitoring Dashboard
#[component]
pub fn Home() -> impl IntoView {
    let (server_address, set_server_address) = signal("http://localhost:50052".to_string());
    let (tables, set_tables) = signal(None);
    let (cache_usage, set_cache_usage) = signal(None);
    let (cache_info, set_cache_info) = signal(None);
    let (system_info, set_system_info) = signal(None);
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None);
    let (success_message, set_success_message) = signal(None);

    let fetch_tables = Action::new(move |_: &()| {
        let address = server_address.get();
        set_loading.set(true);
        set_error.set(None);
        
        async move {
            match fetch_api::<TablesResponse>(&format!("{}/get_registered_tables", address))
                .await
            {
                Ok(response) => {
                    set_tables.set(Some(response.tables));
                    set_error.set(None);
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to fetch tables: {}", e)));
                }
            }
            set_loading.set(false);
        }
    });

    let fetch_cache_usage = Action::new(move |_: &()| {
        let address = server_address.get();
        set_loading.set(true);
        set_error.set(None);
        
        async move {
            match fetch_api::<ParquetCacheUsage>(&format!("{}/parquet_cache_usage", address))
                .await
            {
                Ok(response) => {
                    set_cache_usage.set(Some(response));
                    set_error.set(None);
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to fetch cache usage: {}", e)));
                }
            }
            set_loading.set(false);
        }
    });

    let fetch_cache_info = Action::new(move |_: &()| {
        let address = server_address.get();
        set_loading.set(true);
        set_error.set(None);
        
        async move {
            match fetch_api::<CacheInfo>(&format!("{}/cache_info", address))
                .await
            {
                Ok(response) => {
                    logging::log!("Cache info: {:?}", response);
                    set_cache_info.set(Some(response));
                    set_error.set(None);
                }
                Err(e) => {
                    logging::error!("Failed to fetch cache info: {}", e);
                    set_error.set(Some(format!("Failed to fetch cache info: {}", e)));
                }
            }
            set_loading.set(false);
        }
    });

    let fetch_system_info = Action::new(move |_: &()| {
        let address = server_address.get();
        set_loading.set(true);
        set_error.set(None);
        
        async move {
            match fetch_api::<SystemInfo>(&format!("{}/system_info", address))
                .await
            {
                Ok(response) => {
                    set_system_info.set(Some(response));
                    set_error.set(None);
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to fetch system info: {}", e)));
                }
            }
            set_loading.set(false);
        }
    });

    let reset_cache = Action::new(move |_: &()| {
        let address = server_address.get();
        set_loading.set(true);
        set_error.set(None);
        set_success_message.set(None);
        
        async move {
            match fetch_api::<ApiResponse>(&format!("{}/reset_cache", address))
                .await
            {
                Ok(response) => {
                    set_success_message.set(Some(response.message));
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to reset cache: {}", e)));
                }
            }
            set_loading.set(false);
        }
    });

    let shutdown_server = Action::new(move |_: &()| {
        let address = server_address.get();
        set_loading.set(true);
        set_error.set(None);
        set_success_message.set(None);
        
        async move {
            match fetch_api::<ApiResponse>(&format!("{}/shutdown", address))
                .await
            {
                Ok(response) => {
                    set_success_message.set(Some(response.message));
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to shutdown server: {}", e)));
                }
            }
            set_loading.set(false);
        }
    });

    let fetch_all_data = move |_| {
        fetch_tables.dispatch(());
        fetch_cache_usage.dispatch(());
        fetch_cache_info.dispatch(());
        fetch_system_info.dispatch(());
    };

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                <ul>
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
            <div class="container mx-auto px-4 py-8 max-w-5xl">
                <h1 class="text-3xl font-bold text-blue-800 mb-6 text-center">
                    "LiquidCache Server Monitor"
                </h1>

                {move || {
                    error
                        .get()
                        .map(|err| {
                            view! {
                                <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
                                    <p>{err}</p>
                                </div>
                            }
                        })
                }}

                {move || {
                    success_message
                        .get()
                        .map(|msg| {
                            view! {
                                <div class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded mb-4">
                                    <p>{msg}</p>
                                </div>
                            }
                        })
                }}

                <div class="bg-white shadow-md rounded-lg p-6 mb-6">
                    <h2 class="text-xl font-semibold text-blue-700 mb-4">"Server Connection"</h2>
                    <div class="flex items-center space-x-2">
                        <input
                            type="text"
                            placeholder="Server address (e.g. http://localhost:3000)"
                            class="w-full px-4 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                            prop:value=server_address
                            on:input=move |ev| {
                                set_server_address.set(event_target_value(&ev));
                            }
                        />
                        <button
                            class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded transition-colors"
                            on:click=fetch_all_data
                            disabled=loading
                        >
                            {move || if loading.get() { "Loading..." } else { "Connect" }}
                        </button>
                    </div>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
                    <div class="bg-white shadow-md rounded-lg p-6">
                        <div class="flex justify-between items-center mb-4">
                            <h2 class="text-xl font-semibold text-blue-700">
                                "System Information"
                            </h2>
                            <button
                                class="bg-gray-200 hover:bg-gray-300 text-gray-800 px-2 py-1 rounded transition-colors text-sm"
                                on:click=move |_| {
                                    let _ = fetch_system_info.dispatch(());
                                }
                                disabled=loading
                            >
                                "Refresh"
                            </button>
                        </div>
                        {move || match system_info.get() {
                            Some(info) => {
                                view! {
                                    <div class="space-y-3">
                                        <div class="flex justify-between">
                                            <span class="text-gray-600">"Host Name:"</span>
                                            <span class="font-medium">{info.host_name}</span>
                                        </div>
                                        <div class="flex justify-between">
                                            <span class="text-gray-600">"OS:"</span>
                                            <span class="font-medium">
                                                {format!("{} ({})", info.name, info.os)}
                                            </span>
                                        </div>
                                        <div class="flex justify-between">
                                            <span class="text-gray-600">"Kernel:"</span>
                                            <span class="font-medium">{info.kernel}</span>
                                        </div>
                                        <div class="flex justify-between">
                                            <span class="text-gray-600">"CPU Cores:"</span>
                                            <span class="font-medium">{info.cpu_cores}</span>
                                        </div>
                                        <div class="flex justify-between">
                                            <span class="text-gray-600">"Total Memory:"</span>
                                            <span class="font-medium">
                                                {format_bytes(info.total_memory_bytes)}
                                            </span>
                                        </div>
                                        <div class="flex justify-between">
                                            <span class="text-gray-600">"Used Memory:"</span>
                                            <span class="font-medium">
                                                {format_bytes(info.used_memory_bytes)}
                                            </span>
                                        </div>
                                        <div class="flex justify-between">
                                            <span class="text-gray-600">"Available Memory:"</span>
                                            <span class="font-medium">
                                                {format_bytes(info.available_memory_bytes)}
                                            </span>
                                        </div>
                                    </div>
                                }
                                    .into_any()
                            }
                            None => {
                                view! {
                                    <div class="text-gray-500 italic">
                                        "Connect to a server to view system information"
                                    </div>
                                }
                                    .into_any()
                            }
                        }}
                    </div>

                    <div class="bg-white shadow-md rounded-lg p-6">
                        <div class="flex justify-between items-center mb-4">
                            <h2 class="text-xl font-semibold text-blue-700">
                                "Cache Configuration"
                            </h2>
                            <button
                                class="bg-gray-200 hover:bg-gray-300 text-gray-800 px-2 py-1 rounded transition-colors text-sm"
                                on:click=move |_| {
                                    let _ = fetch_cache_info.dispatch(());
                                }
                                disabled=loading
                            >
                                "Refresh"
                            </button>
                        </div>
                        {move || match cache_info.get() {
                            Some(info) => {
                                view! {
                                    <div class="space-y-3">
                                        <div class="flex justify-between">
                                            <span class="text-gray-600">"Batch Size:"</span>
                                            <span class="font-medium">{info.batch_size}</span>
                                        </div>
                                        <div class="flex justify-between">
                                            <span class="text-gray-600">"Max Cache Size:"</span>
                                            <span class="font-medium">
                                                {format_bytes(info.max_cache_bytes as u64)}
                                            </span>
                                        </div>
                                        <div class="flex justify-between">
                                            <span class="text-gray-600">"Memory Usage:"</span>
                                            <span class="font-medium">
                                                {format_bytes(info.memory_usage_bytes as u64)}
                                            </span>
                                        </div>
                                        <div class="flex justify-between">
                                            <span class="text-gray-600">"Disk Usage:"</span>
                                            <span class="font-medium">
                                                {format_bytes(info.disk_usage_bytes as u64)}
                                            </span>
                                        </div>
                                        <div class="mt-4">
                                            <div class="text-gray-600 mb-1">"Memory Usage"</div>
                                            <div class="w-full bg-gray-200 rounded-full h-2.5">
                                                <div
                                                    class="bg-blue-600 h-2.5 rounded-full"
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
                                        </div>
                                    </div>
                                }
                                    .into_any()
                            }
                            None => {
                                view! {
                                    <div class="text-gray-500 italic">
                                        "Connect to a server to view cache configuration"
                                    </div>
                                }
                                    .into_any()
                            }
                        }}
                    </div>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
                    <div class="bg-white shadow-md rounded-lg p-6">
                        <div class="flex justify-between items-center mb-4">
                            <h2 class="text-xl font-semibold text-blue-700">"Cache Usage"</h2>
                            <button
                                class="bg-gray-200 hover:bg-gray-300 text-gray-800 px-2 py-1 rounded transition-colors text-sm"
                                on:click=move |_| {
                                    let _ = fetch_cache_usage.dispatch(());
                                }
                                disabled=loading
                            >
                                "Refresh"
                            </button>
                        </div>
                        {move || match cache_usage.get() {
                            Some(usage) => {
                                view! {
                                    <div class="space-y-3">
                                        <div class="flex justify-between">
                                            <span class="text-gray-600">"Cache Directory:"</span>
                                            <span class="font-medium">{usage.directory}</span>
                                        </div>
                                        <div class="flex justify-between">
                                            <span class="text-gray-600">"File Count:"</span>
                                            <span class="font-medium">{usage.file_count}</span>
                                        </div>
                                        <div class="flex justify-between">
                                            <span class="text-gray-600">"Total Size:"</span>
                                            <span class="font-medium">
                                                {format_bytes(usage.total_size_bytes)}
                                            </span>
                                        </div>
                                    </div>
                                }
                                    .into_any()
                            }
                            None => {
                                view! {
                                    <div class="text-gray-500 italic">
                                        "Connect to a server to view cache usage"
                                    </div>
                                }
                                    .into_any()
                            }
                        }}
                    </div>

                    <div class="bg-white shadow-md rounded-lg p-6">
                        <h2 class="text-xl font-semibold text-blue-700 mb-4">"Quick Actions"</h2>
                        <div class="flex flex-wrap gap-2">
                            <button
                                class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded transition-colors"
                                on:click=move |_| {
                                    let _ = reset_cache.dispatch(());
                                }
                                disabled=loading
                            >
                                "Reset Cache"
                            </button>
                            <button
                                class="bg-red-600 hover:bg-red-700 text-white px-4 py-2 rounded transition-colors"
                                on:click=move |_| {
                                    let _ = shutdown_server.dispatch(());
                                }
                                disabled=loading
                            >
                                "Shutdown Server"
                            </button>
                            <button
                                class="bg-gray-200 hover:bg-gray-300 text-gray-800 px-4 py-2 rounded transition-colors"
                                on:click=fetch_all_data
                                disabled=loading
                            >
                                "Refresh Data"
                            </button>
                        </div>
                    </div>
                </div>

                <div class="bg-white shadow-md rounded-lg p-6">
                    <div class="flex justify-between items-center mb-4">
                        <h2 class="text-xl font-semibold text-blue-700">"Registered Tables"</h2>
                        <button
                            class="bg-gray-200 hover:bg-gray-300 text-gray-800 px-2 py-1 rounded transition-colors text-sm"
                            on:click=move |_| {
                                let _ = fetch_tables.dispatch(());
                            }
                            disabled=loading
                        >
                            "Refresh"
                        </button>
                    </div>
                    {move || match tables.get() {
                        Some(table_list) if !table_list.is_empty() => {
                            view! {
                                <div class="overflow-x-auto">
                                    <table class="min-w-full bg-white">
                                        <thead>
                                            <tr class="bg-gray-100 text-gray-700 text-left">
                                                <th class="py-2 px-3 border-b">"Table Name"</th>
                                                <th class="py-2 px-3 border-b">"Path"</th>
                                                <th class="py-2 px-3 border-b">"Cache Mode"</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {table_list
                                                .into_iter()
                                                .map(|table| {
                                                    view! {
                                                        <tr class="border-b hover:bg-gray-50">
                                                            <td class="py-2 px-3">{table.name}</td>
                                                            <td class="py-2 px-3 text-sm font-mono">{table.path}</td>
                                                            <td class="py-2 px-3">{table.cache_mode}</td>
                                                        </tr>
                                                    }
                                                })
                                                .collect_view()}
                                        </tbody>
                                    </table>
                                </div>
                            }
                                .into_any()
                        }
                        Some(_) => {
                            view! { <div class="text-gray-500 italic">"No tables registered"</div> }
                                .into_any()
                        }
                        None => {
                            view! {
                                <div class="text-gray-500 italic">
                                    "Connect to a server to view registered tables"
                                </div>
                            }
                                .into_any()
                        }
                    }}
                </div>
            </div>
        </ErrorBoundary>
    }
}
