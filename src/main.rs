mod grid_navigation;

use std::collections::HashSet;
use grid_navigation::{Vertex, Edge, Graph, GraphNavigation};

fn main() {
    let mut v_ = HashSet::<Vertex>::new();
    v_.insert(Vertex::new(0, 0));
    v_.insert(Vertex::new(0, 1));
    v_.insert(Vertex::new(1, 0));
    v_.insert(Vertex::new(1, 1));
    let mut e_ = HashSet::<Edge>::new();
    e_.insert(Edge { from: Vertex::new(0, 0), to: Vertex::new(0, 1) });
    e_.insert(Edge { from: Vertex::new(0, 1), to: Vertex::new(1, 1) });
    e_.insert(Edge { from: Vertex::new(1, 1), to: Vertex::new(1, 0) });
    
    let graph = Graph::new(v_, e_);
    println!("Path: {:?}", graph.bfs(Vertex::new(0, 0), Vertex::new(1, 1)));
}
