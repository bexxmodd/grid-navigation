use std::collections::{HashMap, VecDeque, BinaryHeap};

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

    pub fn reconstruct_path(came_from: &HashMap<Location, Location>,
        start: &Location, goal: &Location) -> VecDeque<Location> {
        let mut current = goal;
        let mut path = VecDeque::new();
        while current != start {
            current = &came_from[current];
            path.push_front(current.clone());
        }
        path
    }

    pub fn heuristic(from: &Location, to: &Location) -> f32 {
        (from.0 as f32 - to.0 as f32).abs()
            + (from.1 as f32 - to.1 as f32).abs()
    }
}

pub fn a_star_search(grid: &WeighterGrid, start: &Location, goal: &Location)
    -> VecDeque<Location> {
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
        path.insert(Location(0, 1), Location(0, 0));
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
