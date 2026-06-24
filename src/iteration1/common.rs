use web_sys::CanvasRenderingContext2d as context;
pub use std::f64::consts::PI;

pub fn white_bg(ctx: &context) {
    ctx.set_fill_style_str("white");
    ctx.fill_rect(0.0, 0.0, 200.0, 200.0);
}