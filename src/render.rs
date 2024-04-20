// render.rs
use web_sys::Document;
use crate::mouse::MouseState;
use crate::MVMatrixValues;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, HtmlInputElement};

pub fn render_scene(
    gl: &WebGlRenderingContext,
    program: &WebGlProgram,
    distance: f32,
    mouse_state: &MouseState,
    mv_matrix_values: &MVMatrixValues,
    num_points: u32,
) {
    let mv_matrix = create_model_view_matrix(distance, mouse_state, mv_matrix_values);
    let p_matrix = create_projection_matrix();
    set_uniform_matrices(gl, program, &mv_matrix, &p_matrix);
    setup_rendering(gl);
    let scale_factor_location = gl.get_uniform_location(&program, "uScaleFactor").unwrap();
    gl.uniform1f(Some(&scale_factor_location), -distance);
    gl.draw_arrays(WebGlRenderingContext::LINES, 0, 6); // Draw the XYZ axis lines
    gl.draw_arrays(WebGlRenderingContext::POINTS, 6, num_points as i32); // Draw the random points
}

fn create_model_view_matrix(distance: f32, mouse_state: &MouseState, mv_matrix_values: &MVMatrixValues) -> nalgebra_glm::Mat4 {
    let mv_matrix = nalgebra_glm::Mat4::look_at_rh(
        &mv_matrix_values.eye.into(),
        &mv_matrix_values.target.into(),
        &mv_matrix_values.up.into(),
    );
    let translation_vector = nalgebra_glm::Vec3::new(0.0, 0.0, -distance);
    let translated_mv_matrix = mv_matrix.append_translation(&translation_vector);
    let rotation_x_matrix =
        nalgebra_glm::Mat4::new_rotation(nalgebra_glm::Vec3::x() * mouse_state.rotation_x);
    let rotation_y_matrix =
        nalgebra_glm::Mat4::new_rotation(nalgebra_glm::Vec3::y() * mouse_state.rotation_y);
    translated_mv_matrix * rotation_x_matrix * rotation_y_matrix
}

fn create_projection_matrix() -> nalgebra_glm::Mat4 {
    nalgebra_glm::perspective(800.0 / 600.0, 45.0_f32.to_radians(), 0.1, 100.0)
}

fn set_uniform_matrices(
    gl: &WebGlRenderingContext,
    program: &WebGlProgram,
    mv_matrix: &nalgebra_glm::Mat4,
    p_matrix: &nalgebra_glm::Mat4,
) {
    let mv_matrix_location = gl.get_uniform_location(&program, "uMVMatrix").unwrap();
    let p_matrix_location = gl.get_uniform_location(&program, "uPMatrix").unwrap();
    gl.uniform_matrix4fv_with_f32_array(Some(&mv_matrix_location), false, mv_matrix.as_slice());
    gl.uniform_matrix4fv_with_f32_array(Some(&p_matrix_location), false, p_matrix.as_slice());
}

fn setup_rendering(gl: &WebGlRenderingContext) {
    gl.clear_color(1.0, 1.0, 1.0, 1.0);
    gl.line_width(2.0);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);
    gl.enable(WebGlRenderingContext::DEPTH_TEST);
}

pub fn start_render_loop(
    gl: WebGlRenderingContext,
    program: WebGlProgram,
    scale_factor: f32,
    mouse_state: Rc<RefCell<MouseState>>,
    mv_matrix_values: Rc<RefCell<MVMatrixValues>>,
    num_points: u32
) {
    let scale_factor_ref = Rc::new(RefCell::new(scale_factor));
    let mouse_state_clone = mouse_state.clone();
    let render_loop = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let render_loop_clone = render_loop.clone();
    let scale_factor_ref_clone = scale_factor_ref.clone();
    let mv_matrix_values_clone = mv_matrix_values.clone();

    let wheel_handler = create_wheel_handler(scale_factor_ref_clone);
    add_wheel_event_listener(wheel_handler);

    let slider_handler = create_slider_handler(mv_matrix_values_clone.clone());
    add_slider_event_listener(slider_handler);

    *render_loop_clone.borrow_mut() = Some(create_render_loop_closure(
        gl,
        program,
        scale_factor_ref,
        mouse_state_clone,
        render_loop,
        mv_matrix_values_clone,
        num_points
    ));

    request_animation_frame(render_loop_clone);
}

fn create_wheel_handler(
    scale_factor_ref: Rc<RefCell<f32>>,
) -> Closure<dyn FnMut(web_sys::WheelEvent)> {
    Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
        let mut scale_factor = scale_factor_ref.borrow_mut();
        let delta = event.delta_y() as f32;
        *scale_factor += delta * 0.001;
    }) as Box<dyn FnMut(_)>)
}

fn add_wheel_event_listener(wheel_handler: Closure<dyn FnMut(web_sys::WheelEvent)>) {
    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback("wheel", wheel_handler.as_ref().unchecked_ref())
        .unwrap();
    wheel_handler.forget();
}

fn create_slider_handler(mv_matrix_values: Rc<RefCell<MVMatrixValues>>) -> Closure<dyn FnMut(web_sys::Event)> {
    Closure::wrap(Box::new(move |event: web_sys::Event| {
        let target = event.target().unwrap();
        let input = target.dyn_ref::<web_sys::HtmlInputElement>().unwrap();
        let value = input.value().parse::<f32>().unwrap();

        let mut mv_matrix_values = mv_matrix_values.borrow_mut();
        match input.id().as_str() {
            "eye-x" => mv_matrix_values.eye.x = value,
            "eye-y" => mv_matrix_values.eye.y = value,
            "eye-z" => mv_matrix_values.eye.z = value,
            "target-x" => mv_matrix_values.target.x = value,
            "target-y" => mv_matrix_values.target.y = value,
            "target-z" => mv_matrix_values.target.z = value,
            "up-x" => mv_matrix_values.up.x = value,
            "up-y" => mv_matrix_values.up.y = value,
            "up-z" => mv_matrix_values.up.z = value,
            _ => {}
        }
    }) as Box<dyn FnMut(_)>)
}



fn add_slider_event_listener(slider_handler: Closure<dyn FnMut(web_sys::Event)>) {
    let window = web_sys::window().expect("No global window exists");
    let document = window.document().expect("Should have a document on window");
    let slider_ids = ["eye-x", "eye-y", "eye-z", "target-x", "target-y", "target-z", "up-x", "up-y", "up-z"];
    for slider_id in slider_ids.iter() {
        let slider = document.query_selector(&format!("input[type=range][id={}]", slider_id)).unwrap().unwrap();
        slider
            .add_event_listener_with_callback("input", slider_handler.as_ref().unchecked_ref())
            .unwrap();
    }
    slider_handler.forget();
}

fn create_render_loop_closure(
    gl: WebGlRenderingContext,
    program: WebGlProgram,
    scale_factor_ref: Rc<RefCell<f32>>,
    mouse_state: Rc<RefCell<MouseState>>,
    render_loop: Rc<RefCell<Option<Closure<dyn FnMut()>>>>,
    mv_matrix_values: Rc<RefCell<MVMatrixValues>>,
    num_points: u32
) -> Closure<dyn FnMut()> {
    Closure::wrap(Box::new(move || {
        let scale_factor = *scale_factor_ref.borrow();
        let mouse_state = mouse_state.borrow();
        let mv_matrix_values = mv_matrix_values.borrow();
        render_scene(&gl, &program, scale_factor, &mouse_state, &mv_matrix_values, num_points);
        request_animation_frame(render_loop.clone());
    }) as Box<dyn FnMut()>)
}

fn request_animation_frame(render_loop: Rc<RefCell<Option<Closure<dyn FnMut()>>>>) {
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
}