use leptos::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};

use crate::home::Home;
use crate::not_found::NotFound;

// JS Bindings for the non-standard PWA installation event
#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    pub type BeforeInstallPromptEvent;

    #[wasm_bindgen(method, catch)]
    pub fn prompt(this: &BeforeInstallPromptEvent) -> Result<(), JsValue>;

    #[wasm_bindgen(method, getter)]
    pub fn userChoice(this: &BeforeInstallPromptEvent) -> js_sys::Promise;
}

#[derive(Copy, Clone)]
pub struct InstallPromptContext {
    pub prompt: ReadSignal<Option<BeforeInstallPromptEvent>>,
    pub set_prompt: WriteSignal<Option<BeforeInstallPromptEvent>>,
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct OnlineStatusContext(pub ReadSignal<bool>);

#[component]
pub fn App() -> impl IntoView {
    // 1. Online/Offline Status Signal
    let (is_online, set_online) = create_signal(
        window()
            .navigator()
            .on_line()
    );
    provide_context(OnlineStatusContext(is_online));

    // Register online/offline event listeners
    window_event_listener(ev::online, move |_| set_online.set(true));
    window_event_listener(ev::offline, move |_| set_online.set(false));

    // 2. PWA Installation Event Signal
    let (install_prompt, set_install_prompt) = create_signal::<Option<BeforeInstallPromptEvent>>(None);
    provide_context(InstallPromptContext {
        prompt: install_prompt,
        set_prompt: set_install_prompt,
    });

    // Capture beforeinstallprompt event
    window_event_listener(ev::Custom::new("beforeinstallprompt"), move |e: web_sys::Event| {
        e.prevent_default();
        let prompt_event: BeforeInstallPromptEvent = e.unchecked_into();
        set_install_prompt.set(Some(prompt_event));
    });

    // 3. Service Worker Update Signal
    let (update_available, set_update_available) = create_signal(false);
    
    // Listen for custom SW update event dispatched by index.html script
    window_event_listener(ev::Custom::new("sw-update-available"), move |_: web_sys::CustomEvent| {
    set_update_available.set(true);
	});

    // 4. Resolve Base Path dynamically
    let pathname = window().location().pathname().unwrap_or_default();
    let base_path = if pathname.starts_with("/leptos-pwa-github-pages-starter") {
        "/leptos-pwa-github-pages-starter"
    } else {
        ""
    };

    view! {
        <div class="app-container">
            // Header Navbar
            <header class="navbar">
                <a href=format!("{}/", base_path) class="brand-link">
                    <span style="font-weight: 800;">"⚡ Leptos"</span>
                    <span>"PWA"</span>
                </a>
                <div class="nav-links">
                    <span class=move || {
                        if is_online.get() { "network-badge online" } else { "network-badge offline" }
                    }>
                        <span class="badge-dot"></span>
                        {move || if is_online.get() { "連線中" } else { "離線模式" }}
                    </span>
                </div>
            </header>

            // Main View router
            <main class="main-content">
                <Router base="/leptos-pwa-github-pages-starter">
					<Routes>
						<Route path="" view=Home />
						<Route path="/*any" view=NotFound />
					</Routes>
				</Router>
            </main>

            // Elegant Footer
            <footer class="footer">
                <p>
                    "Leptos PWA on GitHub Pages Starter © 2026 • Powered by "
                    <a href="https://leptos.dev/" target="_blank">"Rust & Leptos"</a>
                </p>
            </footer>

            // Service Worker Update notification banner
            <Show
                when=move || update_available.get()
                fallback=|| view! {}
            >
                <div class="update-toast">
                    <div style="font-weight: 700; font-size: 1.1rem; color: #fff;">
                        "✨ 應用程式有新版本！"
                    </div>
                    <div style="font-size: 0.9rem; color: #cbd5e1;">
                        "新版本已載入完成，點擊下方按鈕以套用更新並重新載入。"
                    </div>
                    <div style="display: flex; gap: 0.5rem; justify-content: flex-end;">
                        <button
                            class="btn"
                            style="padding: 0.5rem 1rem; font-size: 0.9rem;"
                            on:click=move |_| {
                                let sw_container = window().navigator().service_worker();
                                if let Ok(ready_promise) = sw_container.ready() {
                                    spawn_local(async move {
                                        if let Ok(reg_val) = JsFuture::from(ready_promise).await {
                                            let reg_js_val: JsValue = reg_val;
                                            let registration: web_sys::ServiceWorkerRegistration = reg_js_val.unchecked_into();
                                            if let Some(waiting) = registration.waiting() {
                                                let msg = js_sys::Object::new();
                                                let key: JsValue = "type".into();
                                                let val: JsValue = "SKIP_WAITING".into();
                                                let _ = js_sys::Reflect::set(&msg, &key, &val);
                                                let msg_val: &JsValue = msg.as_ref();
                                                let _ = waiting.post_message(msg_val);
                                            }
                                        }
                                    });
                                }
                            }
                        >
                            "立即更新"
                        </button>
                    </div>
                </div>
            </Show>
        </div>
    }
}