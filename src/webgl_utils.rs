use wasm_bindgen::JsValue;
use web_sys::{WebGl2RenderingContext, HtmlCanvasElement};
use wasm_bindgen::JsCast;

pub fn get_webgl_context(
    canvas: &HtmlCanvasElement,
) -> Result<WebGl2RenderingContext, JsValue> {
    let context = canvas
        .get_context("webgl2")?
        .ok_or_else(|| JsValue::from_str("Unable to obtain WebGL2 context"))?
        .dyn_into::<WebGl2RenderingContext>()
        .map_err(|_| JsValue::from_str("Could not cast to WebGL2RenderingContext"))?;

    // Set the viewport to match the canvas size
    context.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    Ok(context)
}