use leptos::prelude::*;

use crate::models::execution_plan::Statistics;

#[component]
fn StatisticsContent(stats: Statistics) -> impl IntoView {
    let columns = stats.column_statistics;
    let num_rows = stats.num_rows.clone();
    let total_byte_size = stats.total_byte_size.clone();

    view! {
        <div>
            <div class="flex gap-4 mb-2">
                <div class="truncate">
                    <span class="text-gray-500">"Num rows: "</span>
                    <span class="text-gray-800">{num_rows}</span>
                </div>
                <div class="truncate">
                    <span class="text-gray-500">"Total byte size: "</span>
                    <span class="text-gray-800">{total_byte_size}</span>
                </div>
            </div>

            <div class="mt-2">
                <div class="font-medium mb-1">"Column Statistics:"</div>
                <div class="space-y-1 max-h-32 overflow-y-auto">
                    {columns
                        .into_iter()
                        .map(|col| {
                            view! {
                                <div class="text-xs bg-white border border-gray-100 rounded p-1">
                                    <div class="font-medium text-gray-700">{col.name}</div>
                                    <div class="grid grid-cols-4 gap-1 text-xs">
                                        {if let Some(min) = &col.min {
                                            view! {
                                                <div class="truncate">
                                                    <span class="text-gray-500">"Min: "</span>
                                                    <span class="text-gray-800">{min.clone()}</span>
                                                </div>
                                            }
                                                .into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }}
                                        {if let Some(max) = &col.max {
                                            view! {
                                                <div class="truncate">
                                                    <span class="text-gray-500">"Max: "</span>
                                                    <span class="text-gray-800">{max.clone()}</span>
                                                </div>
                                            }
                                                .into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }}
                                        {if let Some(sum) = &col.sum {
                                            view! {
                                                <div class="truncate">
                                                    <span class="text-gray-500">"Sum: "</span>
                                                    <span class="text-gray-800">{sum.clone()}</span>
                                                </div>
                                            }
                                                .into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }}
                                        {if let Some(null) = &col.null {
                                            view! {
                                                <div class="truncate">
                                                    <span class="text-gray-500">"Null: "</span>
                                                    <span class="text-gray-800">{null.clone()}</span>
                                                </div>
                                            }
                                                .into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }}
                                        {if let Some(distinct) = &col.distinct_count {
                                            view! {
                                                <div class="truncate">
                                                    <span class="text-gray-500">"Distinct: "</span>
                                                    <span class="text-gray-800">{distinct.clone()}</span>
                                                </div>
                                            }
                                                .into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }}
                                    </div>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn StatisticsComponent(stats: Statistics) -> impl IntoView {
    let (expand_statistics, set_expand_statistics) = signal(false);

    view! {
        <div class="text-xs rounded">
            <button
                class="flex items-center gap-1 text-xs text-gray-600 hover:text-gray-800 transition-colors mb-2 font-medium"
                on:click=move |_| set_expand_statistics.update(|e| *e = !*e)
            >
                <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                    <path
                        fill-rule="evenodd"
                        d=move || {
                            if expand_statistics.get() {
                                "M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
                            } else {
                                "M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
                            }
                        }
                        clip-rule="evenodd"
                    />
                </svg>
                "Statistics"
            </button>

            <Show when=move || expand_statistics.get()>
                <StatisticsContent stats=stats.clone() />
            </Show>
        </div>
    }
}
