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
        let abort_controller = SendWrapper::new(web_sys::AbortController::new().ok());
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
            match fetch_api::<TablesResponse>(&format!("{}/get_registered_tables", address)).await {
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
            match fetch_api::<ParquetCacheUsage>(&format!("{}/parquet_cache_usage", address)).await
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
            match fetch_api::<CacheInfo>(&format!("{}/cache_info", address)).await {
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
            match fetch_api::<SystemInfo>(&format!("{}/system_info", address)).await {
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
            match fetch_api::<ApiResponse>(&format!("{}/reset_cache", address)).await {
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
            match fetch_api::<ApiResponse>(&format!("{}/shutdown", address)).await {
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

                {move || {
                    error
                        .get()
                        .map(|err| {
                            view! {
                                <div class="bg-red-50 border border-red-100 text-red-600 px-4 py-3 rounded mb-6 text-sm">
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
                                <div class="bg-green-50 border border-green-100 text-green-600 px-4 py-3 rounded mb-6 text-sm">
                                    <p>{msg}</p>
                                </div>
                            }
                        })
                }}

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
                            disabled=loading
                        >
                            {move || if loading.get() { "Connecting..." } else { "Connect" }}
                        </button>
                    </div>
                </div>

                <div class="space-y-6 mb-8">
                    <div class="border border-gray-200 rounded-lg bg-white p-6">
                        <div class="flex justify-between items-center mb-4">
                            <h2 class="text-lg font-medium text-gray-700">"System Information"</h2>
                            <button
                                class="text-xs text-gray-500 hover:text-gray-700"
                                on:click=move |_| {
                                    fetch_system_info.dispatch(());
                                }
                                disabled=loading
                            >
                                "Refresh"
                            </button>
                        </div>
                        {move || match system_info.get() {
                            Some(info) => {
                                view! {
                                    <div class="grid grid-cols-2 gap-y-2 text-sm">
                                        <span class="text-gray-500">"Host Name"</span>
                                        <span class="text-gray-800">{info.host_name}</span>

                                        <span class="text-gray-500">"OS"</span>
                                        <span class="text-gray-800">
                                            {format!("{} ({})", info.name, info.os)}
                                        </span>

                                        <span class="text-gray-500">"Kernel"</span>
                                        <span class="text-gray-800">{info.kernel}</span>

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

                    <div class="border border-gray-200 rounded-lg bg-white p-6">
                        <div class="flex justify-between items-center mb-4">
                            <h2 class="text-lg font-medium text-gray-700">"Cache Information"</h2>
                            <button
                                class="text-xs text-gray-500 hover:text-gray-700"
                                on:click=move |_| {
                                    fetch_cache_info.dispatch(());
                                    fetch_cache_usage.dispatch(());
                                }
                                disabled=loading
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
                                on:click=move |_| {
                                    reset_cache.dispatch(());
                                }
                                disabled=loading
                            >
                                "Reset Cache"
                            </button>
                            <button
                                class="px-3 py-1.5 border border-red-100 rounded text-red-500 hover:bg-red-50 transition-colors text-xs"
                                on:click=move |_| {
                                    shutdown_server.dispatch(());
                                }
                                disabled=loading
                            >
                                "Shutdown Server"
                            </button>
                        </div>
                    </div>

                    <div class="border border-gray-200 rounded-lg bg-white p-6">
                        <div class="flex justify-between items-center mb-4">
                            <h2 class="text-lg font-medium text-gray-700">"Registered Tables"</h2>
                            <button
                                class="text-xs text-gray-500 hover:text-gray-700"
                                on:click=move |_| {
                                    fetch_tables.dispatch(());
                                }
                                disabled=loading
                            >
                                "Refresh"
                            </button>
                        </div>
                        {move || match tables.get() {
                            Some(table_list) if !table_list.is_empty() => {
                                view! {
                                    <div class="overflow-x-auto -mx-6">
                                        <table class="min-w-full border-collapse">
                                            <thead>
                                                <tr>
                                                    <th class="py-2 px-6 text-left font-medium text-gray-500 text-sm border-b border-gray-100">
                                                        "Table Name"
                                                    </th>
                                                    <th class="py-2 px-6 text-left font-medium text-gray-500 text-sm border-b border-gray-100">
                                                        "Path"
                                                    </th>
                                                    <th class="py-2 px-6 text-left font-medium text-gray-500 text-sm border-b border-gray-100">
                                                        "Cache Mode"
                                                    </th>
                                                </tr>
                                            </thead>
                                            <tbody class="text-sm">
                                                {table_list
                                                    .into_iter()
                                                    .map(|table| {
                                                        view! {
                                                            <tr>
                                                                <td class="py-2 px-6 border-b border-gray-50 text-gray-800">
                                                                    {table.name}
                                                                </td>
                                                                <td
                                                                    class="py-2 px-6 border-b border-gray-50 font-mono text-xs text-gray-600 truncate"
                                                                    title=table.path.clone()
                                                                >
                                                                    {table.path.clone()}
                                                                </td>
                                                                <td class="py-2 px-6 border-b border-gray-50 text-gray-800">
                                                                    {table.cache_mode}
                                                                </td>
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
                                view! {
                                    <div class="text-gray-400 text-sm italic">
                                        "No tables registered"
                                    </div>
                                }
                                    .into_any()
                            }
                            None => {
                                view! {
                                    <div class="text-gray-400 text-sm italic">
                                        "Connect to view registered tables"
                                    </div>
                                }
                                    .into_any()
                            }
                        }}
                    </div>
                </div>
            </div>
        </ErrorBoundary>
    }
}
