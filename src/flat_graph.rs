#![allow(dead_code)]

use rand::Rng;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaChaRng;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone)]
pub(crate) struct FlatGraph {
    pub max_capacity: usize,
    current_size: usize,
    nodes: Vec<Option<Color>>,
    vertices: Vec<Option<Vertex>>,
}

impl FlatGraph {
    pub fn new(max_nodes: usize) -> FlatGraph {
        FlatGraph {
            max_capacity: max_nodes,
            current_size: 0,
            nodes: vec![None; max_nodes],
            vertices: vec![None; max_nodes - 1],
        }
    }

    pub fn generate_random(
        &mut self,
        red_node_probability: f32,
        red_edge_probability: f32,
        left_directed_edge_probability: f32,
    ) {
        let mut chacha_secure_rng = ChaChaRng::from_entropy();
        for node in self.nodes.iter_mut() {
            match chacha_secure_rng.gen_range(0f32..1f32) {
                x if x < red_node_probability => {
                    *node = Some(Color::RED);
                }
                _ => {
                    *node = Some(Color::BLUE);
                }
            }
        }
        for vertex in self.vertices.iter_mut() {
            let vertex_color = match chacha_secure_rng.gen_range(0f32..1f32) {
                x if x < red_edge_probability => Color::RED,
                _ => Color::BLUE,
            };
            let vertex_direction = match chacha_secure_rng.gen_range(0f32..1f32) {
                x if x < left_directed_edge_probability => VertexDirection::LEFT,
                _ => VertexDirection::RIGHT,
            };
            *vertex = Some(Vertex {
                color: vertex_color,
                direction: vertex_direction,
            });
        }
        self.current_size = self.max_capacity;
    }

    pub fn get_node(&self, index: usize) -> Result<&Option<Color>, String> {
        self.nodes
            .get(index)
            .ok_or(format!("Node index {} out of bounds", index))
    }

    pub fn get_vertex(&self, index: usize) -> Result<&Option<Vertex>, String> {
        self.vertices
            .get(index)
            .ok_or(format!("Vertice index {} out of bounds", index))
    }

    pub fn node_exists(&self, index: usize) -> bool {
        self.nodes.get(index).is_some() && self.nodes.get(index).unwrap().is_some()
    }

    pub fn vertex_exists(&self, index: usize) -> bool {
        self.vertices.get(index).is_some() && self.vertices.get(index).unwrap().is_some()
    }

    pub fn add_node(&mut self, index: usize, color: Color) -> Result<(), String> {
        if self.current_size >= self.max_capacity {
            return Err(format!("Cannot add node at index >= graph max capacity"));
        }
        if self.node_exists(index) {
            return Err(format!("Node index {} already exists", index));
        }
        self.nodes[index] = Some(color);
        self.current_size += 1;
        Ok(())
    }

    pub fn add_vertex(
        &mut self,
        index: usize,
        color: Color,
        direction: VertexDirection,
    ) -> Result<(), String> {
        if self.current_size >= self.max_capacity - 1 {
            return Err(format!(
                "Cannot add vertex at index >= graph max capacity - 1"
            ));
        }
        if self.vertex_exists(index) {
            return Err(format!("Vertex index {} already exists", index));
        }
        self.vertices[index] = Some(Vertex { color, direction });
        Ok(())
    }

    pub fn remove_node(&mut self, index: usize) -> Result<(), String> {
        if index >= self.max_capacity {
            return Err(format!("Index {} is out of bounds", index));
        }
        if !self.node_exists(index) {
            return Err(format!("Node index {} does not exist", index));
        }
        self.nodes[index] = None;
        self.current_size -= 1;
        if index != 0 && self.nodes[index - 1].is_some() && self.vertices[index - 1].is_some() {
            if self.vertices[index - 1].unwrap().direction == VertexDirection::LEFT {
                self.nodes[index - 1] = Option::from(self.vertices[index - 1].unwrap().color);
            }
            self.vertices[index - 1] = None;
        }
        if index != self.max_capacity - 1
            && self.nodes[index + 1].is_some()
            && self.vertices[index].is_some()
        {
            if self.vertices[index].unwrap().direction == VertexDirection::RIGHT {
                self.nodes[index + 1] = Option::from(self.vertices[index].unwrap().color);
            }
            self.vertices[index] = None;
        }
        Ok(())
    }

    pub fn get_sequence_max(&self) -> VecDeque<usize> {
        VecDeque::<usize>::new()
    }
}

impl fmt::Display for FlatGraph {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for i in 0..=self.max_capacity - 1 {
            match self.nodes[i] {
                Some(Color::RED) => {
                    result.push_str("(R)");
                }
                Some(Color::BLUE) => {
                    result.push_str("(B)");
                }
                None => {
                    result.push_str(" ");
                }
            }
            if i == self.max_capacity - 1 {
                continue;
            }
            match self.vertices[i] {
                Some(vertex) => {
                    result.push_str(&*format!("{}", vertex));
                }
                None => {
                    result.push_str("     ");
                }
            }
        }
        write!(f, "{}", result)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum VertexDirection {
    LEFT,
    RIGHT,
}

#[derive(Clone, Copy)]
pub(crate) struct Vertex {
    pub color: Color,
    pub direction: VertexDirection,
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.direction {
            VertexDirection::LEFT => write!(f, "<-{}-", self.color),
            VertexDirection::RIGHT => write!(f, "-{}->", self.color),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Color {
    RED,
    BLUE,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Color::RED => write!(f, "R"),
            Color::BLUE => write!(f, "B"),
        }
    }
}
