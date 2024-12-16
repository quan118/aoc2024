use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day12/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day12/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u64> {
  let (map, nrow, ncol) = read_map(file)?;
  let mut visited: HashSet<usize> = HashSet::new();
  let mut result: u64 = 0;

  for r in 0..nrow {
    for c in 0..ncol {
      let idx = r * ncol + c;
      if visited.contains(&idx) {
        continue;
      }
      result += get_region_price(&map, nrow, ncol, idx, &mut visited);
    }
  }

  Ok(result)
}

fn part2(file: &str) -> io::Result<u64> {
  let (map, nrow, ncol) = read_map(file)?;

  let mut visited: HashSet<usize> = HashSet::new();
  let mut result: u64 = 0;

  for r in 0..nrow {
    for c in 0..ncol {
      let idx = r * ncol + c;
      if visited.contains(&idx) {
        continue;
      }
      result += get_region_price2(&map, nrow, ncol, idx, &mut visited);
    }
  }

  Ok(result)
}

fn get_region_price(
  map: &Vec<char>,
  nrow: usize,
  ncol: usize,
  idx: usize,
  visited: &mut HashSet<usize>,
) -> u64 {
  let mut queue: VecDeque<usize> = VecDeque::new();
  queue.push_back(idx);
  visited.insert(idx);

  let mut area: u64 = 0;
  let mut perimeter: u64 = 0;

  while !queue.is_empty() {
    let pos = queue.pop_front().unwrap();
    area += 1;

    let mut neighbors: Vec<usize> = get_neighbors(&map, nrow, ncol, pos);

    perimeter += 4 - neighbors.len() as u64;

    neighbors = neighbors
      .into_iter()
      .filter(|&pos| !visited.contains(&pos))
      .collect();

    for neighbor in neighbors {
      visited.insert(neighbor);
      queue.push_back(neighbor);
    }
  }

  area * perimeter
}

fn get_region_price2(
  map: &Vec<char>,
  nrow: usize,
  ncol: usize,
  idx: usize,
  visited: &mut HashSet<usize>,
) -> u64 {
  let mut queue: VecDeque<usize> = VecDeque::new();
  queue.push_back(idx);
  visited.insert(idx);

  let mut area: u64 = 0;
  let mut perimeter: u64 = 0;

  while !queue.is_empty() {
    let pos = queue.pop_front().unwrap();
    area += 1;

    let mut neighbors: Vec<usize> = get_neighbors(&map, nrow, ncol, pos);

    if neighbors.len() == 0 {
      perimeter += 4;
    } else if neighbors.len() == 1 {
      perimeter += 2;
    } else if neighbors.len() == 2 {
      let row0 = neighbors[0] / ncol;
      let col0 = neighbors[0] % ncol;
      let row1 = neighbors[1] / ncol;
      let col1 = neighbors[1] % ncol;

      if row0 != row1 && col0 != col1 {
        perimeter += 1;
      }

      let idx01 = row0 * ncol + col1;
      let idx10 = row1 * ncol + col0;

      if map[idx01] != map[idx10] {
        perimeter += 1;
      }
    } else if neighbors.len() >= 3 {
      let l_shape_counter = count_L_shape(&map, nrow, ncol, pos);
      perimeter += count_L_shape(&map, nrow, ncol, pos);
    }
    neighbors = neighbors
      .into_iter()
      .filter(|&pos| !visited.contains(&pos))
      .collect();

    for neighbor in neighbors {
      visited.insert(neighbor);
      queue.push_back(neighbor);
    }
  }
  let t = area * perimeter;
  area * perimeter
}

fn get_neighbors(map: &Vec<char>, nrow: usize, ncol: usize, cur: usize) -> Vec<usize> {
  let mut neighbors = Vec::new();
  let row = cur / ncol;
  let col = cur % ncol;

  // Up
  if row > 0 {
    neighbors.push(cur - ncol);
  }
  // Down
  if row < nrow - 1 {
    neighbors.push(cur + ncol);
  }
  // Left
  if col > 0 {
    neighbors.push(cur - 1);
  }
  // Right
  if col < ncol - 1 {
    neighbors.push(cur + 1);
  }

  neighbors = neighbors
    .into_iter()
    .filter(|&pos| map[pos] == map[cur])
    .collect();

  neighbors
}

fn count_L_shape(map: &Vec<char>, nrow: usize, ncol: usize, center: usize) -> u64 {
  let mut result: u64 = 0;

  let row = center / ncol;
  let col = center % ncol;
  let drow: [i32; 2] = [-1, 1];
  let dcol: [i32; 2] = [-1, 1];

  for dr in 0..2 {
    for dc in 0..2 {
      let corner_row = row as i32 + drow[dr];
      let corner_col = col as i32 + dcol[dc];
      if corner_row < 0 || corner_col < 0 || corner_row >= nrow as i32 || corner_col >= ncol as i32
      {
        continue;
      }
      let corner_idx = (corner_row as usize) * ncol + (corner_col as usize);
      if map[corner_idx] == map[center] {
        continue;
      }
      let idx1 = ((row * ncol + col) as i32 + dcol[dc]) as usize;
      let idx2 = ((row as i32 + drow[dr]) as usize) * ncol + col;
      if map[idx1] == map[center] && map[idx2] == map[center] {
        result += 1;
      }
    }
  }

  return result;
}

fn read_map(file: &str) -> io::Result<(Vec<char>, usize, usize)> {
  let input = fs::read_to_string(file)?;
  let nrow = input.lines().count();
  let ncol = input.lines().next().unwrap().len();
  let map = input.chars().filter(|c| *c != '\n').collect();

  Ok((map, nrow, ncol))
}
