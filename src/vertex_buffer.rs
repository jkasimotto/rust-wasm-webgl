use nalgebra_glm::Vec3;
use rand::Rng;
use wasm_bindgen::JsValue;
use web_sys::{WebGlBuffer, WebGl2RenderingContext};

use crate::octree::Octree;

#[derive(Clone)] // Add this line
pub struct VertexData {
    pub point_vbo: web_sys::WebGlBuffer,
    pub point_ebo: web_sys::WebGlBuffer,
    pub axis_vbo: web_sys::WebGlBuffer,
    pub cube_vbo: web_sys::WebGlBuffer,
    pub octree: Octree,
    pub num_points: u32,
    pub draggable_point_vbo: web_sys::WebGlBuffer,
    pub sphere_radius: f32,
    pub sphere_vbo: web_sys::WebGlBuffer,
    pub num_sphere_vertices: u32,
}

pub fn create_vertex_buffers(
    gl: &WebGl2RenderingContext,
    num_points: u32,
) -> Result<VertexData, JsValue> {
    let axis_vertices = generate_axis_vertices();
    let (point_vertices, point_indices) = generate_point_vertices(num_points);
    let mut cube_vertices: Vec<f32> = Vec::new();
    let octree = generate_octree(&point_vertices, &mut cube_vertices);
    let draggable_point_vertex = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0];
    let (sphere_vertices, num_sphere_vertices) = generate_sphere_vertices(&[0.0, 0.0, 0.0], 0.1);

    let axis_buffer = create_axis_vbo(gl, &axis_vertices)?;
    let point_buffer = create_point_vbo(gl, &point_vertices)?;
    let point_index_buffer = create_point_ebo(gl, &point_indices)?;
    let cube_buffer = create_cube_vbo(gl, &cube_vertices)?;
    let draggable_point_buffer = create_draggable_point_vbo(gl, &draggable_point_vertex)?;
    let sphere_buffer = create_sphere_vbo(gl, &sphere_vertices)?;

    Ok(VertexData {
        point_vbo: point_buffer,
        point_ebo: point_index_buffer,
        axis_vbo: axis_buffer,
        cube_vbo: cube_buffer,
        octree: octree,
        num_points: num_points,
        draggable_point_vbo: draggable_point_buffer,
        sphere_radius: 0.1,
        sphere_vbo: sphere_buffer,
        num_sphere_vertices: num_sphere_vertices,
    })
}

fn generate_axis_vertices() -> Vec<f32> {
    vec![
        1.0, 0.0, 0.0, 1.0, 0.0, 0.0, // x-axis (red)
        -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, // y-axis (blue)
        0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, // z-axis (green)
        0.0, 0.0, -1.0, 0.0, 0.0, 1.0,
    ]
}

fn generate_point_vertices(num_points: u32) -> (Vec<f32>, Vec<u32>) {
    let mut point_vertices: Vec<f32> = Vec::new();
    let mut point_indices: Vec<u32> = Vec::new();

    // Generate random points within the XYZ axis lines
    let mut rng = rand::thread_rng();

    for i in 0..num_points {
        let x = rng.gen_range(-1.0..=1.0);
        let y = rng.gen_range(-1.0..=1.0);
        let z = rng.gen_range(-1.0..=1.0);
        point_vertices.extend_from_slice(&[x, y, z, 0.0, 0.0, 0.0]); // Black color for points
        point_indices.push(i as u32); // Account for stride
    }

    (point_vertices, point_indices)
}

fn generate_octree(point_vertices: &[f32], cube_vertices: &mut Vec<f32>) -> Octree {
    let mut octree = Octree::new(Vec3::new(0.0, 0.0, 0.0), 2.0);

    for i in (0..point_vertices.len()).step_by(6) {
        let x = point_vertices[i];
        let y = point_vertices[i + 1];
        let z = point_vertices[i + 2];
        let point = Vec3::new(x, y, z);
        let point_index = i / 6;
        octree.insert(point_index, point);
    }

    octree.get_vertices(cube_vertices);
    octree
}

pub fn generate_sphere_vertices(center: &[f32; 3], radius: f32) -> (Vec<f32>, u32) {
    let mut sphere_vertices: Vec<f32> = Vec::new();
    let sectors = 160;
    let stacks = 160;

    for i in 0..stacks + 1 {
        let stack_angle =
            std::f32::consts::PI / 2.0 - (i as f32) * std::f32::consts::PI / (stacks as f32);
        let xy = radius * stack_angle.cos();
        let z = radius * stack_angle.sin();

        for j in 0..sectors + 1 {
            let sector_angle = (j as f32) * 2.0 * std::f32::consts::PI / (sectors as f32);
            let x = xy * sector_angle.cos();
            let y = xy * sector_angle.sin();

            sphere_vertices.extend_from_slice(&[
                center[0] + x,
                center[1] + y,
                center[2] + z,
                0.0,
                1.0,
                0.0,
            ]);
        }
    }

    (sphere_vertices, ((stacks + 1) * (sectors + 1)) as u32)
}

fn create_axis_vbo(gl: &WebGl2RenderingContext, vertices: &[f32]) -> Result<WebGlBuffer, JsValue> {
    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &js_sys::Float32Array::from(vertices),
        WebGl2RenderingContext::STATIC_DRAW,
    );
    Ok(buffer)
}

fn create_point_vbo(gl: &WebGl2RenderingContext, vertices: &[f32]) -> Result<WebGlBuffer, JsValue> {
    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &js_sys::Float32Array::from(vertices),
        WebGl2RenderingContext::STATIC_DRAW,
    );
    Ok(buffer)
}

fn create_point_ebo(gl: &WebGl2RenderingContext, indices: &[u32]) -> Result<WebGlBuffer, JsValue> {
    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
        &js_sys::Uint32Array::from(indices),
        WebGl2RenderingContext::STATIC_DRAW,
    );
    Ok(buffer)
}

fn create_cube_vbo(gl: &WebGl2RenderingContext, vertices: &[f32]) -> Result<WebGlBuffer, JsValue> {
    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &js_sys::Float32Array::from(vertices),
        WebGl2RenderingContext::STATIC_DRAW,
    );
    Ok(buffer)
}
pub fn create_draggable_point_vbo(
    gl: &WebGl2RenderingContext,
    vertex: &[f32],
) -> Result<WebGlBuffer, JsValue> {
    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &js_sys::Float32Array::from(vertex),
        WebGl2RenderingContext::DYNAMIC_DRAW,
    );
    Ok(buffer)
}

pub fn create_sphere_vbo(
    gl: &WebGl2RenderingContext,
    vertices: &[f32],
) -> Result<WebGlBuffer, JsValue> {
    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &js_sys::Float32Array::from(vertices),
        WebGl2RenderingContext::DYNAMIC_DRAW,
    );
    Ok(buffer)
}

pub fn set_vertex_attribute_pointer(
    gl: &WebGl2RenderingContext,
    position_attribute_location: u32,
    color_attribute_location: u32,
) {
    gl.vertex_attrib_pointer_with_i32(
        position_attribute_location,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        6 * std::mem::size_of::<f32>() as i32,
        0,
    );
    gl.enable_vertex_attrib_array(position_attribute_location);

    gl.vertex_attrib_pointer_with_i32(
        color_attribute_location,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        6 * std::mem::size_of::<f32>() as i32,
        3 * std::mem::size_of::<f32>() as i32,
    );
    gl.enable_vertex_attrib_array(color_attribute_location);
}
