// Importing the wasm_bindgen crate. Used for Rust - Javascript interaction in a WebAssembly context.
use wasm_bindgen::prelude::*;

fn log(message: &str) {
    web_sys::console::log_1(&message.into());
}

#[wasm_bindgen]
pub fn start_rust_program() {
    log("Getting window");
    let window = match web_sys::window() {
        Some(window) => window,
        None => {
            log("Failed to get window, exiting program");
            return;
        }
    };
    log("Getting document");
    let document = match window.document() {
        Some(document) => document,
        None => {
            log("Failed to get document, exiting program");
            return;
        }
    };
    log("Getting canvas");
    let canvas = match document.query_selector("canvas") {
        Ok(canvas) => match canvas {
            Some(canvas) => canvas,
            None => {
                log("No canvas element found, exiting program");
                return;
            }
        },
        Err(_) => {
            log("Query selector returned an error, exiting program");
            return;
        }
    };
    log("Converting to canvas element");
    let canvas = match canvas.dyn_into::<web_sys::HtmlCanvasElement>() {
        Ok(canvas) => canvas,
        Err(_) => {
            log("Failed to convert canvas to HtmlCanvasElement, exiting program");
            return;
        }
    };
    log("Getting canvas context");
    let ctx = match canvas.get_context("2d") {
        Ok(Some(ctx)) => ctx,
        _ => {
            log("Failed to get canvas context, exiting program");
            return;
        }
    };
    log("Converting to context object");
    let ctx = match ctx.dyn_into::<web_sys::CanvasRenderingContext2d>() {
        Ok(ctx) => ctx,
        Err(_) => {
            log("Failed to convert canvas to 2D context, exiting program");
            return;
        }
    };
    log("Drawing something on canvas");
    ctx.set_fill_style_str("blue");
    ctx.fill_rect(10.0, 10.0, 100.0, 100.0);
}