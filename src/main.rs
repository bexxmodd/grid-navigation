mod grid_navigation;

use std::{collections::HashSet, mem};
use grid_navigation::{Vertex, Edge, Graph, GraphNavigation};

fn main() {
    let mut v_ = HashSet::<Vertex>::new();
    v_.insert(Vertex::new(0, 0));
    v_.insert(Vertex::new(0, 1));
    v_.insert(Vertex::from((1, 0)));
    v_.insert((1, 1).into());

    let mut e_ = HashSet::<Edge>::new();
    e_.insert(Edge { from: (0, 0).into(), to: (0, 1).into() });
    e_.insert(Edge { from: (0, 1).into(), to: (1, 1).into() });
    e_.insert(Edge { from: Vertex::new(1, 1), to: Vertex::new(1, 0) });
    
    let graph = Graph::new(v_, e_);
    println!("Path: {:?}", graph.bfs((0, 0).into(), (1, 1).into()));
    println!("char: {}", mem::size_of::<char>());
    println!("u8: {}", mem::size_of::<u8>());
    println!("i128: {}", mem::size_of::<i128>());
    println!("size: {}", mem::size_of::<(char, u8, i128)>());
    println!("align: {}", mem::align_of::<(char, u8, i128)>());
}
