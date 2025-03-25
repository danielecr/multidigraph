use std::fmt::Display;

use petgraph::graph::Graph;
use petgraph::graph::Node;
use fixedbitset::FixedBitSet;
use petgraph::visit::EdgeRef;

#[derive(Debug)]
pub struct Adjac<T> where 
  T: PartialOrd + Clone {
  bitset: FixedBitSet,
  rn: Vec<Node<T>>,
  size: usize,
}

#[derive(Debug, Clone)]
pub enum MyDAG {
  Path(Vec<(usize,usize)>),
  Single(usize),
}

#[derive(Debug, Clone)]
pub enum HuDAG<T> {
  Path(Vec<(T,T)>),
  Single(T),
}

impl<T> Adjac<T> where T: PartialOrd + Clone + Display{

  pub fn new_from_graph(g: &Graph<T,()>) -> Self {
    let size = g.node_count();
    let mut bitset = FixedBitSet::with_capacity(size*size);
    for edge in g.edge_references() {
      let (a, b) = (edge.source().index(), edge.target().index());
      bitset.insert(a*size + b);
    }
    Adjac { bitset, rn: g.raw_nodes().to_vec(), size }
  }

  pub fn new(f: &FixedBitSet, rn: &[Node<T>]) -> Self {
    let size = rn.len();
    Adjac { bitset: f.clone(), rn: rn.to_vec(), size }
  }

  pub fn getsize(&self) -> usize {
    self.size
  }

  pub fn contains(&self, x: usize, y: usize) -> bool {
    self.bitset.contains(x*self.size + y)
  }
  
  /// returns the list of loops
  /// digraph should not contains loops
  pub fn check_loops(&self) -> Vec<Vec<T>> {
    let mut loops = Vec::new();
    let mut visited = FixedBitSet::with_capacity(self.size);
    for i in 0..self.size {
      if !visited.contains(i) {
        let mut loop_nodes = Vec::new();
        self.dfs_loop(i, i, &mut visited, &mut loop_nodes);
        if !loop_nodes.is_empty() {
          loops.push(loop_nodes.iter().map(|i| self.rn[*i].weight.clone()).collect());
        }
      }
    }
    loops
  }

  fn dfs_loop(&self, start: usize, node: usize, visited: &mut FixedBitSet, loop_nodes: &mut Vec<usize>) {
    visited.insert(node);
    for i in 0..self.size {
      if self.contains(node, i) {
        if i == start {
          loop_nodes.push(i);
        } else if !visited.contains(i) {
          self.dfs_loop(start, i, visited, loop_nodes);
        }
      }
    }
  }

  pub fn matching_dags(dags: Vec<Vec<(T,T)>>, node: &T) -> Vec<Vec<(T,T)>> {
    dags.into_iter().filter(|dag|{
      dag.iter().any(|(from, to)| from == node || to == node)
    }).collect::<Vec<Vec<(T,T)>>>()
  }
  
  pub fn matching_mdags(dags: Vec<MyDAG>, node: usize) -> Vec<MyDAG> {
    dags.into_iter().filter(|dag|{
      match dag {
        MyDAG::Path(p) => p.iter().any(|(from, to)| *from == node || *to == node),
        MyDAG::Single(a) => *a == node,
      }
    }).collect::<Vec<MyDAG>>()
  }
  
  #[allow(unused)]
  pub fn printit(&self) {
    for x in b'A'..=b'r' { //(self.size-1) {
      print!("{}",x as char);
    }
    println!();
    let mut letter = b'A';
    for x in 0..self.size {
      for y in 0..self.size {
        if self.contains(x,y) {
          print!("1");
        } else {
          print!("0");
        }
      }
      println!(" {}", letter as char);
      letter += 1;
    }
    println!();
  }
  
  /// returns the nodes that has no incoming edges
  /// these are the starting nodes
  pub fn select_starting_nodes(&self) -> Vec<usize> {
    let mut v = vec![];
    for y in 0..self.size {
      let mut has1 = false;
      for x in 0..self.size {
        if self.contains(x,y) {
          has1 = true
        }
      }
      if ! has1 {
        v.push(y);
      }
    }
    v
  }
  
  /// given a list of path, returns the list of path which contains node
  pub fn path_including_node(&self, dags: &Vec<MyDAG>, node: &T) -> Vec<Vec<(T,Option<T>)>> {
    // find the nodes which contains node
    let node_node = self.rn.iter().position(|n| n.weight == *node).unwrap();
    let mut v = vec![];
    for dag in dags {
      match dag {
        MyDAG::Path(p) => {
          // search for node in p
          if p.iter().any(|(from, to)| *from == node_node || *to == node_node) {
            let mut v2 = vec![];
            for (from, to) in p {
              v2.push((self.rn[*from].weight.clone(), Some(self.rn[*to].weight.clone())));
            }
            v.push(v2);
          }
        },
        MyDAG::Single(a) => {
          if self.rn[*a].weight == *node {
            v.push(vec![(self.rn[*a].weight.clone(), None)]);
          }
        }
      }
    }
    v
  }
  
  pub fn connected_dags(&self) -> Vec<MyDAG> {
    // given the adjacency matrix for each column which has no setted bit
    // do: dfs(on that node)
    let startings = self.select_starting_nodes();
    let mut sub_dags = Vec::new();
    
    for node in startings.iter() {
      let mut visited = vec![false; self.size];
      let mut dag = Vec::new();
      self.dfs( *node, &mut visited, &mut dag);
      if !dag.is_empty() {
        sub_dags.push(MyDAG::Path(dag));
      } else {
        sub_dags.push(MyDAG::Single(*node));
      }
    }
    
    sub_dags
  }

  fn to_human(&self, md: MyDAG) -> HuDAG<T> {
    match md {
      MyDAG::Path(p) => {
        let mut v = Vec::new();
        for (from, to) in p {
          v.push((self.rn[from].weight.clone(), self.rn[to].weight.clone()));
        }
        HuDAG::Path(v)
      },
      MyDAG::Single(a) => HuDAG::Single(self.rn[a].weight.clone()),
    }
  }

  pub fn hu_connected_dags(&self) -> Vec<HuDAG<T>> {
    // given the adjacency matrix for each column which has no setted bit
    // do: dfs(on that node)
    let startings = self.select_starting_nodes();
    let mut sub_dags = Vec::new();
    
    for node in startings.iter() {
      let mut visited = vec![false; self.size];
      let mut dag = Vec::new();
      self.dfs( *node, &mut visited, &mut dag);
      if !dag.is_empty() {
        sub_dags.push(self.to_human(MyDAG::Path(dag)));
      } else {
        sub_dags.push(self.to_human(MyDAG::Single(*node)));
      }
    }
    
    sub_dags
  }

  pub fn dot_notation(&self) -> String {
    let hu = self.hu_connected_dags();
    let mut s = String::new();
    s.push_str("digraph G {\n");
    for h in hu.iter() {
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

  fn dfs(&self, node: usize, visited: &mut [bool], dag: &mut Vec<(usize, usize)>) {
    let len = self.getsize();
    visited[node] = true;
    
    for i in 0..len {
      if self.contains(node, i) {
        dag.push((node, i));
        if !visited[i] {
          self.dfs(i, visited, dag);
        }
      }
    }
  }
}