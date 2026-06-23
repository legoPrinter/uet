// Importing the wasm_bindgen crate. Used for Rust - Javascript interaction in a WebAssembly context.
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn message(length: u32) -> String {
    let chars = "abcdefghijklmnopqrstuvwxyz";
    let mut result = String::new();
    let mut i: usize = length as usize % 100;
    for _ in 0..length-1 {
        i = (i * 2) - 1;
        if i > 10000 {i = length as usize % 100}
        result.push(chars.chars().nth(i % chars.len()).unwrap());
    }
    result.push('!');
    return result;
}