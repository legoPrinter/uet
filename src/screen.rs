use web_sys::CanvasRenderingContext2d as context;

pub fn screen1(ctx: &context, input1: f64, input2: f64) {
    white_bg(ctx);
    ctx.set_fill_style_str("blue");
    ctx.fill_rect(0.0, 0.0, 200.0 * input1, 200.0 * input2);
}

pub fn screen2(ctx: &context, input1: f64, input2: f64) {
    white_bg(ctx);
    ctx.set_fill_style_str(&format!("rgb(0, 255, {})", input1 * input2 * 255.0));
    ctx.fill_rect(0.0, 0.0, 200.0, 200.0);
}

pub fn screen3(ctx: &context, input1: f64, input2: f64) {
    white_bg(ctx);
    ctx.set_fill_style_str(&format!("rgb({}, 255, 0)", input1 * input2 * 255.0));
    ctx.fill_rect(0.0, 0.0, 200.0, 200.0);
}

fn white_bg(ctx: &context) {
    ctx.set_fill_style_str("white");
    ctx.fill_rect(0.0, 0.0, 200.0, 200.0);
}