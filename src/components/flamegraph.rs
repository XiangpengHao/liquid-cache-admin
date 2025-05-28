use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

#[component]
pub fn Flamegraph(svg_content: String, plan_id: String) -> impl IntoView {
    let svg_for_download = svg_content.clone();
    let plan_id_for_download = plan_id.clone();

    let download_svg = move |_| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Ok(element) = document.create_element("a") {
                    let anchor = element.unchecked_into::<web_sys::HtmlAnchorElement>();

                    let data_url = format!(
                        "data:image/svg+xml;charset=utf-8,{}",
                        urlencoding::encode(&svg_for_download)
                    );

                    anchor.set_href(&data_url);
                    anchor.set_download(&format!("flamegraph-{plan_id_for_download}.svg"));

                    if let Some(body) = document.body() {
                        let _ = body.append_child(&anchor);
                        anchor.click();
                        let _ = body.remove_child(&anchor);
                    }
                }
            }
        }
    };

    view! {
        <div class="bg-white rounded overflow-auto mt-0">
            <iframe
                srcdoc=format!(
                    "<!DOCTYPE html><html><head><style>body{{margin:0;padding:0;}} svg{{width:100%;height:auto;}}</style></head><body>{}</body></html>",
                    svg_content,
                )
                class="w-full h-[600px] border-0"
                sandbox="allow-scripts allow-same-origin"
            ></iframe>
        </div>
        <button
            class="px-3 py-1 border border-gray-200 rounded text-gray-600 hover:bg-gray-50 transition-colors text-xs flex items-center gap-1"
            on:click=download_svg
        >
            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                ></path>
            </svg>
            "Download SVG"
        </button>
    }
}
