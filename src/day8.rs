use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day8/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day8/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u32> {
  let (map, nrow, ncol, mut freqs) = read_map(file)?;

  let mut antinodes_set: HashSet<u32> = HashSet::new();

  for antennas in freqs.values() {
    for i in 0..(antennas.len() - 1) {
      for j in i + 1..antennas.len() {
        let antinodes: Vec<u32> = get_antinodes(antennas[i], antennas[j], nrow, ncol);
        antinodes_set.extend(antinodes);
      }
    }
  }

  Ok(antinodes_set.len() as u32)
}

fn part2(file: &str) -> io::Result<u32> {
  let (map, nrow, ncol, mut freqs) = read_map(file)?;

  let mut antinodes_set: HashSet<u32> = HashSet::new();

  for antennas in freqs.values() {
    for i in 0..(antennas.len() - 1) {
      for j in i + 1..antennas.len() {
        let antinodes: Vec<u32> = get_antinodes2(antennas[i], antennas[j], nrow, ncol);
        antinodes_set.extend(antinodes);
      }
    }
  }

  Ok(antinodes_set.len() as u32)
}

fn get_antinodes(idx1: u32, idx2: u32, nrow: usize, ncol: usize) -> Vec<u32> {
  let r1: i32 = (idx1 as i32) / (ncol as i32);
  let c1: i32 = (idx1 as i32) % (ncol as i32);
  let r2: i32 = (idx2 as i32) / (ncol as i32);
  let c2: i32 = (idx2 as i32) % (ncol as i32);
  let r3: i32 = r1 + (r2 - r1) * 2;
  let c3: i32 = c1 + (c2 - c1) * 2;
  let r0: i32 = r2 + (r1 - r2) * 2;
  let c0: i32 = c2 + (c1 - c2) * 2;
  let mut antinodes: Vec<u32> = Vec::new();
  if r0 >= 0 && r0 < nrow as i32 && c0 >= 0 && c0 < ncol as i32 {
    antinodes.push((r0 * ncol as i32 + c0) as u32);
  }
  if r3 >= 0 && r3 < nrow as i32 && c3 >= 0 && c3 < ncol as i32 {
    antinodes.push((r3 * ncol as i32 + c3) as u32);
  }

  antinodes
}

fn get_antinodes2(idx1: u32, idx2: u32, nrow: usize, ncol: usize) -> Vec<u32> {
  let r1: i32 = (idx1 as i32) / (ncol as i32);
  let c1: i32 = (idx1 as i32) % (ncol as i32);
  let r2: i32 = (idx2 as i32) / (ncol as i32);
  let c2: i32 = (idx2 as i32) % (ncol as i32);
  let mut antinodes: Vec<u32> = Vec::new();
  antinodes.push((r1 * ncol as i32 + c1) as u32);
  antinodes.push((r2 * ncol as i32 + c2) as u32);
  let mut k = 1;
  loop {
    let r: i32 = r2 + (r2 - r1) * k;
    let c: i32 = c2 + (c2 - c1) * k;
    if r < 0 || r >= nrow as i32 || c < 0 || c >= ncol as i32 {
      break;
    }
    antinodes.push((r * ncol as i32 + c) as u32);
    k += 1;
  }

  let mut k = 1;
  loop {
    let r: i32 = r1 + (r1 - r2) * k;
    let c: i32 = c1 + (c1 - c2) * k;
    if r < 0 || r >= nrow as i32 || c < 0 || c >= ncol as i32 {
      break;
    }
    antinodes.push((r * ncol as i32 + c) as u32);
    k += 1;
  }

  antinodes
}

fn read_map(file: &str) -> io::Result<(Vec<char>, usize, usize, HashMap<char, Vec<u32>>)> {
  let input = fs::read_to_string(file)?;
  let nrow = input.lines().count();
  let ncol = input.lines().next().unwrap().len();
  let map = input.chars().filter(|c| *c != '\n').collect::<Vec<char>>();

  let mut freqs: HashMap<char, Vec<u32>> = HashMap::new();
  for r in 0..nrow {
    for c in 0..ncol {
      let idx = r * ncol + c;
      if map[idx] != '.' {
        if freqs.contains_key(&map[idx]) {
          freqs.get_mut(&map[idx]).unwrap().push(idx as u32);
        } else {
          freqs.insert(map[idx], vec![idx as u32]);
        }
      }
    }
  }

  Ok((map, nrow, ncol, freqs))
}
