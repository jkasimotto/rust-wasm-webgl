use crate::input::add_num_points_event_listener;
use crate::input::add_slider_event_listener;
use crate::input::add_wheel_event_listener;
use crate::input::add_xyz_event_listener;
use crate::input::create_num_points_handler;
use crate::input::create_slider_handler;
use crate::input::create_wheel_handler;
use crate::input::create_xyz_handler;
use crate::matrix::create_model_view_matrix;
use crate::matrix::create_projection_matrix;
use crate::matrix::set_uniform_matrices;
// render.rs
use crate::mouse::MouseState;

use crate::vertex_buffer::VertexData;
use crate::MVMatrixValues;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlBuffer;
use web_sys::{WebGlProgram, WebGl2RenderingContext};

pub fn render_scene(
    gl: &WebGl2RenderingContext,
    program: &WebGlProgram,
    vertex_data: Rc<RefCell<VertexData>>,
    distance: f32,
    mouse_state: &MouseState,
    mv_matrix_values: &MVMatrixValues,
) {
    let mv_matrix = create_model_view_matrix(distance, mouse_state, mv_matrix_values);
    let p_matrix = create_projection_matrix();

    set_uniform_matrices(gl, program, &mv_matrix, &p_matrix);
    setup_rendering(gl);

    let scale_factor_location = gl.get_uniform_location(&program, "uScaleFactor").unwrap();
    let u_is_rendering_cubes = gl
        .get_uniform_location(&program, "uIsRenderingCubes")
        .unwrap();
    let u_cube_transparency = gl
        .get_uniform_location(&program, "uCubeTransparency")
        .unwrap();
    let u_is_rendering_points = gl
        .get_uniform_location(&program, "uIsRenderingPoints")
        .unwrap();
    let u_is_rendering_draggable_point = gl
        .get_uniform_location(&program, "uIsRenderingDraggablePoint")
        .unwrap();
    let u_draggable_point_transparency = gl
        .get_uniform_location(&program, "uDraggablePointTransparency")
        .unwrap();
    let u_is_rendering_sphere_surface = gl
        .get_uniform_location(&program, "uIsRenderingSphereSurface")
        .unwrap();
    let u_sphere_surface_transparency = gl
        .get_uniform_location(&program, "uSphereSurfaceTransparency")
        .unwrap();

    gl.uniform1f(Some(&scale_factor_location), -distance);

    let vertex_data_ref = vertex_data.clone();

    fn bind_and_enable_attributes(
        gl: &WebGl2RenderingContext,
        vbo: &WebGlBuffer,
        ebo: Option<&WebGlBuffer>,
    ) {
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(vbo));

        if let Some(ebo) = ebo {
            gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(ebo));
        }

        gl.vertex_attrib_pointer_with_i32(
            0,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            6 * std::mem::size_of::<f32>() as i32,
            0,
        );
        gl.enable_vertex_attrib_array(0);

        gl.vertex_attrib_pointer_with_i32(
            1,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            6 * std::mem::size_of::<f32>() as i32,
            3 * std::mem::size_of::<f32>() as i32,
        );
        gl.enable_vertex_attrib_array(1);
    }

    // Render XYZ axis lines
    gl.uniform1i(Some(&u_is_rendering_points), 0);
    gl.uniform1i(Some(&u_is_rendering_cubes), 0);
    gl.uniform1i(Some(&u_is_rendering_draggable_point), 0);
    gl.uniform1i(Some(&u_is_rendering_sphere_surface), 0);
    bind_and_enable_attributes(gl, &vertex_data_ref.borrow().axis_vbo, None);
    gl.draw_arrays(WebGl2RenderingContext::LINES, 0, 6);

    // Render points
    gl.uniform1i(Some(&u_is_rendering_points), 1);
    gl.uniform1i(Some(&u_is_rendering_cubes), 0);
    gl.uniform1i(Some(&u_is_rendering_draggable_point), 0);
    gl.uniform1i(Some(&u_is_rendering_sphere_surface), 0);

    bind_and_enable_attributes(
        gl,
        &vertex_data_ref.borrow().point_vbo,
        Some(&vertex_data_ref.borrow().point_ebo),
    );

    gl.draw_elements_with_i32(
        WebGl2RenderingContext::POINTS,
        vertex_data_ref.borrow().num_points as i32,
        WebGl2RenderingContext::UNSIGNED_INT,
        0,
    );

    // Render octree cubes
    gl.uniform1i(Some(&u_is_rendering_points), 0);
    gl.uniform1i(Some(&u_is_rendering_cubes), 1);
    gl.uniform1i(Some(&u_is_rendering_draggable_point), 0);
    gl.uniform1i(Some(&u_is_rendering_sphere_surface), 0);
    gl.uniform1f(Some(&u_cube_transparency), 0.3); // Set the desired transparency value
    bind_and_enable_attributes(gl, &vertex_data_ref.borrow().cube_vbo, None);
    gl.draw_arrays(
        WebGl2RenderingContext::TRIANGLES,
        0,
        vertex_data_ref.borrow().octree.get_num_cubes() as i32 * 36,
    );

    // Render draggable point
    gl.uniform1i(Some(&u_is_rendering_points), 0);
    gl.uniform1i(Some(&u_is_rendering_cubes), 0);
    gl.uniform1i(Some(&u_is_rendering_draggable_point), 1);
    gl.uniform1i(Some(&u_is_rendering_sphere_surface), 0);
    gl.uniform1f(Some(&u_draggable_point_transparency), 1.0); // Set the desired transparency value
    bind_and_enable_attributes(gl, &vertex_data_ref.borrow().draggable_point_vbo, None);
    gl.draw_arrays(WebGl2RenderingContext::POINTS, 0, 1);

    // Render sphere surface
    gl.uniform1i(Some(&u_is_rendering_points), 0);
    gl.uniform1i(Some(&u_is_rendering_cubes), 0);
    gl.uniform1i(Some(&u_is_rendering_draggable_point), 0);
    gl.uniform1i(Some(&u_is_rendering_sphere_surface), 1);
    gl.uniform1f(Some(&u_sphere_surface_transparency), 1.0); // Set the desired transparency value
    bind_and_enable_attributes(gl, &vertex_data_ref.borrow().sphere_vbo, None);
    gl.draw_arrays(
        WebGl2RenderingContext::TRIANGLES,
        0,
        vertex_data_ref.borrow().num_sphere_vertices as i32,
    );
}

fn setup_rendering(gl: &WebGl2RenderingContext) {
    gl.clear_color(1.0, 1.0, 1.0, 1.0);
    gl.line_width(2.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
    gl.enable(WebGl2RenderingContext::DEPTH_TEST);

    // Enable blending
    gl.enable(WebGl2RenderingContext::BLEND);
    gl.blend_func(
        WebGl2RenderingContext::SRC_ALPHA,
        WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
    );
}

pub fn start_render_loop(
    gl: Rc<WebGl2RenderingContext>,
    program: WebGlProgram,
    vertex_data: Rc<RefCell<VertexData>>,
    scale_factor: f32,
    mouse_state: Rc<RefCell<MouseState>>,
    mv_matrix_values: Rc<RefCell<MVMatrixValues>>,
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

    let num_points_handler = create_num_points_handler(gl.clone(), vertex_data.clone());
    add_num_points_event_listener(num_points_handler);

    let xyz_handler = create_xyz_handler(gl.clone(), vertex_data.clone());
    add_xyz_event_listener(xyz_handler);

    *render_loop_clone.borrow_mut() = Some(create_render_loop_closure(
        gl.clone(),
        program,
        vertex_data.clone(),
        scale_factor_ref,
        mouse_state_clone,
        render_loop,
        mv_matrix_values_clone,
    ));

    request_animation_frame(render_loop_clone);
}

fn create_render_loop_closure(
    gl: Rc<WebGl2RenderingContext>,
    program: WebGlProgram,
    vertex_data: Rc<RefCell<VertexData>>,
    scale_factor_ref: Rc<RefCell<f32>>,
    mouse_state: Rc<RefCell<MouseState>>,
    render_loop: Rc<RefCell<Option<Closure<dyn FnMut()>>>>,
    mv_matrix_values: Rc<RefCell<MVMatrixValues>>,
) -> Closure<dyn FnMut()> {
    Closure::wrap(Box::new(move || {
        let scale_factor = *scale_factor_ref.borrow();
        let mouse_state = mouse_state.borrow();
        let mv_matrix_values = mv_matrix_values.borrow();
        render_scene(
            gl.as_ref(),
            &program,
            vertex_data.clone(),
            scale_factor,
            &mouse_state,
            &mv_matrix_values,
        );
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
