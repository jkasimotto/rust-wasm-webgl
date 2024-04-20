use std::{cell::RefCell, rc::Rc};

use nalgebra_glm::{vec3, Vec3};
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext;

mod matrix;
mod mouse;
mod octree;
mod render;
mod shaders;
mod vertex_buffer;
mod webgl_utils;

use matrix::MVMatrixValues;
use octree::Octree;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let canvas = setup_canvas()?;
    let gl = setup_webgl_context(&canvas)?;
    let program = setup_shader_program(&gl)?;
    let mut octree = create_octree();
    let num_points = get_num_points_from_html()?;
    let (buffer, octree_vertex_count) = setup_vertex_buffer(&gl, &mut octree, num_points)?;
    setup_vertex_attributes(&gl, &program);
    let mouse_state = setup_mouse_state(&canvas)?;
    let mv_matrix_values = create_mv_matrix_values();
    register_num_points_listener(gl.clone(), program.clone(), mouse_state.clone(), mv_matrix_values.clone())?;
    start_rendering(
        &gl,
        &program,
        mouse_state,
        mv_matrix_values,
        octree_vertex_count as u32,
        num_points,
    );
    Ok(())
}

fn setup_canvas() -> Result<web_sys::HtmlCanvasElement, JsValue> {
    let window = web_sys::window().ok_or("No global `window` exists")?;
    let document = window
        .document()
        .ok_or("Should have a document on window")?;
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Can't find canvas element")?;
    canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| JsValue::from_str("Could not convert canvas to HtmlCanvasElement"))
}

fn setup_webgl_context(
    canvas: &web_sys::HtmlCanvasElement,
) -> Result<WebGlRenderingContext, JsValue> {
    webgl_utils::get_webgl_context(canvas)
}

fn setup_shader_program(gl: &WebGlRenderingContext) -> Result<web_sys::WebGlProgram, JsValue> {
    shaders::create_shader_program(gl)
}

fn create_octree() -> Octree {
    Octree::new(Vec3::new(0.0, 0.0, 0.0), 2.0)
}

fn get_num_points_from_html() -> Result<i32, JsValue> {
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

fn setup_vertex_buffer(
    gl: &WebGlRenderingContext,
    octree: &mut Octree,
    num_points: i32,
) -> Result<(web_sys::WebGlBuffer, i32), JsValue> {
    vertex_buffer::create_vertex_buffer(gl, num_points as u32, octree)
}

fn setup_vertex_attributes(gl: &WebGlRenderingContext, program: &web_sys::WebGlProgram) {
    let position_attribute_location = gl.get_attrib_location(program, "position");
    let color_attribute_location = gl.get_attrib_location(program, "color");
    vertex_buffer::set_vertex_attribute_pointer(
        gl,
        position_attribute_location as u32,
        color_attribute_location as u32,
    );
}

fn setup_mouse_state(
    canvas: &web_sys::HtmlCanvasElement,
) -> Result<Rc<RefCell<mouse::MouseState>>, JsValue> {
    mouse::create_mouse_state(canvas)
}

fn create_mv_matrix_values() -> Rc<RefCell<MVMatrixValues>> {
    let eye = vec3(0.0, 0.0, 5.0);
    let target = vec3(0.0, 0.0, 0.0);
    let up = vec3(0.0, 1.0, 0.0);
    Rc::new(RefCell::new(MVMatrixValues { eye, target, up }))
}

fn start_rendering(
    gl: &WebGlRenderingContext,
    program: &web_sys::WebGlProgram,
    mouse_state: Rc<RefCell<mouse::MouseState>>,
    mv_matrix_values: Rc<RefCell<MVMatrixValues>>,
    octree_vertex_count: u32,
    num_points: i32,
) {
    let scale_factor = 1.0;
    render::start_render_loop(
        gl.clone(),
        program.clone(),
        scale_factor,
        mouse_state,
        mv_matrix_values,
        num_points as u32,
        octree_vertex_count,
    );
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
        .ok_or("Can't find num-points input element")?;
    let num_points_input = num_points_input.dyn_into::<web_sys::HtmlInputElement>()?;

    let gl_clone = gl.clone();
    let program_clone = program.clone();
    let mouse_state_clone = mouse_state.clone();
    let mv_matrix_values_clone = mv_matrix_values.clone();
    let num_points_input_clone = num_points_input.clone();
    let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
        let num_points = num_points_input_clone.value().parse().unwrap_or(1000);
        update_num_points(
            &gl_clone,
            &program_clone,
            mouse_state_clone.clone(),
            mv_matrix_values_clone.clone(),
            num_points,
        );
    }) as Box<dyn FnMut(_)>);

    num_points_input.set_oninput(Some(closure.as_ref().unchecked_ref()));
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
    let mut octree = create_octree();
    let (buffer, octree_vertex_count) = setup_vertex_buffer(gl, &mut octree, num_points).unwrap();
    setup_vertex_attributes(gl, program);

    let scale_factor = 1.0;
    render::start_render_loop(
        gl.clone(),
        program.clone(),
        scale_factor,
        mouse_state,
        mv_matrix_values,
        num_points as u32,
        octree_vertex_count as u32,
    );
}
