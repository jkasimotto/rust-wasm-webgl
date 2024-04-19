// This code uses functionality from the wasm_bindgen and web_sys crates.
// wasm_bindgen is a library that facilitates interoperability between Rust and JavaScript.
// web_sys is a crate that provides bindings for Web APIs, including WebGL.
use wasm_bindgen::JsValue;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

// This function creates a shader program, which is a fundamental concept in WebGL.
// A shader program consists of a vertex shader and a fragment shader.
// The vertex shader processes each vertex of the geometry and determines its position on the screen.
// The fragment shader determines the color of each pixel that makes up the geometry.
pub fn create_program(gl: &WebGlRenderingContext) -> Result<WebGlProgram, JsValue> {
    let vert_shader = compile_shader(
        &gl,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        attribute vec3 position;
        attribute vec3 color;
        uniform mat4 uMVMatrix;
        uniform mat4 uPMatrix;

        varying vec3 vColor;

        void main() {
            gl_Position = uPMatrix * uMVMatrix * vec4(position, 1.0);
            vColor = color;
        }
        "#,
    )?;

    let frag_shader = compile_shader(
        &gl,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        precision mediump float;

        varying vec3 vColor;

        void main() {
            gl_FragColor = vec4(vColor, 1.0);
        }
        "#,
    )?;

    link_program(&gl, &vert_shader, &frag_shader)
}
// This function compiles a shader from its source code.
// It takes the WebGL rendering context, the shader type (vertex or fragment), and the shader source code as input.
fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, JsValue> {
    // Create a new shader of the specified type (vertex or fragment).
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| "Could not create shader".to_string())?;

    // Set the shader source code.
    gl.shader_source(&shader, source);

    // Compile the shader.
    gl.compile_shader(&shader);

    // Check if the shader compilation was successful.
    // Rust's Result type is used for error handling. Ok is returned if the compilation succeeded, and Err is returned if it failed.
    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".to_string())
            .into())
    }
}

// This function links the compiled vertex and fragment shaders into a shader program.
// Linking combines the shaders into a single program that can be used for rendering.
fn link_program(
    gl: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, JsValue> {
    // Create a new shader program.
    let program = gl
        .create_program()
        .ok_or_else(|| "Unable to create shader program".to_string())?;

    // Attach the vertex shader to the program.
    gl.attach_shader(&program, vert_shader);

    // Attach the fragment shader to the program.
    gl.attach_shader(&program, frag_shader);

    // Link the shader program.
    gl.link_program(&program);

    // Check if the shader program linking was successful.
    // Rust's Result type is used for error handling. Ok is returned if the linking succeeded, and Err is returned if it failed.
    if gl
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error creating program".to_string())
            .into())
    }
}
