use leptos::prelude::*;
use web_sys::wasm_bindgen::JsValue;
use web_sys::wasm_bindgen::JsCast;

#[component]
pub fn Mandelbrot() -> impl IntoView {
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();

    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let width = 600.0_f64;
            let height = 400.0_f64;
            canvas.set_width(width as u32);
            canvas.set_height(height as u32);

            let ctx = canvas
                .get_context("2d")
                .ok()
                .and_then(|v| v)
                .and_then(|v| v.dyn_into::<web_sys::CanvasRenderingContext2d>().ok());

            if let Some(ctx) = ctx {
                for y in 0..(height as i32) {
                    for x in 0..(width as i32) {
                        let cx = (x as f64 / width) * 3.5 - 2.5;
                        let cy = (y as f64 / height) * 2.0 - 1.0;

                        let mut zx = 0.0;
                        let mut zy = 0.0;
                        let mut iter = 0;
                        let max_iter = 80;

                        while zx * zx + zy * zy <= 4.0 && iter < max_iter {
                            let tmp = zx * zx - zy * zy + cx;
                            zy = 2.0 * zx * zy + cy;
                            zx = tmp;
                            iter += 1;
                        }

                        let (r, g, b) = if iter == max_iter { 
                            (0, 0, 0) 
                        } else { 
                            ((iter * 9) as u8, (iter * 7) as u8, (iter * 5) as u8) 
                        };
                        let color = format!("rgb({}, {}, {})", r, g, b);
                        ctx.set_fill_style_str(&color);
                        ctx.fill_rect(x as f64, y as f64, 1.0, 1.0);
                    }
                }
            }
        }
    });

    view! {
        <div class="bg-white rounded-lg shadow-md p-4">
            <h2 class="text-2xl font-bold mb-3">"Mandelbrot"</h2>
            <canvas id="mandelbrot-canvas" node_ref=canvas_ref class="w-full border rounded"></canvas>
        </div>
    }
}

