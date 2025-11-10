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

use std::hash::Hash;

use std::collections::{HashMap, HashSet};

pub trait Graph {
    type Vertex: Hash;

    fn contains_edge(&self, v1: &Self::Vertex, v2: &Self::Vertex) -> bool;
    fn neighbors(&self, v: &Self::Vertex) -> Vec<Self::Vertex>;
    fn neighbors_set(&self, v: &Self::Vertex) -> HashSet<Self::Vertex>;
    fn vertices(&self) -> Vec<Self::Vertex>;
}

#[derive(Debug, Clone)]
pub struct AdjacencyList<V> {
    contents: HashMap<V, HashSet<V>>,
}

impl<V> AdjacencyList<V>
where
    V: Eq + Ord + Hash + Clone,
{
    pub fn new() -> AdjacencyList<V> {
        AdjacencyList {
            contents: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, v: V) {
        self.contents.entry(v).or_default();
    }

    pub fn add_edge(&mut self, v1: V, v2: V) {
        self.contents
            .entry(v1.clone())
            .or_default()
            .insert(v2.clone());
        self.contents.entry(v2).or_default().insert(v1);
    }
}

impl<V> Default for AdjacencyList<V>
where
    V: Eq + Ord + Hash + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<V> Graph for AdjacencyList<V>
where
    V: Eq + Ord + Hash + Clone,
{
    type Vertex = V;
    fn contains_edge(&self, v1: &V, v2: &V) -> bool {
        self.contents.get(v1).is_some_and(|es| es.contains(v2))
    }

    fn neighbors(&self, v: &V) -> Vec<V> {
        let Some(ns) = self.contents.get(v) else {
            return vec![];
        };
        let mut ns: Vec<V> = ns.iter().cloned().collect();
        ns.sort_unstable();
        ns
    }

    fn neighbors_set(&self, v: &V) -> HashSet<V> {
        self.contents.get(v).cloned().unwrap_or_else(HashSet::new)
    }

    fn vertices(&self) -> Vec<V> {
        let mut verts: Vec<V> = self.contents.keys().cloned().collect();
        verts.sort_unstable();
        verts
    }
}

pub struct AdjacencyMatrix<V> {
    max_vertices: usize,
    vertex_map: Vec<Option<V>>,
    connection_map: Vec<bool>,
    vertex_hasher: fn(V) -> usize,
}

impl<V> AdjacencyMatrix<V>
where
    V: Eq + Copy + Hash,
{
    pub fn new(max_vertices: usize, vertex_hasher: fn(V) -> usize) -> AdjacencyMatrix<V> {
        AdjacencyMatrix {
            max_vertices,
            vertex_map: vec![None; max_vertices],
            connection_map: vec![false; max_vertices * max_vertices],
            vertex_hasher,
        }
    }

    pub fn add_vertex(&mut self, vertex: V) {
        let index = (self.vertex_hasher)(vertex);
        self.vertex_map[index] = Some(vertex);
    }

    pub fn add_edge(&mut self, v1: V, v2: V) {
        let i1 = (self.vertex_hasher)(v1);
        let i2 = (self.vertex_hasher)(v2);

        self.vertex_map[i1] = Some(v1);
        self.vertex_map[i2] = Some(v2);

        self.connection_map[i1 * self.max_vertices + i2] = true;
        self.connection_map[i2 * self.max_vertices + i1] = true;
    }
}

impl<V> Graph for AdjacencyMatrix<V>
where
    V: Eq + Hash + Copy,
{
    type Vertex = V;
    fn contains_edge(&self, v1: &V, v2: &V) -> bool {
        let i1 = (self.vertex_hasher)(*v1);
        let i2 = (self.vertex_hasher)(*v2);

        self.connection_map[i1 * self.max_vertices + i2]
    }

    fn neighbors(&self, v: &V) -> Vec<V> {
        let index = (self.vertex_hasher)(*v);

        self.connection_map[index * self.max_vertices..(index + 1) * self.max_vertices]
            .iter()
            .enumerate()
            .filter(|(_, conn)| **conn)
            .map(|(i, _)| self.vertex_map[i].expect("edge found but no vertex found"))
            .collect()
    }

    fn neighbors_set(&self, v: &V) -> HashSet<V> {
        self.neighbors(v).into_iter().collect()
    }

    fn vertices(&self) -> Vec<V> {
        self.vertex_map.iter().filter_map(|v| *v).collect()
    }
}

/// # Panics
pub fn bron_kerbosch<G, V>(
    graph: &G,
    current: &HashSet<V>,
    potential: &mut HashSet<V>,
    excluded: &mut HashSet<V>,
    cliques: &mut Vec<Vec<V>>,
) where
    G: Graph<Vertex = V>,
    V: Eq + Ord + Hash + Clone,
{
    if potential.is_empty() && excluded.is_empty() {
        if current.len() > 2 {
            let mut clique: Vec<V> = current.iter().cloned().collect();
            clique.sort_unstable();
            cliques.push(clique);
        }
        return;
    }

    let pivot = potential
        .union(excluded)
        .max_by_key(|v| graph.neighbors_set(v).len())
        .cloned()
        .expect("potential and excluded are both empty when at least one should be non-empty");

    let pivot_neighbors = graph.neighbors_set(&pivot);
    let candidates: Vec<V> = potential.difference(&pivot_neighbors).cloned().collect();
    for candidate in candidates {
        let mut current2 = current.clone();
        current2.insert(candidate.clone());

        let candidate_neighbors = graph.neighbors_set(&candidate);
        let mut potential2 = potential
            .intersection(&candidate_neighbors)
            .cloned()
            .collect();

        let mut excluded2 = excluded
            .intersection(&candidate_neighbors)
            .cloned()
            .collect();

        bron_kerbosch(graph, &current2, &mut potential2, &mut excluded2, cliques);

        potential.remove(&candidate);
        excluded.insert(candidate);
    }
}

/// # Panics
pub fn max_cliques<G, V>(graph: &G) -> Vec<Vec<V>>
where
    G: Graph<Vertex = V>,
    V: std::cmp::Ord + std::cmp::Eq + std::hash::Hash + Clone,
{
    let mut cliques = vec![];
    let current = HashSet::new();
    let mut potentials = graph.vertices().into_iter().collect();
    let mut excluded = HashSet::new();

    bron_kerbosch(
        graph,
        &current,
        &mut potentials,
        &mut excluded,
        &mut cliques,
    );

    if cliques.is_empty() {
        return vec![];
    }

    let largest_size = cliques
        .iter()
        .map(Vec::len)
        .max()
        .expect("cliques should be non-empty");

    cliques
        .into_iter()
        .filter(|c| c.len() == largest_size)
        .collect()
}
