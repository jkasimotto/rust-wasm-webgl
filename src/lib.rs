use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext, WebGlProgram, WebGlShader};
use nalgebra_glm::{perspective, scale, Mat4, Vec3};


fn render_scene(gl: &WebGlRenderingContext, program: &WebGlProgram, scale_factor: f32) {
    let mv_matrix = Mat4::look_at_rh(
        &Vec3::new(1.5, 1.5, 1.5).into(),
        &Vec3::new(0.0, 0.0, 0.0).into(),
        &Vec3::new(0.0, 1.0, 0.0),
    );

    let scaling_vector = Vec3::new(scale_factor, scale_factor, scale_factor);
    let scaled_mv_matrix = scale(&mv_matrix, &scaling_vector);

    let p_matrix = perspective(800.0 / 600.0, 45.0_f32.to_radians(), 0.1, 100.0);

    let mv_matrix_location = gl.get_uniform_location(&program, "uMVMatrix").unwrap();
    let p_matrix_location = gl.get_uniform_location(&program, "uPMatrix").unwrap();

    gl.uniform_matrix4fv_with_f32_array(
        Some(&mv_matrix_location),
        false,
        scaled_mv_matrix.as_slice(),
    );

    gl.uniform_matrix4fv_with_f32_array(
        Some(&p_matrix_location),
        false,
        p_matrix.as_slice(),
    );

    gl.clear_color(1.0, 1.0, 1.0, 1.0);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);
    gl.enable(WebGlRenderingContext::DEPTH_TEST);
    gl.draw_arrays(WebGlRenderingContext::LINES, 0, 6);
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let gl = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    let vert_shader = compile_shader(
        &gl,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        attribute vec3 position;
        uniform mat4 uMVMatrix;
        uniform mat4 uPMatrix;
        void main() {
            gl_Position = uPMatrix * uMVMatrix * vec4(position, 1.0);
        }
    "#,
    )?;

    let frag_shader = compile_shader(
        &gl,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        void main() {
            gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        }
    "#,
    )?;

    let program = link_program(&gl, &vert_shader, &frag_shader)?;
    gl.use_program(Some(&program));

    let vertices: [f32; 18] = [
        0.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        0.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 0.0,
        0.0, 0.0, 1.0,
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

    let scale_factor = 1.0;
    let scale_speed = 0.01;

    let gl_clone = gl.clone();
    let program_clone = program.clone();
    let scale_factor_ref = Rc::new(RefCell::new(scale_factor));

    let render_loop = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let render_loop_clone = render_loop.clone();

    *render_loop_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let mut scale_factor = scale_factor_ref.borrow_mut();
        *scale_factor -= scale_speed;
        if *scale_factor < 0.1 {
            *scale_factor = 1.0;
        }

        render_scene(&gl_clone, &program_clone, *scale_factor);

        web_sys::window().unwrap().request_animation_frame(render_loop.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();
    }) as Box<dyn FnMut()>));

    web_sys::window().unwrap().request_animation_frame(render_loop_clone.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();


    Ok(())
}

fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<web_sys::WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| "Could not create shader".to_string())?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".to_string()))
    }
}

fn link_program(
    gl: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| "Unable to create shader program".to_string())?;

    gl.attach_shader(&program, vert_shader);
    gl.attach_shader(&program, frag_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error creating program".to_string()))
    }
}