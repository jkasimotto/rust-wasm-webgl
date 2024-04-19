use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext;

mod mouse;
mod render;
mod shaders;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Get the window and document objects from the web_sys crate
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Get the canvas element by its ID
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    // Get the WebGL rendering context from the canvas
    let gl = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    // Set the viewport to match the canvas size
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    // Create the shader program
    let program = shaders::create_program(&gl)?;
    gl.use_program(Some(&program));

    // Define the vertices for the triangle
    let vertices: [f32; 18] = [
        0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ];

    // Create a buffer to hold the vertex data
    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Load the vertex data into the buffer
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &js_sys::Float32Array::from(vertices.as_slice()),
        WebGlRenderingContext::STATIC_DRAW,
    );

    // Get the location of the "position" attribute in the shader program
    let position_attribute_location = gl.get_attrib_location(&program, "position");

    // Set up the vertex attribute pointer for the "position" attribute
    gl.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        3,
        WebGlRenderingContext::FLOAT,
        false,
        0,
        0,
    );

    // Enable the "position" vertex attribute array
    gl.enable_vertex_attrib_array(position_attribute_location as u32);

    // Create a mouse state object to track mouse events
    let mouse_state = mouse::create_mouse_state(&canvas)?;

    // Set the scale factor for rendering
    let scale_factor = 1.0;

    // Start the rendering loop
    render::start_render_loop(gl, program, scale_factor, mouse_state);

    Ok(())
}