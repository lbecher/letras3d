use std::rc::Rc;
use std::cell::RefCell;

use crate::types::{
    Position,
    FaceType,
    Matrix4x1,
    Matrix4x4,
    Rotation,
    Scale,
    ShaderVertex,
};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Position,
}

#[derive(Debug)]
pub struct Edge {
    origin_vertex: Rc<RefCell<Vertex>>,
    destination_vertex: Rc<RefCell<Vertex>>,
}

#[derive(Debug)]
pub struct Face {
    start_edge: Rc<Edge>,
    middle_edge: Rc<Edge>,
    end_edge: Rc<Edge>,
}

#[derive(Debug)]
pub struct Object {
    vertices: Vec<Rc<RefCell<Vertex>>>,
    edges: Vec<Rc<Edge>>,
    faces: Vec<Face>,
    pub position: Position,
    pub rotation: Rotation,
    pub scale: Scale,
    pub extrusion: Option<f64>,
    vertex_buffer: Vec<ShaderVertex>,
    triangle_index_buffer: Vec<u32>,
    line_index_buffer: Vec<u32>,
}

impl Object{
    pub fn new(position: Position) -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
            faces: Vec::new(),
            position: position,
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
            extrusion: None,
            vertex_buffer: Vec::new(),
            triangle_index_buffer: Vec::new(),
            line_index_buffer: Vec::new(),
        }
    }

    pub fn set_scale(&mut self, scale: Scale) {
        self.scale = scale;
        self.update_buffers();
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
        self.update_buffers();
    }

    pub fn set_rotation(&mut self, rotation: Rotation) {
        self.rotation = rotation;
        self.update_buffers();
    }

    pub fn set_extrusion(&mut self, extrusion: Option<f64>) {
        self.extrusion = extrusion;
        self.update_buffers();
    }

    pub fn add_face(&mut self, face: FaceType) {
        // obtém o index de cada vértice
        // caso algum não exista, adiciona
        let v0_index = match self.vertices.iter().position(|vertex|
                face.0 == vertex.borrow().position
        ) {
            Some(index) => {
                index
            }
            None => {
                self.vertices.push(Rc::new(RefCell::new(
                    Vertex {
                        position: face.0,
                    }
                )));
                self.vertices.len() - 1
            }
        };

        let v1_index = match self.vertices.iter().position(|vertex|
            face.1 == vertex.borrow().position
        ) {
            Some(index) => {
                index
            }
            None => {
                self.vertices.push(Rc::new(RefCell::new(
                    Vertex {
                        position: face.1,
                    }
                )));
                self.vertices.len() - 1
            }
        };

        let v2_index = match self.vertices.iter().position(|vertex|
            face.2 == vertex.borrow().position
        ) {
            Some(index) => {
                index
            }
            None => {
                self.vertices.push(Rc::new(RefCell::new(
                    Vertex {
                        position: face.2,
                    }
                )));
                self.vertices.len() - 1
            }
        };

        // obtém o index de cada aresta
        // caso alguma não exista, adiciona
        let e0_index = match self.edges.iter().position(|edge|
            (face.0 == edge.origin_vertex.borrow().position &&
            face.1 == edge.destination_vertex.borrow().position) ||
            (face.1 == edge.origin_vertex.borrow().position &&
            face.0 == edge.destination_vertex.borrow().position) 
        ) {
            Some(index) => {
                index
            }
            None => {
                self.edges.push(Rc::new(
                    Edge {
                        origin_vertex: Rc::clone(&self.vertices[v0_index]),
                        destination_vertex: Rc::clone(&self.vertices[v1_index]),
                    }
                ));
                self.edges.len() - 1
            }
        };

        let e1_index = match self.edges.iter().position(|edge|
            (face.1 == edge.origin_vertex.borrow().position &&
            face.2 == edge.destination_vertex.borrow().position) ||
            (face.2 == edge.origin_vertex.borrow().position &&
            face.1 == edge.destination_vertex.borrow().position)
        ) {
            Some(index) => {
                index
            }
            None => {
                self.edges.push(Rc::new(
                    Edge {
                        origin_vertex: Rc::clone(&self.vertices[v1_index]),
                        destination_vertex: Rc::clone(&self.vertices[v2_index]),
                    }
                ));
                self.edges.len() - 1
            }
        };

        let e2_index = match self.edges.iter().position(|edge|
            (face.2 == edge.origin_vertex.borrow().position &&
            face.0 == edge.destination_vertex.borrow().position) ||
            (face.0 == edge.origin_vertex.borrow().position &&
            face.2 == edge.destination_vertex.borrow().position) 
        ) {
            Some(index) => {
                index
            }
            None => {
                self.edges.push(Rc::new(
                    Edge {
                        origin_vertex: Rc::clone(&self.vertices[v2_index]),
                        destination_vertex: Rc::clone(&self.vertices[v0_index]),
                    }
                ));
                self.edges.len() - 1
            }
        };

        self.faces.push(
            Face {
                start_edge: Rc::clone(&self.edges[e0_index]),
                middle_edge: Rc::clone(&self.edges[e1_index]),
                end_edge: Rc::clone(&self.edges[e2_index]),
            }
        );

        self.update_buffers();
    }

    fn gen_x_rotation_matriz(&self) -> Matrix4x4 {
        let rotation = (self.rotation[0] * std::f64::consts::PI) / 180.0;
        Matrix4x4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, rotation.cos(), -rotation.sin(), 0.0,
            0.0, rotation.sin(), rotation.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }
    
    fn gen_y_rotation_matriz(&self) -> Matrix4x4 {
        let rotation = (self.rotation[1] * std::f64::consts::PI) / 180.0;
        Matrix4x4::new(
            rotation.cos(), 0.0, rotation.sin(), 0.0,
            0.0, 1.0, 0.0, 0.0,
            -rotation.sin(), 0.0, rotation.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }
    
    fn gen_z_rotation_matriz(&self) -> Matrix4x4 {
        let rotation = (self.rotation[2] * std::f64::consts::PI) / 180.0;
        Matrix4x4::new(
            rotation.cos(), -rotation.sin(), 0.0, 0.0,
            rotation.sin(), rotation.cos(), 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    fn gen_scaling_matriz(&self) -> Matrix4x4 {
        Matrix4x4::new(
            self.scale[0], 0.0, 0.0, 0.0,
            0.0, self.scale[1], 0.0, 0.0,
            0.0, 0.0, self.scale[2], 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    fn gen_translation_matriz(&self) -> Matrix4x4 {
        Matrix4x4::new(
            1.0, 0.0, 0.0, self.position[0],
            0.0, 1.0, 0.0, self.position[1],
            0.0, 0.0, 1.0, self.position[2],
            0.0, 0.0, 0.0, 1.0,
        )
    }

    fn update_buffers(&mut self) {
        let x_rotation_matrix: Matrix4x4 = self.gen_x_rotation_matriz();
        let y_rotation_matrix: Matrix4x4 = self.gen_y_rotation_matriz();
        let z_rotation_matrix: Matrix4x4 = self.gen_z_rotation_matriz();
        let scaling_matrix: Matrix4x4 = self.gen_scaling_matriz();
        let translation_matrix: Matrix4x4 = self.gen_translation_matriz();

        self.vertex_buffer = Vec::new();
        self.line_index_buffer = Vec::new();
        self.triangle_index_buffer = Vec::new();

        let mut vertices: Vec<Position>;

        if let Some(extrusion) = self.extrusion {
            vertices = self.vertices
                .iter()
                .map(|v| [
                    v.borrow().position[0],
                    v.borrow().position[1],
                    -extrusion / 2.0,
                    1.0,
                ])
                .collect();

            let mut extruded_vertices: Vec<Position> = Vec::new();
            for v in vertices.to_vec() {
                extruded_vertices.push([
                    v[0],
                    v[1],
                    extrusion / 2.0,
                    1.0,
                ]);
            }
            vertices.extend(extruded_vertices);
        } else {
            vertices = self.vertices
                .iter()
                .map(|v| v.borrow().position)
                .collect();
        }

        for vertex in vertices {
            let mut vertex_matrix: Matrix4x1 = Matrix4x1::from_vec(
                vertex.to_vec(),
            );

            vertex_matrix = x_rotation_matrix * vertex_matrix;
            vertex_matrix = y_rotation_matrix * vertex_matrix;
            vertex_matrix = z_rotation_matrix * vertex_matrix;
            vertex_matrix = scaling_matrix * vertex_matrix;
            vertex_matrix = translation_matrix * vertex_matrix;

            self.vertex_buffer.push(ShaderVertex {
                position: [
                    vertex_matrix[0] as f32,
                    vertex_matrix[1] as f32,
                    vertex_matrix[2] as f32,
                ],
                color: [
                    1.0,
                    1.0,
                    1.0,
                ]
            });
        }

        for edge in self.edges.iter() {
            let origin_index = self.vertices.iter()
                .position(|vertex| Rc::ptr_eq(vertex, &edge.origin_vertex))
                .unwrap();

            let destination_index = self.vertices.iter()
                .position(|vertex| Rc::ptr_eq(vertex, &edge.destination_vertex))
                .unwrap();

            self.line_index_buffer.push(origin_index as u32);
            self.line_index_buffer.push(destination_index as u32);

            if self.extrusion.is_some() {
                let vert_len = self.vertices.len();

                self.line_index_buffer.push((origin_index + vert_len) as u32);
                self.line_index_buffer.push((destination_index + vert_len) as u32);

                if Rc::strong_count(&edge) == 2 {
                    self.line_index_buffer.push(origin_index as u32);
                    self.line_index_buffer.push((origin_index + vert_len) as u32);

                    self.line_index_buffer.push(destination_index as u32);
                    self.line_index_buffer.push((destination_index + vert_len) as u32);

                    self.line_index_buffer.push(origin_index as u32);
                    self.line_index_buffer.push((destination_index + vert_len) as u32);
                }
            }
        }

        for face in self.faces.iter() {
            let start_edge_index = self.vertices.iter()
                .position(|vertex| Rc::ptr_eq(vertex, &face.start_edge.origin_vertex))
                .unwrap();

            let middle_edge_index = self.vertices.iter()
                .position(|vertex| Rc::ptr_eq(vertex, &face.middle_edge.origin_vertex))
                .unwrap();

            let end_edge_index = self.vertices.iter()
                .position(|vertex| Rc::ptr_eq(vertex, &face.end_edge.origin_vertex))
                .unwrap();

            self.triangle_index_buffer.push(start_edge_index as u32);
            self.triangle_index_buffer.push(middle_edge_index as u32);
            self.triangle_index_buffer.push(end_edge_index as u32);
        }
    }

    pub fn get_vertices_vec(&self) -> Vec<ShaderVertex> {
        self.vertex_buffer.to_vec()
    }

    pub fn get_lines_indices_vec(&self) -> Vec<u32> {
        self.line_index_buffer.to_vec()
    }

    pub fn get_triangles_indices_vec(&self) -> Vec<u32> {
        self.triangle_index_buffer.to_vec()
    }
}