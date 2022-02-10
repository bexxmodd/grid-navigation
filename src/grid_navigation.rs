use std::cmp::Reverse;
use std::collections::{HashMap, VecDeque, BinaryHeap};
use priq::PriorityQueue;

#[derive(PartialEq, Eq, Hash, Clone, Debug, Ord, PartialOrd)]
pub struct Location(usize, usize);

pub struct WeighterGrid {
    rows: usize,
    columns: usize,
    weights: HashMap<Location, f32>,
}

impl WeighterGrid {
    pub fn new(rows: usize, columns: usize) -> Self {
        WeighterGrid { rows, columns, weights: HashMap::new() }
    }

    pub fn cost(&self, from: &Location, to: &Location) -> f32 {
        *self.weights.get(to).unwrap_or(&1.0)
    }

    pub fn neighbors(&self, loc: &Location) -> Vec<Location> {
        let x = loc.0;
        let y = loc.1;
        let neighbors = vec![(x + 1, y), (x - 1, y), (x, y - 1), (x, y + 1)];
        neighbors.iter()
                 .filter(|(x, y)| self.in_bounds(&Location(*x, *y))
                     && self.passable(&Location(*x, *y)))
                 .map(|(x, y)| Location(*x, *y))
                 .collect()
    }

    pub fn reconstruct_path(came_from: &HashMap<Location, Option<Location>>,
        start: &Location, goal: &Location) -> VecDeque<Location> {
        let mut current = goal;
        let mut path = VecDeque::new();

        while current != start {
            let key = &came_from[current];
            if let Some(k) = key {
                current = k;
            } else {
                return path
            }
            path.push_front(current.clone());
        }
        path
    }

    pub fn heuristic(from: &Location, to: &Location) -> f32 {
        (from.0 as f32 - to.0 as f32).abs()
            + (from.1 as f32 - to.1 as f32).abs()
    }

    fn in_bounds(&self, loc: &Location) -> bool {
        loc.0 < self.rows && loc.1 < self.columns
    }

    fn passable(&self, loc: &Location) -> bool {
        // TODO: add Grid member to the struct
       true 
    }
}

pub fn a_star_search(grid: &WeighterGrid, start: &Location, goal: &Location)
    -> VecDeque<Location> {
        let mut frontier: PriorityQueue<f32, Location> = PriorityQueue::new();
        frontier.put(0.0, start.clone());
        let mut came_from: HashMap<Location, Option<Location>> = HashMap::new();
        let mut gscore_so_far: HashMap<Location, f32> = HashMap::new();
        came_from.insert(start.clone(), None); 

        while frontier.is_empty() {
            let current = frontier.pop().unwrap();
            if current.1 == *goal {
                return WeighterGrid::reconstruct_path(&came_from, &start, &goal);
            }

            for neighbor in grid.neighbors(&current.1) {
                let tent_gscore = gscore_so_far[&current.1]
                    + grid.cost(&current.1, &neighbor);
                if gscore_so_far.get(&neighbor).is_none()
                    || tent_gscore < gscore_so_far[&neighbor] {
                    gscore_so_far.insert(neighbor.clone(), tent_gscore);
                    frontier.put(tent_gscore, neighbor.clone());
                }
            }
        }

        VecDeque::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_cost_simple() {
        let grid = WeighterGrid::new(5, 5);
        assert_eq!(1.0, grid.cost(&Location(2, 2), &Location(3, 2)));
    }

    #[test]
    fn reconstruct_path() {
        let mut path = HashMap::new();
        path.insert(Location(0, 1), Some(Location(0, 0)));
        let mut expected = VecDeque::new();
        expected.push_front(Location(0, 0));
        assert_eq!(expected, WeighterGrid::reconstruct_path(
                &path, &Location(0, 0), &Location(0, 1)));
    }

    #[test]
    fn heuristic_md() {
        assert_eq!(2.0,
            WeighterGrid::heuristic(&Location(1, 1), &Location(2, 2)));
    }
}
