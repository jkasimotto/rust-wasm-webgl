use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext;

mod mouse;
mod render;
mod shaders;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let gl = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;


    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    let program = shaders::create_program(&gl)?;
    gl.use_program(Some(&program));

    let vertices: [f32; 18] = [
        0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ];

    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &js_sys::Float32Array::from(vertices.as_slice()),
        WebGlRenderingContext::STATIC_DRAW,
    );

    let position_attribute_location = gl.get_attrib_location(&program, "position");
    gl.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        3,
        WebGlRenderingContext::FLOAT,
        false,
        0,
        0,
    );
    gl.enable_vertex_attrib_array(position_attribute_location as u32);

    let mouse_state = mouse::create_mouse_state(&canvas)?;
    let scale_factor = 1.0;

    render::start_render_loop(gl, program, scale_factor, mouse_state);

    Ok(())
}
