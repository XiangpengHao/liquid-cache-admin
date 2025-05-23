use leptos::{logging, prelude::*};
use leptos::task::spawn_local;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum ToastType {
    Success,
    Error,
    Info,
}

#[derive(Clone, Debug)]
pub struct Toast {
    pub id: Uuid,
    pub message: String,
    pub toast_type: ToastType,
    pub duration: Option<u64>, // duration in milliseconds, None for persistent
}

impl Toast {
    pub fn new(message: String, toast_type: ToastType, duration: Option<u64>) -> Self {
        Self {
            id: Uuid::new_v4(),
            message,
            toast_type,
            duration,
        }
    }

    pub fn success(message: String) -> Self {
        Self::new(message, ToastType::Success, Some(4000))
    }

    pub fn error(message: String) -> Self {
        Self::new(message, ToastType::Error, Some(6000))
    }

    pub fn info(message: String) -> Self {
        Self::new(message, ToastType::Info, Some(4000))
    }
}

#[derive(Clone, Debug)]
pub struct ToastContext {
    pub toasts: ReadSignal<HashMap<Uuid, Toast>>,
    pub add_toast: WriteSignal<Option<Toast>>,
    pub remove_toast: WriteSignal<Option<Uuid>>,
}

impl ToastContext {
    pub fn show_success(&self, message: String) {
        logging::log!("Showing success toast: {}", message);
        self.add_toast.set(Some(Toast::success(message)));
    }

    pub fn show_error(&self, message: String) {
        logging::error!("Showing error toast: {}", message);
        self.add_toast.set(Some(Toast::error(message)));
    }

	#[allow(dead_code)]
    pub fn show_info(&self, message: String) {
        logging::log!("Showing info toast: {}", message);
        self.add_toast.set(Some(Toast::info(message)));
    }

    pub fn remove(&self, id: Uuid) {
        self.remove_toast.set(Some(id));
    }
}

#[component]
pub fn ToastProvider(children: ChildrenFn) -> impl IntoView {
    let (toasts, set_toasts) = signal(HashMap::<Uuid, Toast>::new());
    let (add_toast, set_add_toast) = signal(None::<Toast>);
    let (remove_toast, set_remove_toast) = signal(None::<Uuid>);

    let toast_context = ToastContext {
        toasts,
        add_toast: set_add_toast,
        remove_toast: set_remove_toast,
    };

    provide_context(toast_context.clone());

    // Effect to add new toasts
    Effect::new(move || {
        if let Some(toast) = add_toast.get() {
            let toast_id = toast.id;
            let duration = toast.duration;
            
            set_toasts.update(|toasts| {
                toasts.insert(toast_id, toast);
            });

            // Auto-remove toast after duration
            if let Some(duration_ms) = duration {
                spawn_local(async move {
                    gloo_timers::future::TimeoutFuture::new(duration_ms as u32).await;
                    set_remove_toast.set(Some(toast_id));
                });
            }

            set_add_toast.set(None);
        }
    });

    // Effect to remove toasts
    Effect::new(move || {
        if let Some(toast_id) = remove_toast.get() {
            set_toasts.update(|toasts| {
                toasts.remove(&toast_id);
            });
            set_remove_toast.set(None);
        }
    });

    view! {
        {children()}
        <ToastContainer />
    }
}

#[component]
pub fn ToastContainer() -> impl IntoView {
    let toast_context = use_context::<ToastContext>()
        .expect("ToastContext must be provided");

    view! {
        <div class="fixed top-4 right-4 z-50 space-y-2 max-w-sm">
            <For
                each=move || { toast_context.toasts.get().into_iter().collect::<Vec<_>>() }
                key=|(id, _)| *id
                children={
                    let toast_context = toast_context.clone();
                    move |(id, toast)| {
                        let toast_context = toast_context.clone();
                        view! {
                            <ToastItem
                                toast=toast
                                on_close=move || {
                                    toast_context.remove(id);
                                }
                            />
                        }
                    }
                }
            />
        </div>
    }
}

#[component]
pub fn ToastItem(
    toast: Toast,
    #[prop(into)] on_close: Callback<()>,
) -> impl IntoView {
    let (bg_class, border_class, text_class) = match toast.toast_type {
        ToastType::Success => (
            "bg-green-50",
            "border-green-100",
            "text-green-700",
        ),
        ToastType::Error => (
            "bg-red-50", 
            "border-red-100",
            "text-red-700",
        ),
        ToastType::Info => (
            "bg-blue-50",
            "border-blue-100", 
            "text-blue-700",
        ),
    };

    let icon = match toast.toast_type {
        ToastType::Success => "✓",
        ToastType::Error => "✕",
        ToastType::Info => "ℹ",
    };

    view! {
        <div class=format!(
            "flex items-start space-x-3 p-4 rounded-lg border shadow-sm transition-all duration-300 ease-in-out {} {} {}",
            bg_class,
            border_class,
            text_class,
        )>
            <div class="flex-shrink-0 text-sm font-medium mt-0.5">{icon}</div>
            <div class="flex-1 text-sm">{toast.message}</div>
            <button
                class="flex-shrink-0 text-xs opacity-60 hover:opacity-100 transition-opacity ml-2"
                on:click=move |_| on_close.run(())
            >
                "✕"
            </button>
        </div>
    }
}

pub fn use_toast() -> ToastContext {
    use_context::<ToastContext>()
        .expect("ToastContext must be provided. Make sure to wrap your app with ToastProvider.")
} 