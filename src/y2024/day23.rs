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

use crate::y2024::graph::{max_cliques, AdjacencyList, Graph};

fn network_to_adjacency_list(input: &str) -> AdjacencyList<&str> {
    input.lines().filter_map(|l| l.split_once('-')).fold(
        AdjacencyList::new(),
        |mut graph, (a, b)| {
            graph.add_edge(a, b);
            graph
        },
    )
}

pub fn part1(input: &str) -> usize {
    let graph: AdjacencyList<&str> = network_to_adjacency_list(input);

    let mut count = 0usize;
    for u in graph.vertices() {
        for v in graph.neighbors(&u).into_iter().filter(|&v| v > u) {
            for w in graph.neighbors(&v).into_iter().filter(|&w| w > v) {
                if graph.contains_edge(&u, &w)
                    && (u.starts_with('t') || v.starts_with('t') || w.starts_with('t'))
                {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn part2(input: &str) -> String {
    let graph = network_to_adjacency_list(input);
    let cliques = max_cliques(&graph);
    cliques[0].join(",")
}
