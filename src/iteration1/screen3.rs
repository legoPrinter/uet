use web_sys::CanvasRenderingContext2d as context;
use super::common::*;

pub fn draw_frame(ctx: &context, input1: f64, input2: f64) -> Result<(), wasm_bindgen::JsValue> {
    white_bg(ctx);
    ctx.set_fill_style_str(&format!("rgb({}, 255, 0)", input1 * input2 * 255.0));
    ctx.fill_rect(0.0, 0.0, 200.0, 200.0);
    Ok(())
}