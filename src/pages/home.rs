use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
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
            <div class="container mx-auto px-4 py-12 max-w-4xl">
                <h1 class="text-3xl font-bold text-blue-800 mb-6 text-center">
                    "Welcome to Liquid Cache Admin"
                </h1>

                <div class="bg-white shadow-md rounded-lg p-6">
                    <p class="text-gray-700 mb-4">
                        "Manage your cache settings and monitor performance from this dashboard."
                    </p>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mt-8">
                        <div class="bg-blue-50 p-4 rounded-lg border border-blue-200">
                            <h2 class="text-xl font-semibold text-blue-700 mb-2">"Cache Status"</h2>
                            <p class="text-gray-600">
                                "Your cache is currently running optimally."
                            </p>
                        </div>

                        <div class="bg-blue-50 p-4 rounded-lg border border-blue-200">
                            <h2 class="text-xl font-semibold text-blue-700 mb-2">
                                "Quick Actions"
                            </h2>
                            <div class="flex space-x-2">
                                <button class="bg-blue-600 hover:bg-blue-700 text-white px-3 py-1 rounded text-sm transition-colors">
                                    "Flush Cache"
                                </button>
                                <button class="bg-gray-200 hover:bg-gray-300 text-gray-800 px-3 py-1 rounded text-sm transition-colors">
                                    "View Stats"
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </ErrorBoundary>
    }
}
