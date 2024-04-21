use std::{cell::RefCell, rc::Rc};

use input::get_num_points_from_html;
use nalgebra_glm::vec3;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

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
    let vertex_data_ref = Rc::new(RefCell::new(vertex_data));
    let scale_factor = 1.0;
    let gl_ref = Rc::new(gl);

    render::start_render_loop(
        gl_ref,
        program,
        vertex_data_ref,
        scale_factor,
        mouse_state,
        mv_matrix_values,
    );
    Ok(())
}

fn setup() -> Result<
    (
        WebGl2RenderingContext,
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
