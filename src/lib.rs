use std::{cell::RefCell, rc::Rc};

use input::get_num_points_from_html;
use nalgebra_glm::vec3;
use vertex_buffer::VertexData;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext;

mod input;
mod matrix;
mod mouse;
mod octree;
mod render;
mod shaders;
mod vertex_buffer;
mod webgl_utils;

use matrix::MVMatrixValues;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let (gl, program, mouse_state, mv_matrix_values) = setup()?;
    let num_points = get_num_points_from_html()?;
    let vertex_data = vertex_buffer::create_vertex_buffers(&gl, num_points as u32)?;
    let scale_factor = 1.0;

    register_num_points_listener(
        gl.clone(),
        program.clone(),
        mouse_state.clone(),
        mv_matrix_values.clone(),
    )?;
    render::start_render_loop(
        gl.clone(),
        program.clone(),
        vertex_data.clone(),
        scale_factor,
        mouse_state,
        mv_matrix_values,
        num_points as u32,
    );
    Ok(())
}

fn setup() -> Result<
    (
        WebGlRenderingContext,
        web_sys::WebGlProgram,
        Rc<RefCell<mouse::MouseState>>,
        Rc<RefCell<MVMatrixValues>>,
    ),
    JsValue,
> {
    let window = web_sys::window().ok_or("No global `window` exists")?;
    let document = window
        .document()
        .ok_or("Should have a document on window")?;
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Can't find canvas element")?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| JsValue::from_str("Could not convert canvas to HtmlCanvasElement"))
        .unwrap();
    let gl = webgl_utils::get_webgl_context(&canvas)?;
    let program = shaders::create_shader_program(&gl)?;
    let mouse_state = mouse::create_mouse_state(&canvas)?;

    let eye = vec3(0.0, 0.0, 5.0);
    let target = vec3(0.0, 0.0, 0.0);
    let up = vec3(0.0, 1.0, 0.0);
    let mv_matrix_values = Rc::new(RefCell::new(MVMatrixValues { eye, target, up }));

    Ok((gl, program, mouse_state, mv_matrix_values))
}

fn register_num_points_listener(
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
        .ok_or("Can't find num-points input element")?
        .dyn_into::<web_sys::HtmlInputElement>() // Ensure it's an HtmlInputElement
        .map_err(|_| JsValue::from_str("Could not convert element to HtmlInputElement"))?;

    let num_points_input_clone = num_points_input.clone();
    let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
        let num_points = num_points_input_clone.value().parse().unwrap_or(1000);
        update_num_points(
            &gl.clone(),
            &program.clone(),
            mouse_state.clone(),
            mv_matrix_values.clone(),
            num_points,
        );
    }) as Box<dyn FnMut(_)>);

    num_points_input.set_oninput(Some(closure.as_ref().unchecked_ref())); // This should now work
    closure.forget();

    Ok(())
}

fn update_num_points(
    gl: &WebGlRenderingContext,
    program: &web_sys::WebGlProgram,
    mouse_state: Rc<RefCell<mouse::MouseState>>,
    mv_matrix_values: Rc<RefCell<MVMatrixValues>>,
    num_points: i32,
) {
    let vertex_data = vertex_buffer::create_vertex_buffers(gl, num_points as u32).unwrap();

    let scale_factor = 1.0;
    render::start_render_loop(
        gl.clone(),
        program.clone(),
        vertex_data,
        scale_factor,
        mouse_state,
        mv_matrix_values,
        num_points as u32,
    );
}
