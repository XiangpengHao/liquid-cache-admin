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

