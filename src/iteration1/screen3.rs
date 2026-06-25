use web_sys::CanvasRenderingContext2d as context;
use super::common::*;

pub fn draw_frame(ctx: &context, input1: f64, input2: f64) -> Result<(), wasm_bindgen::JsValue> {
    white_bg(ctx);
    
    let radius = 15.0;
    let padding = radius;
    
    let range = 200.0 - 2.0 * padding;

    let x = padding + input1 * range;
    let y = padding + input2 * range;

    white_bg(ctx);
    ctx.set_fill_style_str("black");
    ctx.begin_path();
    ctx.arc(x, y, radius, 0.0, 2.0 * PI)?;
    ctx.fill();

    Ok(())
}