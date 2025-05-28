use leptos::logging;
use serde::{de::DeserializeOwned, Deserialize};

// Helper function to format bytes to human-readable format
pub fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

// Helper function to format unix timestamp to local time
pub fn format_timestamp(timestamp: u64) -> String {
    let js_date = js_sys::Date::new(&(timestamp as f64 * 1000.0).into());
    let hours = js_date.get_hours();
    let minutes = js_date.get_minutes();
    let seconds = js_date.get_seconds();
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

pub fn format_duration(duration_str: &str) -> String {
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

pub fn format_number(num_str: &str) -> String {
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

#[derive(Deserialize, Clone)]
pub struct ApiResponse {
    pub message: String,
}
