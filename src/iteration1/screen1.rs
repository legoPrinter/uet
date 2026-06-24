use web_sys::CanvasRenderingContext2d as context;
use super::common::*;

pub fn draw_frame(ctx: &context, input1: f64, input2: f64) -> Result<(), wasm_bindgen::JsValue> {
    white_bg(ctx);
    
    let hole_radius = 50.0 * input1;
    let width = 50.0 * input2;

    ctx.set_stroke_style_str("black");
    ctx.set_line_width(width);

    ctx.begin_path();
    ctx.arc(100.0, 100.0, hole_radius + width/2.0, 0.0, 2.0 * PI)?;
    ctx.stroke();

    Ok(())
}