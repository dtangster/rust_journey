#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

struct GraphNode<T, U> {
    id: T,
    data: U,
    adjacency_list: HashMap<T, GraphNode<T, U>>,
}

impl<T, U> GraphNode<T, U> {
    pub fn new(id: T, data: U) -> Self {
        GraphNode {
            id,
            data,
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_edge(&self, id: T, node: GraphNode<T, U>) -> bool {
        true
    }

    pub fn delete_edge(&self, id: T) -> bool {
        true
    }
}

struct Graph<T, U> {
    nodes: Vec<GraphNode<T, U>>,
}

impl<T, U> Graph<T, U> {
    pub fn new() -> Self {
        Graph { nodes: vec![] }
    }

    pub fn add_node(&self, id: T, data: U) -> bool {
        true
    }

    pub fn delete_node(&self, id: T) -> bool {
        true
    }
}
