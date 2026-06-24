use web_sys::CanvasRenderingContext2d as context;

pub fn screen1(ctx: &context, input1: f64, input2: f64) {
    ctx.set_fill_style_str("white");
    ctx.fill_rect(0.0, 0.0, 200.0, 200.0);
    ctx.set_fill_style_str("blue");
    ctx.fill_rect(0.0, 0.0, 200.0 * input1, 200.0 * input2);
}

pub fn _screen2(_ctx: &context, _input1: f64, _input2: f64) {

}

pub fn _screen3(_ctx: &context, _input1: f64, _input2: f64) {

}