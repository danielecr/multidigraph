use petgraph::Graph;
use petgraph::graph::NodeIndex;
use serde::Serialize;
use std::{collections::HashMap, hash::Hash};

use crate::adjac::{Adjac, MyDAG};

/// Trait for NodePath, the argument of Multidigraph.add_paths
pub trait NodePathTrait<T> where T: PartialOrd + Clone + std::fmt::Display + Eq + Hash {
    fn new(node: T, edges: Vec<T>) -> Self;
    fn get_node(&self) -> T;
    fn get_edges(&self) -> Vec<T>;
}

#[derive(Serialize)]
pub struct NodePath<T: PartialOrd + Clone + std::fmt::Display + Eq + Hash> {
    pub node: T,
    pub edges: Vec<T>,
}

impl<T> NodePathTrait<T> for NodePath<T> where T: PartialOrd + Clone + std::fmt::Display + Eq + Hash {
    fn new(node: T, edges: Vec<T>) -> Self {
        Self {
            node,
            edges,
        }
    }

    fn get_node(&self) -> T {
        self.node.clone()
    }

    fn get_edges(&self) -> Vec<T> {
        self.edges.clone()
    }
}

impl <T: PartialOrd + Clone + std::fmt::Display + Eq + Hash> NodePath<T> {
    pub fn new(node: T, edges: Vec<T>) -> NodePath<T> {
        Self {
            node,
            edges,
        }
    }
}

pub struct Multidigraph<T> where T: PartialOrd + Clone + Eq + Hash  {
    pub graph: Graph<T,()>,
    pub nodes: HashMap<T,NodeIndex>,
    adjac: Option<Adjac<T>>,
    pub connected_dags: Option<Vec<MyDAG>>,
    pub need_remove: Option<HashMap<T,Vec<T>>>,
}

impl<T: PartialOrd + Clone + std::fmt::Display + Eq + Hash > Multidigraph<T> {

    pub fn new() -> Multidigraph<T> {
        Multidigraph {
            graph: Graph::<T,()>::new(),
            nodes: HashMap::<T,NodeIndex>::new(),
            adjac: None,
            connected_dags: None,
            need_remove: None,
        }
    }

    pub fn add_paths<'a>(&mut self, node_path: Vec<&'a impl NodePathTrait<T>>) {
        for node in node_path.iter() {
            // check if node is already in the graph
            if self.nodes.contains_key(&node.get_node()) {
                // if it is, add the edges to the graph
                let node_i = *self.nodes.get(&node.get_node()).unwrap();
                for edge in node.get_edges().iter() {
                    // check if edge is already in the graph
                    if self.nodes.contains_key(edge) {
                        // if it is, add an edge between NodePath.node and edge
                        self.graph.add_edge(node_i, *self.nodes.get(edge).unwrap(), ());
                        continue;
                    } else {
                        // if it is not, add it to the graph
                        let edge_i = self.graph.add_node(edge.clone());
                        self.nodes.insert(edge.clone(), edge_i);
                        // then add an edge between NodePath.node and edge
                        self.graph.update_edge(node_i, edge_i, ());
                    }
                }
                continue;
            }
            let node_i = self.graph.add_node(node.get_node());
            self.nodes.insert(node.get_node().clone(), node_i);
            // fill the edges
            for edge in node.get_edges().iter() {
                // check if edge is already in the graph
                if self.nodes.contains_key(edge) {
                    // if it is, add an edge between NodePath.node and edge
                    self.graph.update_edge(node_i, *self.nodes.get(edge).unwrap(), ());
                } else {
                    // if it is not, add it to the graph
                    let edge_i = self.graph.add_node(edge.clone());
                    self.nodes.insert(edge.clone(), edge_i);
                    // then add an edge between NodePath.node and edge
                    self.graph.update_edge(node_i, edge_i, ());
                }
                continue;
            }
        }
    }

    pub fn build_adjac(&mut self) {
        self.adjac = Some(Adjac::new_from_graph(&self.graph));
        self.connected_dags = Some(self.adjac.as_ref().unwrap().connected_dags());
    }

    pub fn check_loops(&self) -> Vec<Vec<T>> {
        if self.adjac.is_none() {
            panic!("Adjac not built");
        }
        let loopd = self.adjac.as_ref().unwrap().check_loops();
        loopd
    }

    pub fn has_loop(&self) -> bool {
        if self.adjac.is_none() {
            panic!("Adjac not built");
        }
        self.check_loops().len() > 0
    }

    pub fn print_internal(&self) {
        self.adjac.as_ref().unwrap().printit();
    }

    pub fn hu_connected_dags(&self) -> Vec<crate::adjac::HuDAG<T>>{
        self.adjac.as_ref().unwrap().hu_connected_dags()
    }
}
