// Importing the wasm_bindgen crate. Used for Rust - Javascript interaction in a WebAssembly context.
use wasm_bindgen::prelude::*;

const WIDTH: u32 = 200;
const HEIGHT: u32 = 200;

#[wasm_bindgen]
pub fn render_frame() -> Vec<u8> {
    let mut buffer = [0u8; (WIDTH * HEIGHT * 4) as usize];
    let color = [255, 0, 255, 255];
    for (i, pixel) in buffer.iter_mut().enumerate() {
        *pixel = color[i % 4];
    }
    buffer.to_vec()
}