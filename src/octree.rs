// octree.rs
use nalgebra_glm::Vec3;

#[derive(Clone)] // Add this line
pub struct Octree {
    center: Vec3,
    size: f32,
    points: Vec<Vec3>,
    children: Option<Box<[Octree; 8]>>,
}

impl Octree {
    pub fn new(center: Vec3, size: f32) -> Self {
        Octree {
            center,
            size,
            points: Vec::new(),
            children: None,
        }
    }

    pub fn insert(&mut self, point: Vec3) {
        if self.size <= 1.0 {
            self.points.push(point);
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
        self.children.as_mut().unwrap()[child_index].insert(point);
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
}
