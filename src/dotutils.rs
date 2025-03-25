use std::collections::HashMap;

use crate::adjac::HuDAG;

#[derive(Debug)]
pub struct Dotutils {
    hu_dag: Vec<crate::adjac::HuDAG<String>>,
    node_list: Vec<String>,
    attributes: HashMap<String, HashMap<String, String>>,
    clusters: Vec<Cluster>,
}

#[derive(Debug)]
pub struct Cluster {
    name: String,
    nodes: Vec<String>,
}

impl Cluster {
    pub fn new(name: String, nodes: Vec<String>) -> Self {
        Self {
            name,
            nodes,
        }
    }
}

impl Dotutils {
    pub fn new(hu_dag: Vec<crate::adjac::HuDAG<String>>, node_list: Vec<String>, clusters: Vec<Cluster>) -> Self {
        Self {
            hu_dag,
            node_list,
            attributes: HashMap::new(),
            clusters,
        }
    }

    pub fn add_node_attribute(&mut self, node: &str, attribute: &str, value: &str) {
        let node = node.to_string();
        if self.node_list.contains(&node) {
            self.attributes.entry(node).or_insert(HashMap::new()).insert(attribute.to_string(), value.to_string());
        }
    }

    pub fn dot_notation(&self) -> String {
        let mut s = String::new();
        s.push_str("digraph G {\n");
        for h in self.hu_dag.iter() {
            match h {
                HuDAG::Path(p) => {
                    for (from, to) in p {
                        s.push_str(&format!("  {} -> {};\n", from, to));
                    }
                },
                HuDAG::Single(a) => {
                    s.push_str(&format!("  {};\n", a));
                }
            }
        }
        s.push_str("}\n");
        s
    }

    pub fn dot_notation_augmented(&self, cluster: bool) -> String {
        let mut s = String::new();
        s.push_str("digraph G {\n");
        for n in self.node_list.iter() {
            s.push_str(&format!("  {} [label=\"{}\"", n, n));
            if let Some(attr) = self.attributes.get(n) {
                for (k, v) in attr.iter() {
                    s.push_str(&format!(", {}=\"{}\"", k, v));
                }
            }
            s.push_str("];\n");
        }
        for h in self.hu_dag.iter() {
            match h {
                HuDAG::Path(p) => {
                    for (from, to) in p {
                        s.push_str(&format!("  {} -> {};\n", from, to));
                    }
                },
                HuDAG::Single(a) => {
                    s.push_str(&format!("  {};\n", a));
                }
            }
        }
        // use cluster for subgraphs
        if cluster {
            s.push_str(&self.cluster_notation());
        }
        
        s.push_str("}\n");
        s
    }

    pub fn cluster_notation(&self) -> String {
        let mut s = String::new();
        for c in self.clusters.iter() {
            s.push_str(format!("  subgraph {} {{\n", c.name).as_str());
            for n in c.nodes.iter() {
                s.push_str(&format!("    {};\n", n));
            }
            s.push_str("  }\n");
        }
        s
    }
}