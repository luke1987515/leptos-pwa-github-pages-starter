use leptos::*;
use wasm_bindgen::JsCast;

use crate::pwa_install::PwaInstallButton;

#[component]
pub fn Home() -> impl IntoView {
    let (max_iterations, set_max_iterations) = create_signal(300);
    let (elapsed_time, set_elapsed_time) = create_signal(0.0);
    let (calculated_pixels, set_calculated_pixels) = create_signal(0);
    
    let canvas_ref = create_node_ref::<html::Canvas>();

    // Mandelbrot render function
    let render_mandelbrot = move |iter: i32| {
        if let Some(canvas) = canvas_ref.get() {
            let ctx = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .unchecked_into::<web_sys::CanvasRenderingContext2d>();

            let width = canvas.width() as usize;
            let height = canvas.height() as usize;
            
            // RGBA pixel buffer
            let mut pixels = vec![0u8; width * height * 4];

            // Perform high-density calculation in Rust
            let start = js_sys::Date::now();
            let mut total_calculations = 0;

            for y in 0..height {
                for x in 0..width {
                    // Map screen coordinates to Mandelbrot complex plane
                    let cx = -2.0 + (x as f64 / width as f64) * 3.0;
                    let cy = -1.5 + (y as f64 / height as f64) * 3.0;

                    let mut zx = 0.0;
                    let mut zy = 0.0;
                    let mut i = 0;

                    while zx * zx + zy * zy <= 4.0 && i < iter {
                        let temp = zx * zx - zy * zy + cx;
                        zy = 2.0 * zx * zy + cy;
                        zx = temp;
                        i += 1;
                    }
                    total_calculations += i;

                    let idx = (y * width + x) * 4;
                    if i == iter {
                        // Interior color (deep navy/slate)
                        pixels[idx] = 15;
                        pixels[idx + 1] = 23;
                        pixels[idx + 2] = 42;
                        pixels[idx + 3] = 255;
                    } else {
                        // Exterior neon gradient based on escape iterations
                        let ratio = i as f64 / iter as f64;
                        // Interpolate between Neon Purple (124, 58, 237) and Neon Cyan (6, 182, 212)
                        pixels[idx] = (124.0 * (1.0 - ratio) + 6.0 * ratio) as u8;     // R
                        pixels[idx + 1] = (58.0 * (1.0 - ratio) + 182.0 * ratio) as u8; // G
                        pixels[idx + 2] = (237.0 * (1.0 - ratio) + 212.0 * ratio) as u8;// B
                        pixels[idx + 3] = 255; // A
                    }
                }
            }

            let end = js_sys::Date::now();
            set_elapsed_time.set(end - start);
            set_calculated_pixels.set(total_calculations);

            // Put image data back to Canvas
            let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
                wasm_bindgen::Clamped(&mut pixels),
                canvas.width(),
                canvas.height()
            ).unwrap();
            
            ctx.put_image_data(&image_data, 0.0, 0.0).unwrap();
        }
    };

    // Auto-render when canvas is ready or when max_iterations changes
    create_effect(move |_| {
        let iter = max_iterations.get();
        render_mandelbrot(iter);
    });

    view! {
        <div>
            // Hero section
            <section class="hero-section">
                <h1 class="hero-title">
                    "極速 WASM 驅動的" <br />
                    <span>"離線優先 Web 應用"</span>
                </h1>
                <p class="hero-subtitle">
                    "利用 Rust + Leptos CSR 建構，搭配 Workbox 進行完全的離線預快取。體驗媲美原生應用的極致流暢感。"
                </p>
            </section>

            // Conditional PWA Install Button
            <PwaInstallButton />

            // Features Grid
            <div class="features-grid">
                <div class="glass-card">
                    <span class="feature-icon">"🦀"</span>
                    <h3 class="feature-title">"Rust & WASM"</h3>
                    <p class="feature-desc">
                        "強大的型別系統與編譯期安全保證。編譯成 WebAssembly，為瀏覽器端帶來接近本機端的速度與效率。"
                    </p>
                </div>
                <div class="glass-card">
                    <span class="feature-icon">"🔌"</span>
                    <h3 class="feature-title">"完全離線優先"</h3>
                    <p class="feature-desc">
                        "整合 Service Worker 與 Workbox CLI 預快取機制。即使在無網路訊號環境，仍能秒速載入並正常操作。"
                    </p>
                </div>
                <div class="glass-card">
                    <span class="feature-icon">"✨"</span>
                    <h3 class="feature-title">"毛玻璃視覺美學"</h3>
                    <p class="feature-desc">
                        "採用精心調配的暗系色調與高質感 HSL 漸層。搭配微互動與平滑 CSS 動畫，打造無懈可擊的互動體驗。"
                    </p>
                </div>
            </div>

            // High Performance WASM Benchmark Section
            <div class="glass-card">
                <div class="benchmark-container">
                    <div class="benchmark-header">
                        <h2 style="font-size: 1.8rem; font-weight: 800; margin-bottom: 0.5rem; background: linear-gradient(135deg, #06b6d4, #7c3aed); -webkit-background-clip: text; -webkit-text-fill-color: transparent;">
                            "🦀 WASM 高密度運算基準測試"
                        </h2>
                        <p style="color: hsl(var(--text-secondary)); font-size: 0.95rem;">
                            "此測試使用 Rust 直接在您的瀏覽器端渲染 Mandelbrot (曼德博) 碎形圖。拉動下方的滑桿調整最大疊代次數，體驗 WebAssembly 即時計算的驚人效能！"
                        </p>
                    </div>

                    <div class="benchmark-grid">
                        <!-- Left Panel: Canvas -->
                        <div class="canvas-container">
                            <canvas
                                node_ref=canvas_ref
                                width="256"
                                height="256"
                                class="benchmark-canvas"
                            ></canvas>
                        </div>

                        <!-- Right Panel: Controls & Stats -->
                        <div class="controls-panel">
                            <div class="slider-group">
                                <label for="iterations">
                                    <span>"最大迭代次數 (Iterations)"</span>
                                    <span style="color: #06b6d4; font-weight: 700;">{move || max_iterations.get()}</span>
                                </label>
                                <input
                                    type="range"
                                    id="iterations"
                                    min="50"
                                    max="1000"
                                    step="50"
                                    value=move || max_iterations.get()
                                    on:input=move |e| {
                                        if let Ok(val) = event_target_value(&e).parse::<i32>() {
                                            set_max_iterations.set(val);
                                        }
                                    }
                                    class="elegant-range"
                                />
                            </div>

                            <div class="stats-card">
                                <div class="stat-item">
                                    <span class="stat-val">{move || format!("{:.1}", elapsed_time.get())} " ms"</span>
                                    <span class="stat-label">"運算耗時"</span>
                                </div>
                                <div class="stat-item">
                                    <span class="stat-val">
                                        {move || {
                                            let count = calculated_pixels.get();
                                            if count >= 1_000_000 {
                                                format!("{:.1}M", count as f64 / 1_000_000.0)
                                            } else {
                                                format!("{:.1}K", count as f64 / 1000.0)
                                            }
                                        }}
                                    </span>
                                    <span class="stat-label">"總迴圈計算次數"</span>
                                </div>
                            </div>

                            <div style="display: flex; gap: 1rem;">
                                <button
                                    class="btn"
                                    style="flex: 1;"
                                    on:click=move |_| render_mandelbrot(max_iterations.get())
                                >
                                    <svg style="width:18px;height:18px" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M21.5 2v6h-6M21.34 15.57a10 10 0 1 1-.57-8.38l5.67-5.67"/>
                                    </svg>
                                    "重新計算"
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
