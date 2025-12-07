//! Path-finding utilities using the A* search algorithm.
//!
//! This module provides helper functions for grid-based pathfinding, using the
//! A* algorithm to find shortest paths while supporting obstacles and grid bounds.
//!
//! The A* algorithm reference: https://en.wikipedia.org/wiki/A*_search_algorithm

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

/// Represents a coordinate on the grid (x, y).
pub type GridNode = (isize, isize);

/// Internal state for the Priority Queue.
///
/// Holds the current position and its associated F-Score (Total estimated cost).
#[derive(Clone, Eq, PartialEq)]
struct State {
    /// f_score = g_score (cost from start) + h_score (heuristic to end).
    /// Lower values are better.
    f_score: u32,
    position: GridNode,
}

// Implement Ord for State to make BinaryHeap a min-heap on f_score.
// Rust's BinaryHeap is a max-heap, so we reverse the comparison on f_score.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f_score
            .cmp(&self.f_score)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Computes the shortest path between two points on a grid using the A* algorithm.
///
/// # Arguments
///
/// * `start` - The starting grid coordinate (x, y).
/// * `end` - The target grid coordinate (x, y).
/// * `blocked_nodes` - A set of coordinates that cannot be traversed.
/// * `bounds` - The width and height of the grid (width, height). Grid is 0-indexed.
///
/// # Returns
///
/// * `Some(Vec<Node>)` - A vector of nodes representing the path from Start to End (inclusive).
/// * `None` - If no path exists.
pub fn a_star_path(
    start: GridNode,
    end: GridNode,
    blocked_nodes: &HashSet<GridNode>,
    bounds: (isize, isize), // (width, height) assuming 0,0 bottom-left
) -> Option<Vec<GridNode>> {
    // If the target itself is blocked, we can't reach it.
    // (Unless we want to path to the *nearest* point, which this fn doesn't do yet).
    if blocked_nodes.contains(&end) {
        return None;
    }

    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<GridNode, GridNode> = HashMap::new();

    // g_score: Cost of getting from Start to node N.
    let mut g_score: HashMap<GridNode, u32> = HashMap::new();

    g_score.insert(start, 0);
    open_set.push(State {
        f_score: heuristic(start, end),
        position: start,
    });

    while let Some(State {
        f_score: _,
        position,
    }) = open_set.pop()
    {
        if position == end {
            return Some(reconstruct_path(came_from, end));
        }

        // Explore neighbors (8-way movement)
        for (neighbor, move_cost) in get_neighbors(position, bounds) {
            if blocked_nodes.contains(&neighbor) {
                continue;
            }

            // Corner Cutting Check (Strict):
            // If moving diagonally (cost 14), ensure we aren't clipping through a wall corner.
            // We check the two cardinal neighbors that form the corner.
            // Example: Moving from (0,0) to (1,1). Check (0,1) and (1,0).
            if move_cost == 14 {
                let dx = neighbor.0 - position.0;
                let dy = neighbor.1 - position.1;
                if blocked_nodes.contains(&(position.0 + dx, position.1))
                    || blocked_nodes.contains(&(position.0, position.1 + dy))
                {
                    continue;
                }
            }

            // Calculate tentative g_score for this neighbor
            let current_g_score = *g_score
                .get(&position)
                .expect("Node in open_set must be in g_score");
            let tentative_g_score = current_g_score + move_cost;

            // If this path to neighbor is better than any previous one, record it!
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                came_from.insert(neighbor, position);
                g_score.insert(neighbor, tentative_g_score);
                open_set.push(State {
                    f_score: tentative_g_score + heuristic(neighbor, end),
                    position: neighbor,
                });
            }
        }
    }

    None
}

/// Converts World Space (floating point) coordinates to Grid Space (integer indices).
///
/// # Arguments
/// * `x`, `y` - World coordinates.
/// * `grid_origin` - The bottom-left corner of the grid in world space.
/// * `cell_size` - The size of one grid cell (square).
pub fn world_to_grid(x: f32, y: f32, grid_origin: (f32, f32), cell_size: f32) -> (isize, isize) {
    let gx = ((x - grid_origin.0) / cell_size).floor() as isize;
    let gy = ((y - grid_origin.1) / cell_size).floor() as isize;
    (gx, gy)
}

/// Converts Grid Space (integer indices) to World Space (floating point).
/// Returns the CENTER of the grid cell.
pub fn grid_to_world(gx: isize, gy: isize, grid_origin: (f32, f32), cell_size: f32) -> (f32, f32) {
    (
        grid_origin.0 + (gx as f32 * cell_size) + (cell_size / 2.0),
        grid_origin.1 + (gy as f32 * cell_size) + (cell_size / 2.0),
    )
}

/// Calculates the Octile Distance heuristic.
/// Used for 8-way movement where diagonals cost more (~1.4x).
fn heuristic(a: GridNode, b: GridNode) -> u32 {
    // Octile Distance
    // cost = 10 * (dx + dy) + (14 - 2 * 10) * min(dx, dy)
    //      = 10 * (dx + dy) - 6 * min(dx, dy)
    // Logic: Move diagonally as much as possible (cost 14), then move straight (cost 10).
    let dx = (a.0 - b.0).abs() as u32;
    let dy = (a.1 - b.1).abs() as u32;
    if dx > dy {
        14 * dy + 10 * (dx - dy)
    } else {
        14 * dx + 10 * (dy - dx)
    }
}

/// Returns valid neighbors for a given node, including 8-way diagonals.
/// Returns a list of `(Node, Cost)`.
fn get_neighbors(node: GridNode, bounds: (isize, isize)) -> Vec<(GridNode, u32)> {
    let (x, y) = node;
    let (w, h) = bounds;
    let mut neighbors = Vec::new();

    // (dx, dy, cost)
    let dirs = [
        // Cardinals (Cost 10)
        (0, 1, 10),
        (0, -1, 10),
        (1, 0, 10),
        (-1, 0, 10),
        // Diagonals (Cost 14)
        (1, 1, 14),
        (1, -1, 14),
        (-1, 1, 14),
        (-1, -1, 14),
    ];

    for (dx, dy, move_cost) in dirs {
        let nx = x + dx;
        let ny = y + dy;
        if nx >= 0 && nx < w && ny >= 0 && ny < h {
            neighbors.push(((nx, ny), move_cost));
        }
    }

    neighbors
}

/// Reconstructs the path by walking backwards from End to Start using the `came_from` map.
fn reconstruct_path(came_from: HashMap<GridNode, GridNode>, current: GridNode) -> Vec<GridNode> {
    let mut total_path = vec![current];
    let mut curr = current;
    while let Some(&prev) = came_from.get(&curr) {
        total_path.push(prev);
        curr = prev;
    }
    total_path.reverse();
    total_path
}

/// Expands a set of blocked nodes to account for an agent's size (Minkowski Sum).
///
/// If an agent occupies multiple grid cells (e.g., 2x2), it cannot stand in a cell
/// if any part of its body would overlap a blocked node.
/// This function returns a new set of blocked nodes that represents the "Configuration Space"
/// for the top-left corner (or origin anchor) of the agent.
///
/// # Arguments
/// * `obstacles` - The original set of blocked single-cell nodes.
/// * `agent_size` - The width and height of the agent in grid cells.
///
/// # Example
/// If there is a wall at (10, 10) and the agent is 2x2:
/// The agent cannot stand at (10, 10)
/// It also cannot stand at (9, 10) (its right side hits the wall).
/// It also cannot stand at (10, 9) (its top side hits the wall).
/// It also cannot stand at (9, 9) (its top-right corner hits the wall).
pub fn inflate_obstacles(
    obstacles: &HashSet<GridNode>,
    agent_size: (usize, usize),
) -> HashSet<GridNode> {
    let mut inflated = HashSet::with_capacity(obstacles.len() * agent_size.0 * agent_size.1);
    let (width, height) = agent_size;

    if width <= 1 && height <= 1 {
        return obstacles.clone();
    }

    for &(wall_x, wall_y) in obstacles {
        for dx in 0..width {
            for dy in 0..height {
                inflated.insert((wall_x - dx as isize, wall_y - dy as isize));
            }
        }
    }
    inflated
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_inflate_obstacles_1x1() {
        let mut obstacles = HashSet::new();
        obstacles.insert((10, 10));
        let inflated = inflate_obstacles(&obstacles, (1, 1));
        assert_eq!(inflated.len(), 1);
        assert!(inflated.contains(&(10, 10)));
    }

    #[test]
    fn test_inflate_obstacles_2x2() {
        let mut obstacles = HashSet::new();
        obstacles.insert((10, 10));
        // Agent is 2x2.
        // Anchor points blocked should be (10,10), (9,10), (10,9), (9,9).
        let inflated = inflate_obstacles(&obstacles, (2, 2));
        assert_eq!(inflated.len(), 4);
        assert!(inflated.contains(&(10, 10)));
        assert!(inflated.contains(&(9, 10)));
        assert!(inflated.contains(&(10, 9)));
        assert!(inflated.contains(&(9, 9)));
    }

    #[test]
    fn test_coordinate_conversion() {
        let origin = (0.0, 0.0);
        let cell_size = 0.5;

        // Test 0,0
        assert_eq!(world_to_grid(0.0, 0.0, origin, cell_size), (0, 0));
        assert_eq!(world_to_grid(0.49, 0.49, origin, cell_size), (0, 0));

        // Test boundary
        assert_eq!(world_to_grid(0.5, 0.5, origin, cell_size), (1, 1));

        // Test Grid to World (centers)
        let (wx, wy) = grid_to_world(0, 0, origin, cell_size);
        assert_eq!(wx, 0.25);
        assert_eq!(wy, 0.25);

        let (wx, wy) = grid_to_world(1, 1, origin, cell_size);
        assert_eq!(wx, 0.75);
        assert_eq!(wy, 0.75);
    }

    #[test]
    fn test_simple_path() {
        let start = (0, 0);
        let end = (2, 0);
        let blocked = HashSet::new();
        let bounds = (10, 10);

        let path = a_star_path(start, end, &blocked, bounds).expect("No path found");
        assert_eq!(path, vec![(0, 0), (1, 0), (2, 0)]);
    }

    #[test]
    fn test_blocked_path() {
        let start = (0, 0);
        let end = (0, 2);
        // Block (0,1) so it has to go around
        let mut blocked = HashSet::new();
        blocked.insert((0, 1));
        let bounds = (10, 10);

        let path = a_star_path(start, end, &blocked, bounds).expect("No path found");
        // With 8-way movement:
        // (0,0) -> (1,1) -> (0,2) ?
        // Wait, corner cut check:
        // Moving (0,0) to (1,1) requires (0,1) and (1,0) to be free.
        // (0,1) is BLOCKED. So (1,1) is NOT reachable from (0,0) diagonally.
        // Path must go: (0,0) -> (1,0) -> (1,1) -> (0,2)
        // (1,1) to (0,2) diagonal? Checks (0,1) and (1,2). (0,1) blocked.
        // So must go (1,1) -> (1,2) -> (0,2).
        // Path: (0,0)->(1,0)->(1,1)->(1,2)->(0,2). Length 5.
        assert_eq!(path.len(), 5);
        assert_eq!(path.first(), Some(&start));
        assert_eq!(path.last(), Some(&end));
    }

    #[test]
    fn test_no_path() {
        let start = (0, 0);
        let end = (5, 5);
        let mut blocked = HashSet::new();
        // Wall off the start
        blocked.insert((0, 1));
        blocked.insert((1, 0));
        // And diagonals
        blocked.insert((1, 1));
        let bounds = (10, 10);

        let path = a_star_path(start, end, &blocked, bounds);
        assert!(path.is_none());
    }

    #[test]
    fn test_u_shape_obstacle() {
        // Layout:
        // y=3  . . .  (Open path)
        // y=2  S | E  (Wall at x=1)
        // y=1  . | .  (Wall at x=1)
        // y=0  . | .  (Wall at x=1)
        //      0 1 2

        let start = (0, 2);
        let end = (2, 2);
        let bounds = (5, 5);

        let mut blocked = HashSet::new();
        blocked.insert((1, 0));
        blocked.insert((1, 1));
        blocked.insert((1, 2)); // Block direct path at y=2

        let path = a_star_path(start, end, &blocked, bounds).expect("Path found");

        // Verify the path went over the wall (y > 2) at x=1
        let crossed_over = path.iter().any(|n| n.0 == 1 && n.1 > 2);
        assert!(crossed_over, "Path should cross over the wall at y > 2");
    }

    #[test]
    fn test_large_open_field() {
        let start = (0, 0);
        let end = (10, 10);
        let blocked = HashSet::new();
        let bounds = (20, 20);

        let path = a_star_path(start, end, &blocked, bounds).expect("Path found");

        // With 8-way movement, the shortest path is a straight diagonal line.
        // (0,0) -> (1,1) -> ... -> (10,10)
        // Length = 10 steps + 1 start node = 11 nodes.
        assert_eq!(path.len(), 11);
        assert_eq!(path.last(), Some(&end));
    }

    #[test]
    fn test_path_to_blocked_target() {
        let start = (0, 0);
        let end = (2, 0);
        let mut blocked = HashSet::new();
        blocked.insert(end);
        let bounds = (10, 10);
        assert!(a_star_path(start, end, &blocked, bounds).is_none());
    }
}
