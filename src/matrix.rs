use nalgebra_glm::Vec3;
use web_sys::{WebGlProgram, WebGlRenderingContext};

use crate::mouse::MouseState;

pub struct MVMatrixValues {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
}

pub fn create_model_view_matrix(
    distance: f32,
    mouse_state: &MouseState,
    mv_matrix_values: &MVMatrixValues,
) -> nalgebra_glm::Mat4 {
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

pub fn create_projection_matrix() -> nalgebra_glm::Mat4 {
    nalgebra_glm::perspective(800.0 / 600.0, 45.0_f32.to_radians(), 0.1, 100.0)
}

pub fn set_uniform_matrices(
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