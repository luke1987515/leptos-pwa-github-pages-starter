use leptos::*;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use js_sys::Promise;

use crate::app::{InstallPromptContext, BeforeInstallPromptEvent};

#[component]
pub fn PwaInstallButton() -> impl IntoView {
    let install_context = use_context::<InstallPromptContext>();

    if install_context.is_none() {
        return view! {}.into_view();
    }

    let context = install_context.unwrap();
    let prompt = context.prompt;
    let set_prompt = context.set_prompt;

    let handle_install = move |_| {
        if let Some(event) = prompt.get_untracked() {
            // Trigger the native installation prompt dialog
            match event.prompt() {
                Ok(_) => {
                    leptos::logging::log!("PWA installation prompt shown to user.");
                    let promise: Promise = event.userChoice();
                    let set_prompt_clone = set_prompt;
                    // Use JsFuture + spawn_local to await the Promise and then clear the prompt
                    spawn_local(async move {
                        let _ = JsFuture::from(promise).await;
                        set_prompt_clone.set(None);
                    });
                }
                Err(err) => {
                    leptos::logging::log!("Failed to prompt PWA installation: {:?}", err);
                    set_prompt.set(None);
                }
            }
        }
    };

    view! {
        <Show
            when=move || prompt.get().is_some()
            fallback=|| view! {}
        >
            <div class="glass-card install-banner">
                <div class="install-text">
                    <h3>"安裝 Leptos PWA 至桌面或手機"</h3>
                    <p>"將此應用程式安裝至您的主畫面，即可享有極速載入與完整的離線存取功能。"</p>
                </div>
                <button class="btn" on:click=handle_install>
                    <svg style="width:20px;height:20px" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                        <polyline points="7 10 12 15 17 10" />
                        <line x1="12" y1="15" x2="12" y2="3" />
                    </svg>
                    "立刻安裝 PWA"
                </button>
            </div>
        </Show>
    }.into_view()
}
