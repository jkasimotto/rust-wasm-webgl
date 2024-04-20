use nalgebra_glm::Vec3;
use rand::Rng;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext;

use crate::octree::Octree;

pub fn create_vertex_buffer(
    gl: &WebGlRenderingContext,
    num_points: u32,
    octree: &mut Octree,
) -> Result<(web_sys::WebGlBuffer, i32), JsValue> {
    let mut vertices: Vec<f32> = vec![
        1.0, 0.0, 0.0, 1.0, 0.0, 0.0, // x-axis (red)
        -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, // y-axis (blue)
        0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, // z-axis (green)
        0.0, 0.0, -1.0, 0.0, 0.0, 1.0,
    ];

    // Generate random points within the XYZ axis lines
    let mut rng = rand::thread_rng();
    for _ in 0..num_points {
        let x = rng.gen_range(-1.0..=1.0);
        let y = rng.gen_range(-1.0..=1.0);
        let z = rng.gen_range(-1.0..=1.0);
        let point = Vec3::new(x, y, z);
        octree.insert(point);
        vertices.extend_from_slice(&[x, y, z, 0.0, 0.0, 0.0]); // Black color for points
    }

    // Calculate the number of octree cube vertices before adding them
    let initial_vertex_count = vertices.len() as i32;
    octree.get_vertices(&mut vertices);
    let octree_vertex_count = vertices.len() as i32 - initial_vertex_count;

    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &js_sys::Float32Array::from(vertices.as_slice()),
        WebGlRenderingContext::STATIC_DRAW,
    );

    Ok((buffer, octree_vertex_count))
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
