use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day10/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day10/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u32> {
  let (map, nrow, ncol) = read_map(file)?;
  let mut result: u32 = 0;
  for idx in 0..map.len() {
    if map[idx] == 0 {
      let nines = flood_fill(&map, nrow, ncol, idx);
      result += nines.len() as u32;
    }
  }

  Ok(result)
}

fn part2(file: &str) -> io::Result<u32> {
  let (map, nrow, ncol) = read_map(file)?;
  let mut result: u32 = 0;
  for idx in 0..map.len() {
    if map[idx] == 0 {
      let nines = flood_fill(&map, nrow, ncol, idx);
      for nine in nines {
        result += get_rating(&map, nrow, ncol, idx, nine);
      }
    }
  }

  Ok(result)
}

fn read_map(file: &str) -> io::Result<(Vec<u32>, usize, usize)> {
  let input = fs::read_to_string(file)?;
  let nrow = input.lines().count();
  let ncol = input.lines().next().unwrap().len();
  let map = input
    .chars()
    .filter(|c| *c != '\n')
    .map(|c| c.to_digit(10).unwrap() as u32)
    .collect();

  Ok((map, nrow, ncol))
}

fn flood_fill(map: &Vec<u32>, nrow: usize, ncol: usize, start: usize) -> Vec<usize> {
  let mut visited: HashSet<usize> = HashSet::new();
  let mut queue: VecDeque<usize> = VecDeque::new();
  let mut nines: Vec<usize> = vec![];

  queue.push_back(start);
  visited.insert(start);

  while !queue.is_empty() {
    let pos = queue.pop_front().unwrap();
    if map[pos] == 9 {
      nines.push(pos);
    }
    let neighbors = get_neighbors(nrow, ncol, pos);
    for neighbor in neighbors {
      if visited.contains(&neighbor) || map[neighbor] != map[pos] + 1 {
        continue;
      }
      queue.push_back(neighbor);
      visited.insert(neighbor);
    }
  }

  nines
}

fn get_neighbors(nrow: usize, ncol: usize, cur: usize) -> Vec<usize> {
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

  neighbors
}

fn get_rating(map: &Vec<u32>, nrow: usize, ncol: usize, from: usize, to: usize) -> u32 {
  let from1: i32 = from as i32;
  let to1: i32 = to as i32;
  if (from1 + 1 == to1
    || from1 - 1 == to1
    || from1 + ncol as i32 == to1
    || from1 - ncol as i32 == to1)
    && map[from] + 1 == map[to]
  {
    return 1;
  }

  let neighbors = get_neighbors(nrow, ncol, to);
  let mut rating: u32 = 0;
  for neighbor in neighbors {
    if map[neighbor] + 1 == map[to] {
      rating += get_rating(&map, nrow, ncol, from, neighbor);
    }
  }

  rating
}
