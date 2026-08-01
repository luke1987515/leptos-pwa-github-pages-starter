use leptos::*;
use leptos_router::A;

#[component]
pub fn NotFound() -> impl IntoView {
    // Resolve base path dynamically to ensure back to home link works correctly
    let base_path = move || {
        let pathname = window().location().pathname().unwrap_or_default();
        if pathname.starts_with("/leptos-pwa-github-pages-starter") {
            "/leptos-pwa-github-pages-starter"
        } else {
            ""
        }
    };

    view! {
        <div class="notfound-container">
            <div class="notfound-title">"404"</div>
            <h2 style="font-size: 2rem; font-weight: 700; margin-bottom: 1rem;">"Oops! 迷失在虛空中"</h2>
            <p style="color: hsl(var(--text-secondary)); margin-bottom: 2rem; max-width: 450px;">
                "您嘗試存取的頁面不存在，或者已經移到其他的宇宙去了。"
            </p>
            <A href=move || format!("{}/", base_path()) class="btn">
                <svg style="width:18px;height:18px" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <line x1="19" y1="12" x2="5" y2="12" />
                    <polyline points="12 19 5 12 12 5" />
                </svg>
                "返回首頁"
            </A>
        </div>
    }
}
