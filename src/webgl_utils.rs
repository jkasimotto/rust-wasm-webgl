use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext;
use wasm_bindgen::JsCast; // Add this line

pub fn get_webgl_context(
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