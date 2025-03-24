use std::fmt::Display;
use std::hash::Hash;

pub mod adjac;
pub mod multidigraph;
// pub mod multidigraphmap;

#[cfg(test)]
mod tests {
    //use crate::multidigraph::{FromNodePath, IntoNodePath};
    use super::*;
    use multidigraph::NodePath;

    #[test]
    fn it_create_agraph() {
        let mut agraph = multidigraph::Multidigraph::new();
        agraph.add_paths(vec![
            &NodePath::new(1, vec![2, 3]),
            &NodePath::new(2, vec![3]),
            &NodePath::new(3, vec![]),
            &NodePath::new(4, vec![1]),
            &NodePath::new(5, vec![4]),
            &NodePath::new(6, vec![5]),
            &NodePath::new(7, vec![6]),
            &NodePath::new(8, vec![7]),
            &NodePath::new(9, vec![8]),
        ]);
        agraph.build_adjac();
        let loops = agraph.check_loops();
        assert_eq!(loops.len(), 0);
        agraph.print_internal();
        let c_dag = agraph.connected_dags;
        println!("{:?}", c_dag);
    }

    #[test]
    fn it_create_agraph_string() {
        let mut agraph = multidigraph::Multidigraph::new();
        agraph.add_paths(vec![
            &NodePath::new("1".to_string(), vec!["2".to_string(), "3".to_string()]),
            &NodePath::new("2".to_string(), vec!["3".to_string()]),
            &NodePath::new("3".to_string(), vec![]),
            &NodePath::new("4".to_string(), vec!["1".to_string()]),
            &NodePath::new("5".to_string(), vec!["4".to_string()]),
            &NodePath::new("6".to_string(), vec!["5".to_string()]),
            &NodePath::new("7".to_string(), vec!["6".to_string()]),
            &NodePath::new("8".to_string(), vec!["7".to_string()]),
            &NodePath::new("9".to_string(), vec!["8".to_string()]),
            &NodePath::new("X".to_string(), vec![]),
        ]);
        agraph.build_adjac();
        let loops = agraph.check_loops();
        assert_eq!(loops.len(), 0);
        agraph.print_internal();
        let c_dag = &agraph.connected_dags;
        println!("{:?}", c_dag);
        let h_dag = agraph.hu_connected_dags();
        println!("{:?}", h_dag);
    }

    /*
    fn it_create_graph() {
        struct MyNodePath {
            node_name: String,
            transition_to: Vec<String>,
            calc_weight: f64,
            map: HashMap<String, usize>,
        }
        impl IntoNodePath<String> for MyNodePath {
            fn into_node_path(&self) -> NodePath<String> {
                NodePath::new(self.node_name.clone(), self.transition_to.clone())
                self.map.insert(self.node_name.clone(), 1);
            }
        }
        impl FromNodePath<String> for MyNodePath {
            fn from_node_path(node_path: &NodePath<String>) -> Self {
                MyNodePath {
                    node_name: node_path.node.clone(),
                    transition_to: node_path.edges.clone(),
                    calc_weight: 0.0,
                }
            }
        }
    }
     */

}
