use web_sys::CanvasRenderingContext2d as context;

mod screen1;
mod screen2;
mod screen3;
mod common;

use std::f64::consts::PI;

pub fn draw_frame(contexts: &[context], frame: u32, dt: f64) -> Result<(), String> {
    if contexts.len() < 3 {
        return Err("Not enough contexts provided, expected 3".into());
    }

    // duration of period in milliseconds
    let speed1: u32 = 4000;
    let speed2: u32 = 5000;
    
    let input1 = (((dt / speed1 as f64) % 1.0) * 2.0 * PI).sin() / 2.0 + 0.5;
    let input2 = (((dt / speed2 as f64) % 1.0) * 2.0 * PI).sin() / 2.0 + 0.5;

    for i in 0..3 {
        let context = &contexts[i];
        let result = match i {
            0 => screen1::draw_frame(context, input1, input2),
            1 => screen2::draw_frame(context, input1, input2),
            2 => screen3::draw_frame(context, input1, input2),
            _ => return Err("Invalid context index".into()),
        };
        if let Err(e) = result {
            return Err(format!("Error drawing frame for context {}: {:?}", i, e));
        }
    }

    Ok(())
}