use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, MouseEvent};

pub struct MouseState {
    pub is_dragging: bool,
    pub last_x: f32,
    pub last_y: f32,
    pub rotation_x: f32,
    pub rotation_y: f32,
}

fn mouse_down_handler(event: MouseEvent, mouse_state: &mut MouseState) {
    mouse_state.is_dragging = true;
    mouse_state.last_x = event.client_x() as f32;
    mouse_state.last_y = event.client_y() as f32;
}

fn mouse_move_handler(event: MouseEvent, mouse_state: &mut MouseState) {
    if mouse_state.is_dragging {
        let delta_x = event.client_x() as f32 - mouse_state.last_x;
        let delta_y = event.client_y() as f32 - mouse_state.last_y;
        mouse_state.last_x = event.client_x() as f32;
        mouse_state.last_y = event.client_y() as f32;
        mouse_state.rotation_x += delta_y * 0.01;
        mouse_state.rotation_y += delta_x * 0.01;
    }
}

fn mouse_up_handler(event: MouseEvent, mouse_state: &mut MouseState) {
    mouse_state.is_dragging = false;
}

fn add_mouse_down_listener(
    canvas: &HtmlCanvasElement,
    mouse_state: Rc<RefCell<MouseState>>,
) -> Result<(), JsValue> {
    let mouse_state_clone = mouse_state.clone();
    let mouse_down_handler = Closure::wrap(Box::new(move |event: MouseEvent| {
        let mut mouse_state = mouse_state_clone.borrow_mut();
        mouse_down_handler(event, &mut mouse_state);
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback(
        "mousedown",
        mouse_down_handler.as_ref().unchecked_ref(),
    )?;
    mouse_down_handler.forget();
    Ok(())
}

fn add_mouse_move_listener(
    canvas: &HtmlCanvasElement,
    mouse_state: Rc<RefCell<MouseState>>,
) -> Result<(), JsValue> {
    let mouse_state_clone = mouse_state.clone();
    let mouse_move_handler = Closure::wrap(Box::new(move |event: MouseEvent| {
        let mut mouse_state = mouse_state_clone.borrow_mut();
        mouse_move_handler(event, &mut mouse_state);
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback(
        "mousemove",
        mouse_move_handler.as_ref().unchecked_ref(),
    )?;
    mouse_move_handler.forget();
    Ok(())
}

fn add_mouse_up_listener(
    canvas: &HtmlCanvasElement,
    mouse_state: Rc<RefCell<MouseState>>,
) -> Result<(), JsValue> {
    let mouse_state_clone = mouse_state.clone();
    let mouse_up_handler = Closure::wrap(Box::new(move |event: MouseEvent| {
        let mut mouse_state = mouse_state_clone.borrow_mut();
        mouse_up_handler(event, &mut mouse_state);
    }) as Box<dyn FnMut(_)>);
    canvas
        .add_event_listener_with_callback("mouseup", mouse_up_handler.as_ref().unchecked_ref())?;
    mouse_up_handler.forget();
    Ok(())
}

pub fn create_mouse_state(canvas: &HtmlCanvasElement) -> Result<Rc<RefCell<MouseState>>, JsValue> {
    let mouse_state = Rc::new(RefCell::new(MouseState {
        is_dragging: false,
        last_x: 0.0,
        last_y: 0.0,
        rotation_x: 0.0,
        rotation_y: 0.0,
    }));

    add_mouse_down_listener(canvas, mouse_state.clone())?;
    add_mouse_move_listener(canvas, mouse_state.clone())?;
    add_mouse_up_listener(canvas, mouse_state.clone())?;

    Ok(mouse_state)
}
