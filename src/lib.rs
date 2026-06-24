// Importing the wasm_bindgen crate. Used for Rust - Javascript interaction in a WebAssembly context.
use wasm_bindgen::prelude::*;
use web_sys::js_sys::{Function};

mod iteration1;

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

    log("Getting HTML elements with class 'screen-canvas'");
    let canvas_collection = document.get_elements_by_class_name("screen-canvas");
    let mut i = 0;
    let mut canvas_elements = Vec::new();
    while let Some(canvas) = canvas_collection.item(i) {
        canvas_elements.push(canvas);
        i += 1;
    }
    log(&format!("Found {} elements", canvas_elements.len()));

    /*
    const AMOUNT_SCREENS: usize = 3;
    if canvas_elements.len() < AMOUNT_SCREENS {
        log("Not enough canvas elements found, exiting program");
        return;
    } else if canvas_elements.len() > AMOUNT_SCREENS {
        log("Getting the first 3 canvas elements");
        let _ = canvas_elements.split_off(3);
    }
    */

    log("Conerting to canvas objects");
    let mut canvas_objects: Vec<web_sys::HtmlCanvasElement> = Vec::new();
    for canvas in canvas_elements { // canvas elements is purposefully moved as it is not needed after this point
        let canvas = match canvas.dyn_into::<web_sys::HtmlCanvasElement>() {
            Ok(canvas) => canvas,
            Err(_) => {
                log("Failed to convert canvas to HtmlCanvasElement, exiting program");
                return;
            }
        };
        canvas_objects.push(canvas);
    };

    log("Getting canvas context");
    let mut ctxs = Vec::new();
    for canvas in canvas_objects { // canvas objects is purposefully moved
        let ctx = match canvas.get_context("2d") {
            Ok(Some(ctx)) => ctx,
            _ => {
                log("Failed to get canvas context, exiting program");
                return;
            }
        };
        ctxs.push(ctx);
    }

    log("Converting to context object");
    let mut ctx_objects = Vec::new();
    for ctx in ctxs { // ctxs is purposefully moved
        let ctx = match ctx.dyn_into::<web_sys::CanvasRenderingContext2d>() {
            Ok(ctx) => ctx,
            Err(_) => {
                log("Failed to convert canvas to 2D context, exiting program");
                return;
            }
        };
        ctx_objects.push(ctx);
    }

    // frame frequency in Hz
    let fps: u32 = 30;

    let mut frame: u32 = 0;

    // create the callback and interval
    log("Starting draw loop...");
    let cb: ScopedClosure<'static, dyn FnMut() -> ()> = Closure::new(move || {
        frame += 1;
        let dt = frame as f64 / fps as f64 * 1000.0; // delta time in milliseconds

        let status = iteration1::draw_frame(&ctx_objects, frame, dt);

        if let Err(e) = status {
            log(&format!("Error drawing frame: {}", e));
        }
    });
    window.set_interval_with_callback_and_timeout_and_arguments_0(&Function::from_closure(cb), 1000 / fps as i32).unwrap();
}