#![allow(dead_code)]

use std::cell::RefCell;
use crate::flat_graph::{Color, FlatGraph, VertexDirection};
use std::collections::VecDeque;

pub struct FlatSequenceFinder {
    flat_graph_copy: RefCell<FlatGraph>,
    color_to_remove: Color,
}

impl FlatSequenceFinder {
    pub fn new(flat_graph: &FlatGraph, color_to_remove: Color) -> Self {
        FlatSequenceFinder {
            flat_graph_copy: RefCell::new(flat_graph.clone()),
            color_to_remove,
        }
    }

    fn sequence_max_push_and_remove_util(&self, current_node: usize, sequence_max: &mut VecDeque<usize>) {
        sequence_max.push_back(current_node);
        self.flat_graph_copy.borrow_mut().remove_node(current_node).unwrap(); // May be useless with a stack
    }

    fn find_nodes_to_remove_before_current(
        &self,
        current_node: usize,
        finding_direction: VertexDirection,
        sequence_max: &mut VecDeque<usize>
    ) {
        // Could MACROS be used here ?
        let is_vertex_left_or_right = |vertex_direction: VertexDirection, edge_id: usize| -> bool {
            self.flat_graph_copy.borrow()
                .get_vertex(edge_id)
                .unwrap()
                .unwrap()
                .direction
                == vertex_direction
        };
        const BACK_EDGE_COMPARE_CURRENT: fn(VertexDirection, usize, usize) -> bool =
            |vertex_direction: VertexDirection, current_node: usize, tmp_node: usize| -> bool {
                match vertex_direction {
                    VertexDirection::LEFT => current_node > tmp_node,
                    VertexDirection::RIGHT => current_node < tmp_node,
                }
            };

        let mut tmp_node = match finding_direction {
            VertexDirection::LEFT => current_node - 1,
            VertexDirection::RIGHT => current_node + 1,
        };
        let mut tmp_edge = match finding_direction {
            VertexDirection::LEFT => current_node - 1,
            VertexDirection::RIGHT => current_node,
        };
        loop {
            if self.flat_graph_copy.borrow().node_exists(tmp_node)
                && self.flat_graph_copy.borrow().get_node(tmp_node).unwrap().unwrap() == self.color_to_remove
                && self.flat_graph_copy.borrow().vertex_exists(tmp_edge)
                && is_vertex_left_or_right(finding_direction, tmp_edge)
            {
                match finding_direction {
                    VertexDirection::LEFT => {
                        tmp_node -= 1;
                        tmp_edge -= 1;
                    }
                    VertexDirection::RIGHT => {
                        tmp_node += 1;
                        tmp_edge += 1;
                    }
                }
            } else {
                let shift_node_and_vertex =
                    |node: &mut usize, vertex: &mut usize| match finding_direction {
                        VertexDirection::LEFT => {
                            *node += 1;
                            *vertex += 1;
                        }
                        VertexDirection::RIGHT => {
                            *node -= 1;
                            *vertex -= 1;
                        }
                    };
                if finding_direction == VertexDirection::RIGHT && (tmp_node == 0 || tmp_edge == 0) {
                    break;
                }
                shift_node_and_vertex(&mut tmp_node, &mut tmp_edge);
                while BACK_EDGE_COMPARE_CURRENT(finding_direction, current_node, tmp_node) {
                    self.sequence_max_push_and_remove_util(tmp_node, sequence_max);
                    if finding_direction == VertexDirection::RIGHT
                        && (tmp_node == 0 || tmp_edge == 0)
                    {
                        break;
                    }
                    shift_node_and_vertex(&mut tmp_node, &mut tmp_edge);
                }
                break;
            }
        }
    }

    fn node_and_vertex_have_color(&self, node: usize, vertex: usize) -> bool {
        self.flat_graph_copy.borrow().node_exists(node)
            && self.flat_graph_copy.borrow().get_node(node).unwrap().unwrap() == self.color_to_remove
            && self.flat_graph_copy.borrow().vertex_exists(vertex)
            && self
                .flat_graph_copy.borrow()
                .get_vertex(vertex)
                .unwrap()
                .unwrap()
                .color
                == self.color_to_remove
    }

    fn node_may_be_interesting_to_remove(
        &self,
        node: usize,
        finding_direction: VertexDirection,
    ) -> bool {
        if node == 0 && finding_direction == VertexDirection::LEFT {
            return false;
        }
        let edge_tmp = match finding_direction {
            VertexDirection::LEFT => node - 1,
            VertexDirection::RIGHT => node,
        };
        if !self.flat_graph_copy.borrow().vertex_exists(edge_tmp) {
            return false;
        }
        if finding_direction
            != self
                .flat_graph_copy.borrow()
                .get_vertex(edge_tmp)
                .unwrap()
                .unwrap()
                .direction
        {
            return false;
        }
        let node_dst: usize = match finding_direction {
            VertexDirection::LEFT => node - 1,
            VertexDirection::RIGHT => node + 1,
        };
        self.node_and_vertex_have_color(node, edge_tmp)
            && self.flat_graph_copy.borrow().node_exists(node_dst)
            && self.flat_graph_copy.borrow().get_node(node_dst).unwrap().unwrap() != self.color_to_remove
    }

    pub fn calculate_sequence_max(&self) -> VecDeque<usize> {
        let mut sequence_max = VecDeque::new();
        let mut current_node: usize = 0;
        while current_node < self.flat_graph_copy.borrow().max_capacity {
            if self.node_may_be_interesting_to_remove(current_node, VertexDirection::RIGHT) {
                if self.node_may_be_interesting_to_remove(current_node, VertexDirection::LEFT) {
                    self.sequence_max_push_and_remove_util(current_node, &mut sequence_max);
                    current_node -= 1;
                } else {
                    self.sequence_max_push_and_remove_util(current_node, &mut sequence_max);
                    current_node += 1;
                }
            } else {
                if self.node_may_be_interesting_to_remove(current_node, VertexDirection::LEFT) {
                    self.find_nodes_to_remove_before_current(current_node, VertexDirection::RIGHT, &mut sequence_max);
                    self.sequence_max_push_and_remove_util(current_node, &mut sequence_max);
                    current_node -= 1;
                } else {
                    if self.flat_graph_copy.borrow().node_exists(current_node)
                        && self
                            .flat_graph_copy.borrow()
                            .get_node(current_node)
                            .unwrap()
                            .unwrap()
                            == self.color_to_remove
                    {
                        self.find_nodes_to_remove_before_current(
                            current_node,
                            VertexDirection::RIGHT,
                            &mut sequence_max
                        );
                        self.sequence_max_push_and_remove_util(current_node, &mut sequence_max);
                    }
                    current_node += 1;
                }
            }
        }
        sequence_max
    }
}
