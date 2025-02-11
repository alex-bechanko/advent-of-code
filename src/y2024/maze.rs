/*
Advent of Code solutions written in the Rust programming language
Copyright (C) 2025 Alexander Bechanko

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::Hash;

type Predecessors<T> = HashMap<T, Vec<T>>;
type Distances<T> = HashMap<T, u32>;
pub trait Maze {
    type Node: Hash + Eq + Ord + Copy + Clone + std::fmt::Debug;
    fn neighbors(&self, n: &Self::Node) -> Vec<(u32, Self::Node)>;

    fn dijkstra(&self, source: &Self::Node) -> (Distances<Self::Node>, Predecessors<Self::Node>) {
        use std::cmp::Ordering::{Equal, Greater, Less};

        let mut dist = HashMap::new();
        dist.insert(*source, 0);
        let mut prev = HashMap::new();

        let mut pq: BinaryHeap<Reverse<(u32, Self::Node)>> = BinaryHeap::new();
        pq.push(Reverse((0, *source)));

        while let Some(Reverse((current_distance, current_node))) = pq.pop() {
            if current_distance > *dist.get(&current_node).unwrap_or(&u32::MAX) {
                continue;
            }

            let ns: Vec<Reverse<(u32, Self::Node)>> = self
                .neighbors(&current_node)
                .into_iter()
                .filter_map(|(neighbor_distance, neighbor_node)| {
                    let alternate_distance = current_distance + neighbor_distance;
                    let shortest_distance = dist.get(&neighbor_node).unwrap_or(&u32::MAX);
                    match alternate_distance.cmp(shortest_distance) {
                        Greater => return None,
                        Less => {
                            dist.insert(neighbor_node, alternate_distance);
                            prev.insert(neighbor_node, vec![current_node]);
                        }
                        Equal => {
                            if let Some(ps) = prev.get_mut(&neighbor_node) {
                                if !ps.contains(&current_node) {
                                    ps.push(current_node);
                                }
                            }
                        }
                    }

                    Some(Reverse((alternate_distance, neighbor_node)))
                })
                .collect();

            pq.extend(ns);
        }

        (dist, prev)
    }

    fn shortest_paths(
        &self,
        dist: &Distances<Self::Node>,
        prev: &Predecessors<Self::Node>,
        start: &Self::Node,
        end: &Self::Node,
    ) -> Vec<Vec<Self::Node>> {
        let mut shortest = vec![];

        let mut stack: Vec<(Vec<Self::Node>, &Self::Node)> = vec![];
        if !dist.contains_key(end) {
            return shortest;
        }

        stack.push((vec![*end], end));

        while let Some((current_path, current_node)) = stack.pop() {
            if current_node == start {
                let mut current_path: Vec<Self::Node> = current_path.clone();
                current_path.reverse();
                shortest.push(current_path);
            } else if let Some(previous_nodes) = prev.get(current_node) {
                for n in previous_nodes {
                    let mut path = current_path.clone();
                    path.push(*n);
                    stack.push((path, n));
                }
            }
        }

        shortest
    }
}
