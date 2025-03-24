
//pub mod ha_graph;
pub mod adjac;
pub mod multidigraph;
pub mod multidigraphmap;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use multidigraph::NodePath;
    use petgraph::graph::Node;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_create_agraph() {
        let mut agraph = multidigraph::Multidigraph::new();
        agraph.add_paths(vec![
            NodePath::new(1, vec![2, 3]),
            NodePath::new(2, vec![3]),
            NodePath::new(3, vec![]),
            NodePath::new(4, vec![1]),
            NodePath::new(5, vec![4]),
            NodePath::new(6, vec![5]),
            NodePath::new(7, vec![6]),
            NodePath::new(8, vec![7]),
            NodePath::new(9, vec![8]),
        ]);
        agraph.build_adjac();
        let loops = agraph.check_loops();
        assert_eq!(loops.len(), 0);
        agraph.print_internal();
    }
}
