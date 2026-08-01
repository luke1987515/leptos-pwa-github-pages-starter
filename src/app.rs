use leptos::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use js_sys;

use crate::home::Home;
use crate::not_found::NotFound;
use crate::pwa_install::PwaInstallButton;

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
    window_event_listener(ev::Custom::new("beforeinstallprompt"), move |e| {
        let event: web_sys::Event = e.unchecked_into();
        event.prevent_default(); // Prevent default browser prompt
        let prompt_event: BeforeInstallPromptEvent = event.unchecked_into();
        set_install_prompt.set(Some(prompt_event));
    });

    // 3. Service Worker Update Signal
    let (update_available, set_update_available) = create_signal(false);
    
    // Listen for custom SW update event dispatched by index.html script
    window_event_listener(ev::Custom::new("sw-update-available"), move |_| {
        set_update_available.set(true);
    });

    // 4. Resolve Base Path dynamically (localhost vs GitHub Pages repo name)
    let base_path = move || {
        let pathname = window().location().pathname().unwrap_or_default();
        if pathname.starts_with("/leptos-pwa-github-pages-starter") {
            "/leptos-pwa-github-pages-starter"
        } else {
            ""
        }
    };

    view! {
        <div class="app-container">
            // Header Navbar
            <header class="navbar">
                <a href=move || format!("{}/", base_path()) class="brand-link">
                    <span style="font-weight: 800;">⚡ Leptos</span>
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
                <Router>
                    <Routes>
                        <Route path=move || format!("{}/", base_path()) view=Home />
                        <Route path="*" view=NotFound />
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

            // Service Worker Update notification banner (Glassmorphism toast)
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
                                // Trigger controller reload by notifying Service Worker
                                if let Ok(sw_container) = window().navigator().service_worker() {
                                    let ready_promise = sw_container.ready();
                                    spawn_local(async move {
                                        match JsFuture::from(ready_promise).await {
                                            Ok(reg) => {
                                                let registration = reg.unchecked_into::<web_sys::ServiceWorkerRegistration>();
                                                if let Some(waiting) = registration.waiting() {
                                                    let msg = js_sys::Object::new();
                                                    let _ = js_sys::Reflect::set(&msg, &"type".into(), &"SKIP_WAITING".into());
                                                    let _ = waiting.post_message(&msg);
                                                }
                                            }
                                            Err(err) => {
                                                leptos::logging::log!("Failed to get SW registration ready: {:?}", err);
                                            }
                                        }
                                    });
                                } else {
                                    // Service Worker unavailable in this environment
                                    leptos::logging::log!("ServiceWorker unavailable or navigator.service_worker() returned Err.");
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
