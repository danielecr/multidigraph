# Multidigraph

Calc multidigraph given an array of Multidigraph::NodePath<T>

It provides method to check if a node is part of any Digraph of the forest defined
by the multipath

Where T: PartialOrd + Clone + std::fmt::Display


## Code Example

```rust
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
    println!("{}", agraph.dot_notation());
```

prints:

~~~

Some([Path([(8, 7), (7, 6), (6, 5), (5, 4), (4, 3), (3, 0), (0, 1), (1, 2), (0, 2)]), Single(9)])
[Path([("9", "8"), ("8", "7"), ("7", "6"), ("6", "5"), ("5", "4"), ("4", "1"), ("1", "2"), ("2", "3"), ("1", "3")]), Single("X")]
digraph G {
  9 -> 8;
  8 -> 7;
  7 -> 6;
  6 -> 5;
  5 -> 4;
  4 -> 1;
  1 -> 2;
  2 -> 3;
  1 -> 3;
  X;
}
~~~

hu_connected_dags() stands for HUman readable / ready

If there are loops are emitted

## Traits

multidigraph::NodePathTrait

## Future release

Evaluate in node weight for each internal nodes