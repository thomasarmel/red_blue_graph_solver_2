#![allow(dead_code)]

use rand::Rng;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone)]
pub(crate) struct FlatGraph {
    max_size: usize,
    current_size: usize,
    nodes: Vec<Option<Color>>,
    vertices: Vec<Option<Vertex>>,
}

impl FlatGraph {
    pub fn new(max_nodes: usize) -> FlatGraph {
        FlatGraph {
            max_size: max_nodes,
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
    ) { // TODO: secure random
        for node in self.nodes.iter_mut() {
            match rand::thread_rng().gen_range(0f32..1f32) {
                x if x < red_node_probability => {
                    *node = Some(Color::RED);
                }
                _ => {
                    *node = Some(Color::BLUE);
                }
            }
        }
        for vertex in self.vertices.iter_mut() {
            let vertex_color = match rand::thread_rng().gen_range(0f32..1f32) {
                x if x < red_edge_probability => Color::RED,
                _ => Color::BLUE,
            };
            let vertex_direction = match rand::thread_rng().gen_range(0f32..1f32) {
                x if x < left_directed_edge_probability => VertexDirection::LEFT,
                _ => VertexDirection::RIGHT,
            };
            *vertex = Some(Vertex {
                color: vertex_color,
                direction: vertex_direction,
            });
        }
        self.current_size = self.max_size;
    }

    pub fn get_node(&self, index: usize) -> Option<Color> {
        self.nodes[index]
    }

    pub fn remove_node(&mut self, index: usize) -> Result<(), String> {
        if index >= self.max_size {
            return Err(format!("Index {} is out of bounds", index));
        }
        self.nodes[index] = None;
        self.current_size -= 1;
        if index != 0 && self.nodes[index - 1].is_some() && self.vertices[index - 1].is_some() {
            if self.vertices[index - 1].unwrap().direction == VertexDirection::LEFT {
                self.nodes[index - 1] = Option::from(self.vertices[index - 1].unwrap().color);
            }
            self.vertices[index - 1] = None;
        }
        if index != self.max_size - 1 && self.nodes[index + 1].is_some() && self.vertices[index].is_some() {
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
        for i in 0..=self.max_size - 1 {
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
            if i == self.max_size - 1 {
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
    color: Color,
    direction: VertexDirection,
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
