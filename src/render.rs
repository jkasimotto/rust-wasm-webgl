use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WheelEvent};

use crate::mouse::MouseState;

pub fn render_scene(gl: &WebGlRenderingContext, program: &WebGlProgram, distance: f32, mouse_state: &MouseState) {
    let mv_matrix = nalgebra_glm::Mat4::look_at_rh(
        &nalgebra_glm::Vec3::new(1.5, 1.5, 1.5).into(),
        &nalgebra_glm::Vec3::new(0.0, 0.0, 0.0).into(),
        &nalgebra_glm::Vec3::new(0.0, 1.0, 0.0),
    );

    let translation_vector = nalgebra_glm::Vec3::new(0.0, 0.0, -distance);
    let translated_mv_matrix = mv_matrix.append_translation(&translation_vector);

    let rotation_x_matrix = nalgebra_glm::Mat4::new_rotation(nalgebra_glm::Vec3::x() * mouse_state.rotation_x);
    let rotation_y_matrix = nalgebra_glm::Mat4::new_rotation(nalgebra_glm::Vec3::y() * mouse_state.rotation_y);
    let rotated_mv_matrix = translated_mv_matrix * rotation_x_matrix * rotation_y_matrix;

    let p_matrix = nalgebra_glm::perspective(800.0 / 600.0, 45.0_f32.to_radians(), 0.1, 100.0);

    let mv_matrix_location = gl.get_uniform_location(&program, "uMVMatrix").unwrap();
    let p_matrix_location = gl.get_uniform_location(&program, "uPMatrix").unwrap();

    gl.uniform_matrix4fv_with_f32_array(
        Some(&mv_matrix_location),
        false,
        rotated_mv_matrix.as_slice(),
    );

    gl.uniform_matrix4fv_with_f32_array(Some(&p_matrix_location), false, p_matrix.as_slice());

    gl.clear_color(1.0, 1.0, 1.0, 1.0);
    gl.line_width(2.0);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);
    gl.enable(WebGlRenderingContext::DEPTH_TEST);
    gl.draw_arrays(WebGlRenderingContext::LINES, 0, 6);
}

pub fn start_render_loop(gl: WebGlRenderingContext, program: WebGlProgram, scale_factor: f32, mouse_state: Rc<RefCell<MouseState>>) {
    let scale_factor_ref = Rc::new(RefCell::new(scale_factor));
    let mouse_state_clone = mouse_state.clone();

    let render_loop = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let render_loop_clone = render_loop.clone();

    let scale_factor_ref_clone = scale_factor_ref.clone();
    let wheel_handler = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
        let mut scale_factor = scale_factor_ref_clone.borrow_mut();
        let delta = event.delta_y() as f32;
        *scale_factor += delta * 0.001;
    }) as Box<dyn FnMut(_)>);
    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback("wheel", wheel_handler.as_ref().unchecked_ref())
        .unwrap();
    wheel_handler.forget();

    *render_loop_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let scale_factor = *scale_factor_ref.borrow();

        let mouse_state = mouse_state_clone.borrow();
        render_scene(&gl, &program, scale_factor, &mouse_state);

        web_sys::window()
            .unwrap()
            .request_animation_frame(
                render_loop
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .unchecked_ref(),
            )
            .unwrap();
    }) as Box<dyn FnMut()>));

    web_sys::window()
        .unwrap()
        .request_animation_frame(
            render_loop_clone
                .borrow()
                .as_ref()
                .unwrap()
                .as_ref()
                .unchecked_ref(),
        )
        .unwrap();
}