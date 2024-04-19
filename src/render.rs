use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WheelEvent};
use crate::mouse::MouseState;

pub fn render_scene(gl: &WebGlRenderingContext, program: &WebGlProgram, distance: f32, mouse_state: &MouseState) {
    // Create a Model-View matrix that positions the camera
    let mv_matrix = nalgebra_glm::Mat4::look_at_rh(
        &nalgebra_glm::Vec3::new(1.5, 1.5, 1.5).into(),
        &nalgebra_glm::Vec3::new(0.0, 0.0, 0.0).into(),
        &nalgebra_glm::Vec3::new(0.0, 1.0, 0.0),
    );

    // Create a translation vector based on the distance
    let translation_vector = nalgebra_glm::Vec3::new(0.0, 0.0, -distance);

    // Apply the translation to the Model-View matrix
    let translated_mv_matrix = mv_matrix.append_translation(&translation_vector);

    // Create rotation matrices based on the mouse state
    let rotation_x_matrix = nalgebra_glm::Mat4::new_rotation(nalgebra_glm::Vec3::x() * mouse_state.rotation_x);
    let rotation_y_matrix = nalgebra_glm::Mat4::new_rotation(nalgebra_glm::Vec3::y() * mouse_state.rotation_y);

    // Apply the rotations to the translated Model-View matrix
    let rotated_mv_matrix = translated_mv_matrix * rotation_x_matrix * rotation_y_matrix;

    // Create a Projection matrix with a 45-degree field of view and a 4:3 aspect ratio
    let p_matrix = nalgebra_glm::perspective(800.0 / 600.0, 45.0_f32.to_radians(), 0.1, 100.0);

    // Get the locations of the uniform variables in the shader program
    let mv_matrix_location = gl.get_uniform_location(&program, "uMVMatrix").unwrap();
    let p_matrix_location = gl.get_uniform_location(&program, "uPMatrix").unwrap();

    // Set the values of the uniform variables in the shader program
    gl.uniform_matrix4fv_with_f32_array(
        Some(&mv_matrix_location),
        false,
        rotated_mv_matrix.as_slice(),
    );
    gl.uniform_matrix4fv_with_f32_array(Some(&p_matrix_location), false, p_matrix.as_slice());

    // Set the clear color to white
    gl.clear_color(1.0, 1.0, 1.0, 1.0);

    // Set the line width to 2 pixels
    gl.line_width(2.0);

    // Clear the color buffer and the depth buffer
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);

    // Enable depth testing
    gl.enable(WebGlRenderingContext::DEPTH_TEST);

    // Draw 6 vertices as lines
    gl.draw_arrays(WebGlRenderingContext::LINES, 0, 6);
}

pub fn start_render_loop(gl: WebGlRenderingContext, program: WebGlProgram, scale_factor: f32, mouse_state: Rc<RefCell<MouseState>>) {
    // Create a shared reference to the scale factor
    let scale_factor_ref = Rc::new(RefCell::new(scale_factor));

    // Clone the shared reference to the mouse state
    let mouse_state_clone = mouse_state.clone();

    // Create a shared reference to store the render loop closure
    let render_loop = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let render_loop_clone = render_loop.clone();

    // Clone the shared reference to the scale factor
    let scale_factor_ref_clone = scale_factor_ref.clone();

    // Create a closure to handle wheel events
    let wheel_handler = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
        // Borrow a mutable reference to the scale factor
        let mut scale_factor = scale_factor_ref_clone.borrow_mut();

        // Get the delta value from the wheel event
        let delta = event.delta_y() as f32;

        // Update the scale factor based on the delta value
        *scale_factor += delta * 0.001;
    }) as Box<dyn FnMut(_)>);

    // Add the wheel event listener to the window
    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback("wheel", wheel_handler.as_ref().unchecked_ref())
        .unwrap();

    // Forget the wheel handler to prevent it from being dropped
    wheel_handler.forget();

    // Create a closure for the render loop
    *render_loop_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        // Borrow the current scale factor and mouse state
        let scale_factor = *scale_factor_ref.borrow();
        let mouse_state = mouse_state_clone.borrow();

        // Call the render_scene function with the current state
        render_scene(&gl, &program, scale_factor, &mouse_state);

        // Request the next animation frame to continue the render loop
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

    // Start the render loop by requesting the first animation frame
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