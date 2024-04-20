use std::{cell::RefCell, rc::Rc};

use nalgebra_glm::{vec3, Vec3};
use rand::Rng;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext;

mod mouse;
mod octree;
mod render;
mod shaders;

use octree::Octree;

struct MVMatrixValues {
    eye: Vec3,
    target: Vec3,
    up: Vec3,
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Get the window and document objects from the web_sys crate
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Get the canvas element by its ID
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    // Get the WebGL rendering context from the canvas
    let gl = get_webgl_context(&canvas)?;

    // Create the shader program
    let program = create_shader_program(&gl)?;

    let mut octree = Octree::new(Vec3::new(0.0, 0.0, 0.0), 2.0);

    // Create a buffer to hold the vertex data
    let num_points = 100; // Set the desired number of points
    let (buffer, octree_vertex_count) = create_vertex_buffer(&gl, num_points, &mut octree)?;

    // Get the location of the "position" and "color" attributes in the shader program
    let position_attribute_location = gl.get_attrib_location(&program, "position");
    let color_attribute_location = gl.get_attrib_location(&program, "color");

    // Set up the vertex attribute pointers for the "position" and "color" attributes
    set_vertex_attribute_pointer(
        &gl,
        position_attribute_location as u32,
        color_attribute_location as u32,
    );

    // Create a mouse state object to track mouse events
    let mouse_state = mouse::create_mouse_state(&canvas)?;

    // Set the scale factor for rendering
    let scale_factor = 1.0;

    // Create initial values for eye, target, and up vectors
    let eye = vec3(0.0, 0.0, 5.0);
    let target = vec3(0.0, 0.0, 0.0);
    let up = vec3(0.0, 1.0, 0.0);
    let mv_matrix_values = Rc::new(RefCell::new(MVMatrixValues { eye, target, up }));

    // Start the rendering loop
    render::start_render_loop(
        gl,
        program,
        scale_factor,
        mouse_state,
        mv_matrix_values,
        num_points,
        octree_vertex_count as u32,
    );

    Ok(())
}

fn get_webgl_context(
    canvas: &web_sys::HtmlCanvasElement,
) -> Result<WebGlRenderingContext, JsValue> {
    let gl = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    // Set the viewport to match the canvas size
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    Ok(gl)
}

fn create_shader_program(gl: &WebGlRenderingContext) -> Result<web_sys::WebGlProgram, JsValue> {
    let program = shaders::create_program(&gl)?;
    gl.use_program(Some(&program));
    Ok(program)
}

fn create_vertex_buffer(
    gl: &WebGlRenderingContext,
    num_points: u32,
    octree: &mut Octree,
) -> Result<(web_sys::WebGlBuffer, i32), JsValue> {
    let mut vertices: Vec<f32> = vec![
        1.0, 0.0, 0.0, 1.0, 0.0, 0.0, // x-axis (red)
        -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, // y-axis (blue)
        0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, // z-axis (green)
        0.0, 0.0, -1.0, 0.0, 0.0, 1.0,
    ];

    // Generate random points within the XYZ axis lines
    let mut rng = rand::thread_rng();
    for _ in 0..num_points {
        let x = rng.gen_range(-1.0..=1.0);
        let y = rng.gen_range(-1.0..=1.0);
        let z = rng.gen_range(-1.0..=1.0);
        let point = Vec3::new(x, y, z);
        octree.insert(point);
        vertices.extend_from_slice(&[x, y, z, 0.0, 0.0, 0.0]); // Black color for points
    }

    // Calculate the number of octree cube vertices before adding them
    let initial_vertex_count = vertices.len() as i32;
    octree.get_vertices(&mut vertices);
    let octree_vertex_count = vertices.len() as i32 - initial_vertex_count;

    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &js_sys::Float32Array::from(vertices.as_slice()),
        WebGlRenderingContext::STATIC_DRAW,
    );

    Ok((buffer, octree_vertex_count))
}

fn set_vertex_attribute_pointer(
    gl: &WebGlRenderingContext,
    position_attribute_location: u32,
    color_attribute_location: u32,
) {
    gl.vertex_attrib_pointer_with_i32(
        position_attribute_location,
        3,
        WebGlRenderingContext::FLOAT,
        false,
        6 * std::mem::size_of::<f32>() as i32,
        0,
    );
    gl.enable_vertex_attrib_array(position_attribute_location);

    gl.vertex_attrib_pointer_with_i32(
        color_attribute_location,
        3,
        WebGlRenderingContext::FLOAT,
        false,
        6 * std::mem::size_of::<f32>() as i32,
        3 * std::mem::size_of::<f32>() as i32,
    );
    gl.enable_vertex_attrib_array(color_attribute_location);
}
