use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlInputElement, Event};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = Rc::new(canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?);

    let slider = Rc::new(document
        .get_element_by_id("radius-slider")
        .unwrap()
        .dyn_into::<HtmlInputElement>()?);

    let context_clone = context.clone();
    let slider_clone = slider.clone();

    let closure = Closure::wrap(Box::new(move |_: Event| {
        let radius = slider_clone.value().parse::<f64>().unwrap();
        draw_circle(&context_clone, radius);
    }) as Box<dyn FnMut(_)>);

    slider.set_oninput(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    draw_circle(&context, 50.0);

    Ok(())
}

fn draw_circle(context: &CanvasRenderingContext2d, radius: f64) {
    let canvas_width = context.canvas().unwrap().width() as f64;
    let canvas_height = context.canvas().unwrap().height() as f64;
    let center_x = canvas_width / 2.0;
    let center_y = canvas_height / 2.0;

    context.clear_rect(0.0, 0.0, canvas_width, canvas_height);
    context.begin_path();
    context
        .arc(center_x, center_y, radius, 0.0, 2.0 * std::f64::consts::PI)
        .unwrap();
    context.stroke();
}