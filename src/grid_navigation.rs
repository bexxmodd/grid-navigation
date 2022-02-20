use std::collections::{HashSet, HashMap, VecDeque};

#[allow(unused_imports)]
use priq::PriorityQueue;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct Vertex {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Vertex {
    fn from(tup: (usize, usize)) -> Self {
        Vertex { x: tup.0, y: tup.1  }
    }
}

impl Vertex {
    pub fn new(x: usize, y: usize) -> Self {
        Vertex { x, y }
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct Edge { 
    pub from: Vertex, 
    pub to: Vertex
}

pub struct Graph {
    pub vertices: HashSet<Vertex>,
    pub edges: HashSet<Edge>,
}

pub trait GraphNavigation {
    fn neighbors(&self, vertex: Vertex) -> HashSet<Vertex>;
    fn get_valid_edges(&self, vertex: Vertex) -> Vec<Vertex>;
    fn backtrace(&self, came_from: &mut HashMap<Vertex, Vertex>,
        start: Vertex, end: Vertex) -> Vec<Vertex>;

    fn bfs(&self, start: Vertex, goal: Vertex) -> Option<Vec<Vertex>> {
        let mut came_from = HashMap::<Vertex, Vertex>::new();
        let mut visisted = HashSet::<Vertex>::new();
        let mut frontier = VecDeque::<Vertex>::new();
        frontier.push_front(start);

        loop {
            if frontier.is_empty() {
                    return None
                }
            let current = frontier.pop_back().unwrap();
            if current == goal {
                return Some(self.backtrace(&mut came_from, start, goal))
            }

            for neighbor in self.neighbors(current) {
                if !visisted.contains(&neighbor) {
                    came_from.insert(neighbor, current);
                    frontier.push_front(neighbor);
                    visisted.insert(neighbor);
                }
            }
        }
    }
}

impl Graph {
    pub fn new(vertices: HashSet<Vertex>, edges: HashSet<Edge>) -> Self {
        Graph { vertices, edges }
    }
}


impl GraphNavigation for Graph {
    fn backtrace(&self, came_from: &mut HashMap<Vertex, Vertex>,
        start: Vertex, end: Vertex) -> Vec<Vertex> {
        let mut path = vec![end];
        while path[path.len() - 1] != start {
            let cur_ = came_from.remove(&path[path.len() - 1]).unwrap();
            path.push(cur_);
        }
        path.reverse();
        path
    }

    fn neighbors(&self, vertex: Vertex) -> HashSet<Vertex> {
        let neighs_ = self.get_valid_edges(vertex);
        neighs_.into_iter()
               .filter(|v| self.edges.contains(&Edge { from: vertex, to: *v }))
               .collect()
    }

    fn get_valid_edges(&self, vertex: Vertex) -> Vec<Vertex> {
        let x = vertex.x;
        let y = vertex.y;
        let mut res = vec![ Vertex::new(x, y + 1), Vertex::new(x + 1, y)];
        if x > 0 { res.push(Vertex::new(x - 1, y)) };
        if y > 0 { res.push(Vertex::new(x, y - 1)) };
        res
    }
}
