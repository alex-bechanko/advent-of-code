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

fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .chars()
        .map(|x| {
            let x = x.to_digit(10).unwrap();
            usize::try_from(x).unwrap()
        })
        .collect()
}

fn to_disk(disk_map: &[usize]) -> Vec<Option<usize>> {
    let size: usize = disk_map.iter().sum();

    let mut disk = vec![None; size];
    let mut cursor = 0;
    let mut file_id = 0;

    for (n, &blocks) in disk_map.iter().enumerate() {
        if n % 2 == 1 {
            // odd number disk map entries are free space entries
            // since disk starts as free space, there's
            // nothing to do.
            cursor += blocks;
            continue;
        }

        for blk in disk.iter_mut().skip(cursor).take(blocks) {
            *blk = Some(file_id);
        }

        file_id += 1;
        cursor += blocks;
    }

    disk
}

fn checksum(disk: &[Option<usize>]) -> usize {
    disk.iter()
        .enumerate()
        .filter_map(|(pos, &file_id)| {
            let f = file_id?;
            Some(pos * f)
        })
        .sum()
}

#[allow(dead_code)]
fn debug_disk(disk: &[Option<usize>]) -> String {
    let mut s = String::with_capacity(disk.len() * 2 + 1);
    for block in disk {
        s.push('|');
        match block {
            Some(id) => {
                s.push_str(id.to_string().as_str());
            }
            None => {
                s.push('.');
            }
        };
    }
    s.push('|');

    s
}

#[allow(dead_code)]
fn debug_diskmap(disk_map: &[usize]) -> String {
    let mut s = String::new();

    for space in disk_map {
        s.push_str(&space.to_string());
    }

    s
}

pub fn part1(input: &str) -> usize {
    let disk_map = parse(input);

    let mut disk = to_disk(&disk_map);
    let mut start = 0;
    let mut end = disk.len() - 1;
    while start < disk.len() - 1 && end > 0 && start != end {
        if disk[start].is_none() && disk[end].is_some() {
            disk[start] = disk[end];
            disk[end] = None;
        } else if disk[start].is_some() {
            start += 1;
        } else if disk[end].is_none() {
            end -= 1;
        }
    }
    checksum(&disk)
}

pub fn part2(input: &str) -> usize {
    let mut free_space_map = parse(input);
    let file_size_map = free_space_map.clone();
    let mut disk = to_disk(&free_space_map);

    let mut file_index = disk.len();
    for i in (0..free_space_map.len()).rev() {
        let blocks = file_size_map[i];
        file_index -= blocks;

        if i % 2 == 1 {
            continue;
        }

        let file = disk[file_index];
        let mut space_index = 0;
        for j in 0..i {
            let space = free_space_map[j];

            if j % 2 == 1 && space >= blocks {
                for k in 0..blocks {
                    disk[space_index + k] = file;
                    disk[file_index + k] = None;
                }

                free_space_map[j] -= blocks;
                // this is a bit of a hack, but add the blocks back to the previous file descriptor
                // this way we don't need to insert into the disk_map
                free_space_map[j - 1] += blocks;
                break;
            }

            space_index += space;
        }
    }

    checksum(&disk)
}
