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

use std::collections::HashSet;

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10).and_then(|d| d.try_into().ok()))
                .collect()
        })
        .collect()
}

fn trailhead_score_part(grid: &Vec<Vec<usize>>, row: usize, col: usize) -> HashSet<(usize, usize)> {
    if grid[row][col] == 9 {
        let mut peaks = HashSet::new();
        peaks.insert((row, col));
        return peaks;
    }

    let elevation = grid[row][col];
    let mut peaks = HashSet::new();
    if row > 0 && elevation + 1 == grid[row - 1][col] {
        peaks.extend(trailhead_score_part(grid, row - 1, col));
    }

    if row + 1 < grid[0].len() && elevation + 1 == grid[row + 1][col] {
        peaks.extend(trailhead_score_part(grid, row + 1, col));
    }

    if col > 0 && elevation + 1 == grid[row][col - 1] {
        peaks.extend(trailhead_score_part(grid, row, col - 1));
    }

    if col + 1 < grid[0].len() && elevation + 1 == grid[row][col + 1] {
        peaks.extend(trailhead_score_part(grid, row, col + 1));
    }

    peaks
}

fn trailhead_score(grid: &Vec<Vec<usize>>, start_row: usize, start_col: usize) -> usize {
    trailhead_score_part(grid, start_row, start_col).len()
}

pub fn part1(input: &str) -> usize {
    let grid = parse(input);
    let width = grid[0].len();
    let height = grid.len();

    let mut total = 0;
    for row in 0..height {
        for col in 0..width {
            if grid[row][col] == 0 {
                total += trailhead_score(&grid, row, col);
            }
        }
    }

    total
}

fn trailhead_rating(grid: &Vec<Vec<usize>>, row: usize, col: usize) -> usize {
    if grid[row][col] == 9 {
        return 1;
    }

    let elevation = grid[row][col];
    let mut score = 0;
    if row > 0 && elevation + 1 == grid[row - 1][col] {
        score += trailhead_rating(grid, row - 1, col);
    }

    if row + 1 < grid[0].len() && elevation + 1 == grid[row + 1][col] {
        score += trailhead_rating(grid, row + 1, col);
    }

    if col > 0 && elevation + 1 == grid[row][col - 1] {
        score += trailhead_rating(grid, row, col - 1);
    }

    if col + 1 < grid[0].len() && elevation + 1 == grid[row][col + 1] {
        score += trailhead_rating(grid, row, col + 1);
    }

    score
}

pub fn part2(input: &str) -> usize {
    let grid = parse(input);
    let width = grid[0].len();
    let height = grid.len();

    let mut total = 0;
    for row in 0..height {
        for col in 0..width {
            if grid[row][col] == 0 {
                total += trailhead_rating(&grid, row, col);
            }
        }
    }

    total
}
