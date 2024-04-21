// octree.rs
use nalgebra_glm::Vec3;

#[derive(Clone)] // Add this line
pub struct Octree {
    center: Vec3,
    size: f32,
    point_indices: Vec<usize>,
    children: Option<Box<[Octree; 8]>>,
}

impl Octree {
    pub fn new(center: Vec3, size: f32) -> Self {
        Octree {
            center,
            size,
            point_indices: Vec::new(),
            children: None,
        }
    }

    // Adjust the insert method to accept a point index and the point itself
    // The point is needed to determine the correct child, but only the index is stored
    pub fn insert(&mut self, point_index: usize, point: Vec3) {
        if self.size <= 1.0 {
            self.point_indices.push(point_index);
            return;
        }

        if self.children.is_none() {
            self.children = Some(Box::new([
                Octree::new(
                    self.center + Vec3::new(-self.size / 4.0, -self.size / 4.0, -self.size / 4.0),
                    self.size / 2.0,
                ),
                Octree::new(
                    self.center + Vec3::new(self.size / 4.0, -self.size / 4.0, -self.size / 4.0),
                    self.size / 2.0,
                ),
                Octree::new(
                    self.center + Vec3::new(-self.size / 4.0, self.size / 4.0, -self.size / 4.0),
                    self.size / 2.0,
                ),
                Octree::new(
                    self.center + Vec3::new(self.size / 4.0, self.size / 4.0, -self.size / 4.0),
                    self.size / 2.0,
                ),
                Octree::new(
                    self.center + Vec3::new(-self.size / 4.0, -self.size / 4.0, self.size / 4.0),
                    self.size / 2.0,
                ),
                Octree::new(
                    self.center + Vec3::new(self.size / 4.0, -self.size / 4.0, self.size / 4.0),
                    self.size / 2.0,
                ),
                Octree::new(
                    self.center + Vec3::new(-self.size / 4.0, self.size / 4.0, self.size / 4.0),
                    self.size / 2.0,
                ),
                Octree::new(
                    self.center + Vec3::new(self.size / 4.0, self.size / 4.0, self.size / 4.0),
                    self.size / 2.0,
                ),
            ]));
        }

        let child_index = self.get_child_index(point);
        self.children.as_mut().unwrap()[child_index].insert(point_index, point);
    }

    fn get_child_index(&self, point: Vec3) -> usize {
        let mut index = 0;
        if point.x > self.center.x {
            index += 1;
        }
        if point.y > self.center.y {
            index += 2;
        }
        if point.z > self.center.z {
            index += 4;
        }
        index
    }

    pub fn get_vertices(&self, vertices: &mut Vec<f32>) {
        if self.children.is_none() {
            // If the current node is a leaf node, generate its cube vertices
            self.generate_cube_vertices(vertices);
        } else {
            // If the current node has children, recursively get the vertices of the child nodes
            for child in self.children.as_ref().unwrap().iter() {
                child.get_vertices(vertices);
            }
        }
    }

    fn generate_cube_vertices(&self, vertices: &mut Vec<f32>) {
        let half_size = self.size / 2.0;
        let min_pos = self.center - Vec3::new(half_size, half_size, half_size);
        let max_pos = self.center + Vec3::new(half_size, half_size, half_size);

        // Generate the vertices of the cube as triangles in counterclockwise order (when viewed from outside)

        // Front face (counterclockwise)
        vertices.extend_from_slice(&[
            min_pos.x, min_pos.y, max_pos.z, 0.5, 0.5, 0.5, // Bottom-left
            max_pos.x, min_pos.y, max_pos.z, 0.5, 0.5, 0.5, // Bottom-right
            max_pos.x, max_pos.y, max_pos.z, 0.5, 0.5, 0.5, // Top-right
            min_pos.x, min_pos.y, max_pos.z, 0.5, 0.5, 0.5, // Bottom-left
            max_pos.x, max_pos.y, max_pos.z, 0.5, 0.5, 0.5, // Top-right
            min_pos.x, max_pos.y, max_pos.z, 0.5, 0.5, 0.5, // Top-left
        ]);

        // Back face (counterclockwise)
        vertices.extend_from_slice(&[
            max_pos.x, min_pos.y, min_pos.z, 0.5, 0.5, 0.5, // Bottom-right
            min_pos.x, min_pos.y, min_pos.z, 0.5, 0.5, 0.5, // Bottom-left
            min_pos.x, max_pos.y, min_pos.z, 0.5, 0.5, 0.5, // Top-left
            max_pos.x, min_pos.y, min_pos.z, 0.5, 0.5, 0.5, // Bottom-right
            min_pos.x, max_pos.y, min_pos.z, 0.5, 0.5, 0.5, // Top-left
            max_pos.x, max_pos.y, min_pos.z, 0.5, 0.5, 0.5, // Top-right
        ]);

        // Left face (counterclockwise)
        vertices.extend_from_slice(&[
            min_pos.x, min_pos.y, min_pos.z, 0.5, 0.5, 0.5, // Bottom-left
            min_pos.x, min_pos.y, max_pos.z, 0.5, 0.5, 0.5, // Bottom-right
            min_pos.x, max_pos.y, max_pos.z, 0.5, 0.5, 0.5, // Top-right
            min_pos.x, min_pos.y, min_pos.z, 0.5, 0.5, 0.5, // Bottom-left
            min_pos.x, max_pos.y, max_pos.z, 0.5, 0.5, 0.5, // Top-right
            min_pos.x, max_pos.y, min_pos.z, 0.5, 0.5, 0.5, // Top-left
        ]);

        // Right face (counterclockwise)
        vertices.extend_from_slice(&[
            max_pos.x, min_pos.y, max_pos.z, 0.5, 0.5, 0.5, // Bottom-right
            max_pos.x, min_pos.y, min_pos.z, 0.5, 0.5, 0.5, // Bottom-left
            max_pos.x, max_pos.y, min_pos.z, 0.5, 0.5, 0.5, // Top-left
            max_pos.x, min_pos.y, max_pos.z, 0.5, 0.5, 0.5, // Bottom-right
            max_pos.x, max_pos.y, min_pos.z, 0.5, 0.5, 0.5, // Top-left
            max_pos.x, max_pos.y, max_pos.z, 0.5, 0.5, 0.5, // Top-right
        ]);

        // Top face (counterclockwise)
        vertices.extend_from_slice(&[
            min_pos.x, max_pos.y, max_pos.z, 0.5, 0.5, 0.5, // Front-left
            max_pos.x, max_pos.y, max_pos.z, 0.5, 0.5, 0.5, // Front-right
            max_pos.x, max_pos.y, min_pos.z, 0.5, 0.5, 0.5, // Back-right
            min_pos.x, max_pos.y, max_pos.z, 0.5, 0.5, 0.5, // Front-left
            max_pos.x, max_pos.y, min_pos.z, 0.5, 0.5, 0.5, // Back-right
            min_pos.x, max_pos.y, min_pos.z, 0.5, 0.5, 0.5, // Back-left
        ]);

        // Bottom face (counterclockwise)
        vertices.extend_from_slice(&[
            min_pos.x, min_pos.y, min_pos.z, 0.5, 0.5, 0.5, // Front-left
            max_pos.x, min_pos.y, min_pos.z, 0.5, 0.5, 0.5, // Front-right
            max_pos.x, min_pos.y, max_pos.z, 0.5, 0.5, 0.5, // Back-right
            min_pos.x, min_pos.y, min_pos.z, 0.5, 0.5, 0.5, // Front-left
            max_pos.x, min_pos.y, max_pos.z, 0.5, 0.5, 0.5, // Back-right
            min_pos.x, min_pos.y, max_pos.z, 0.5, 0.5, 0.5, // Back-left
        ]);
    }
    /// Returns the number of cubes in the octree.
    /// This includes both leaf and non-leaf nodes.
    pub fn get_num_cubes(&self) -> usize {
        if self.children.is_none() {
            // If there are no children, this is a leaf node, so return 1
            1
        } else {
            // If there are children, count this node plus all children recursively
            1 + self
                .children
                .as_ref()
                .unwrap()
                .iter()
                .map(|child| child.get_num_cubes())
                .sum::<usize>()
        }
    }

    pub fn query_sphere(&self, center: &Vec3, radius: f32, vertex_points: &[f32]) -> Vec<usize> {
        let mut point_indices = Vec::new();

        if !self.intersects_sphere(center, radius) {
            return point_indices;
        }

        if self.children.is_none() {
            for &point_index in &self.point_indices {
                // Each vertex point consists of 6 values: x, y, z, r, g, b
                // Calculate the offset to access the x, y, z values of the point
                let offset = point_index * 6;
                let point = Vec3::new(
                    vertex_points[offset],
                    vertex_points[offset + 1],
                    vertex_points[offset + 2],
                );

                if nalgebra_glm::distance(&point, center) <= radius {
                    point_indices.push(point_index);
                }
            }
        } else {
            for child in self.children.as_ref().unwrap().iter() {
                point_indices.extend(child.query_sphere(center, radius, vertex_points));
            }
        }

        point_indices
    }


    fn intersects_sphere(&self, center: &Vec3, radius: f32) -> bool {
        let half_size = self.size / 2.0;
        let min_pos = self.center - Vec3::new(half_size, half_size, half_size);
        let max_pos = self.center + Vec3::new(half_size, half_size, half_size);

        let closest_point = Vec3::new(
            center.x.clamp(min_pos.x, max_pos.x),
            center.y.clamp(min_pos.y, max_pos.y),
            center.z.clamp(min_pos.z, max_pos.z),
        );

        nalgebra_glm::distance(&closest_point, center) <= radius
    }
}
