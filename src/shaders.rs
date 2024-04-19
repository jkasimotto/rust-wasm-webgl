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
    // The vertex shader source code is written in GLSL (OpenGL Shading Language).
    // It defines the position of each vertex using the 'position' attribute.
    // The 'uMVMatrix' and 'uPMatrix' are uniform variables that represent transformation matrices.
    // - 'uMVMatrix' is the Model-View matrix, which transforms vertices from model space to view space.
    //   It combines the model matrix (model to world space) and the view matrix (world to view space).
    // - 'uPMatrix' is the Projection matrix, which transforms vertices from view space to clip space.
    //   It defines the camera's projection (perspective or orthographic) and sets up the view frustum.
    let vert_shader = compile_shader(
        &gl,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
    attribute vec3 position;
    uniform mat4 uMVMatrix;
    uniform mat4 uPMatrix;

    void main() {
        // Transform the vertex position from model space to clip space.
        // The 'position' attribute is a vec3, so we need to convert it to a vec4 before multiplication.
        // We do this by appending a 1.0 as the fourth component, which represents a point in homogeneous coordinates.
        // The multiplication order is important: projection * view * model * position.
        // This transforms the vertex from model space to view space, and then from view space to clip space.
        gl_Position = uPMatrix * uMVMatrix * vec4(position, 1.0);
    }
    "#,
    )?;

    // The fragment shader source code is also written in GLSL.
    // It determines the color of each pixel. In this case, it sets the color to black (0.0, 0.0, 0.0, 1.0).
    let frag_shader = compile_shader(
        &gl,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        void main() {
            gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        }
        "#,
    )?;

    // The compiled vertex and fragment shaders are linked together into a shader program.
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
