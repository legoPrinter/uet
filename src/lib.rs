// Importing the wasm_bindgen crate. Used for Rust - Javascript interaction in a WebAssembly context.
use wasm_bindgen::prelude::*;
use web_sys::js_sys::{Function};

mod screen;

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

    const AMOUNT_SCREENS: usize = 3;
    if canvas_elements.len() < AMOUNT_SCREENS {
        log("Not enough canvas elements found, exiting program");
        return;
    } else if canvas_elements.len() > AMOUNT_SCREENS {
        log("Getting the first 3 canvas elements");
        let _ = canvas_elements.split_off(3);
    }

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

    // define variables
    
    // duration of period in milliseconds
    let speed1: u32 = 4000;
    let speed2: u32 = 5000;

    // frame frequency in Hz
    let fps: u32 = 30;

    let mut frame: u32 = 0;

    // create the callback and interval
    let cb: ScopedClosure<'static, dyn FnMut() -> ()> = Closure::new(move || {
        frame += 1;
        let dt = frame as f64 / fps as f64 * 1000.0; // delta time in milliseconds

        let input1 = (((dt / speed1 as f64) % 1.0) * 2.0 * 3.14159).sin() / 2.0 + 0.5;
        let input2 = (((dt / speed2 as f64) % 1.0) * 2.0 * 3.14159).sin() / 2.0 + 0.5;

        screen::screen1(&ctx_objects[0], input1, input2);
        screen::screen2(&ctx_objects[1], input1, input2);
        screen::screen3(&ctx_objects[2], input1, input2);
    });
    window.set_interval_with_callback_and_timeout_and_arguments_0(&Function::from_closure(cb), 1000 / fps as i32).unwrap();
}