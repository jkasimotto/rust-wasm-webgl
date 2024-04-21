use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::WebGlRenderingContext;

use crate::{matrix::MVMatrixValues, mouse};

pub fn register_num_points_listener(
    gl: WebGlRenderingContext,
    program: web_sys::WebGlProgram,
    mouse_state: Rc<RefCell<mouse::MouseState>>,
    mv_matrix_values: Rc<RefCell<MVMatrixValues>>,
) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No global `window` exists")?;
    let document = window
        .document()
        .ok_or("Should have a document on window")?;
    let num_points_input = document
        .get_element_by_id("num-points")
        .ok_or("Can't find num-points input element")?;
    let num_points_input = num_points_input.dyn_into::<web_sys::HtmlInputElement>()?;

    let gl_clone = gl.clone();
    let program_clone = program.clone();
    let mouse_state_clone = mouse_state.clone();
    let mv_matrix_values_clone = mv_matrix_values.clone();
    let num_points_input_clone = num_points_input.clone();
    let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
        let num_points = num_points_input_clone.value().parse().unwrap_or(1000);
        // update_num_points(
        //     &gl_clone,
        //     &program_clone,
        //     mouse_state_clone.clone(),
        //     mv_matrix_values_clone.clone(),
        //     num_points,
        // );
    }) as Box<dyn FnMut(_)>);

    num_points_input.set_oninput(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    Ok(())
}

pub fn get_num_points_from_html() -> Result<i32, JsValue> {
    let window = web_sys::window().ok_or("No global `window` exists")?;
    let document = window
        .document()
        .ok_or("Should have a document on window")?;
    let num_points_input = document
        .get_element_by_id("num-points")
        .ok_or("Can't find num-points input element")?;
    let num_points_str = num_points_input
        .dyn_into::<web_sys::HtmlInputElement>()?
        .value();
    num_points_str
        .parse()
        .map_err(|_| JsValue::from_str("Invalid number of points"))
}
pub fn add_slider_event_listener(slider_handler: Closure<dyn FnMut(web_sys::Event)>) {
    let window = web_sys::window().expect("No global window exists");
    let document = window.document().expect("Should have a document on window");
    let slider_ids = [
        "eye-x", "eye-y", "eye-z", "target-x", "target-y", "target-z", "up-x", "up-y", "up-z",
    ];
    for slider_id in slider_ids.iter() {
        let slider = document
            .query_selector(&format!("input[type=range][id={}]", slider_id))
            .unwrap()
            .unwrap();
        slider
            .add_event_listener_with_callback("input", slider_handler.as_ref().unchecked_ref())
            .unwrap();
    }
    slider_handler.forget();
}

pub fn create_wheel_handler(
    scale_factor_ref: Rc<RefCell<f32>>,
) -> Closure<dyn FnMut(web_sys::WheelEvent)> {
    Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
        let mut scale_factor = scale_factor_ref.borrow_mut();
        let delta = event.delta_y() as f32;
        *scale_factor += delta * 0.001;
    }) as Box<dyn FnMut(_)>)
}

pub fn add_wheel_event_listener(wheel_handler: Closure<dyn FnMut(web_sys::WheelEvent)>) {
    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback("wheel", wheel_handler.as_ref().unchecked_ref())
        .unwrap();
    wheel_handler.forget();
}

pub fn create_slider_handler(
    mv_matrix_values: Rc<RefCell<MVMatrixValues>>,
) -> Closure<dyn FnMut(web_sys::Event)> {
    Closure::wrap(Box::new(move |event: web_sys::Event| {
        let target = event.target().unwrap();
        let input = target.dyn_ref::<web_sys::HtmlInputElement>().unwrap();
        let value = input.value().parse::<f32>().unwrap();

        let mut mv_matrix_values = mv_matrix_values.borrow_mut();
        match input.id().as_str() {
            "eye-x" => mv_matrix_values.eye.x = value,
            "eye-y" => mv_matrix_values.eye.y = value,
            "eye-z" => mv_matrix_values.eye.z = value,
            "target-x" => mv_matrix_values.target.x = value,
            "target-y" => mv_matrix_values.target.y = value,
            "target-z" => mv_matrix_values.target.z = value,
            "up-x" => mv_matrix_values.up.x = value,
            "up-y" => mv_matrix_values.up.y = value,
            "up-z" => mv_matrix_values.up.z = value,
            _ => {}
        }
    }) as Box<dyn FnMut(_)>)
}
