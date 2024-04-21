use nalgebra_glm::Vec3;
use rand::Rng;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext;

use crate::octree::Octree;

#[derive(Clone)] // Add this line
pub struct VertexData {
    pub point_vbo: web_sys::WebGlBuffer,
    pub point_ebo: web_sys::WebGlBuffer,
    pub axis_vbo: web_sys::WebGlBuffer,
    pub cube_vbo: web_sys::WebGlBuffer,
    pub octree: Octree,
}

pub fn create_vertex_buffers(
    gl: &WebGlRenderingContext,
    num_points: u32,
) -> Result<VertexData, JsValue> {
    let axis_vertices: Vec<f32> = vec![
        1.0, 0.0, 0.0, 1.0, 0.0, 0.0, // x-axis (red)
        -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, // y-axis (blue)
        0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, // z-axis (green)
        0.0, 0.0, -1.0, 0.0, 0.0, 1.0,
    ];

    let mut cloud_vertices: Vec<f32> = Vec::new();
    let mut point_indices: Vec<u32> = Vec::new();

    // Generate random points within the XYZ axis lines
    let mut rng = rand::thread_rng();
    let mut octree = Octree::new(Vec3::new(0.0, 0.0, 0.0), 2.0);
    for i in 0..num_points {
        let x = rng.gen_range(-1.0..=1.0);
        let y = rng.gen_range(-1.0..=1.0);
        let z = rng.gen_range(-1.0..=1.0);
        let point = Vec3::new(x, y, z);
        octree.insert(point);
        cloud_vertices.extend_from_slice(&[x, y, z, 0.0, 0.0, 0.0]); // Black color for points
        point_indices.push(i);
    }

    let mut cube_vertices: Vec<f32> = Vec::new();
    octree.get_vertices(&mut cube_vertices);

    let axis_buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&axis_buffer));
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &js_sys::Float32Array::from(axis_vertices.as_slice()),
        WebGlRenderingContext::STATIC_DRAW,
    );

    let cloud_buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&cloud_buffer));
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &js_sys::Float32Array::from(cloud_vertices.as_slice()),
        WebGlRenderingContext::STATIC_DRAW,
    );

    let point_index_buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(
        WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
        Some(&point_index_buffer),
    );
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
        &js_sys::Uint32Array::from(point_indices.as_slice()),
        WebGlRenderingContext::STATIC_DRAW,
    );

    let cube_buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&cube_buffer));
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &js_sys::Float32Array::from(cube_vertices.as_slice()),
        WebGlRenderingContext::STATIC_DRAW,
    );

    Ok(VertexData {
        point_vbo: cloud_buffer,
        point_ebo: point_index_buffer,
        axis_vbo: axis_buffer,
        cube_vbo: cube_buffer,
        octree: octree,
    })
}

pub fn set_vertex_attribute_pointer(
    gl: &WebGlRenderingContext,
    position_attribute_location: u32,
    color_attribute_location: u32,
) {
    gl.vertex_attrib_pointer_with_i32(
        position_attribute_location,
        3,
        WebGlRenderingContext::FLOAT,
        false,
        6 * std::mem::size_of::<f32>() as i32,
        0,
    );
    gl.enable_vertex_attrib_array(position_attribute_location);

    gl.vertex_attrib_pointer_with_i32(
        color_attribute_location,
        3,
        WebGlRenderingContext::FLOAT,
        false,
        6 * std::mem::size_of::<f32>() as i32,
        3 * std::mem::size_of::<f32>() as i32,
    );
    gl.enable_vertex_attrib_array(color_attribute_location);
}
