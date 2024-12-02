use std::fs;
use std::io;
use std::collections::HashMap;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day1/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day1/part1").unwrap());
}

fn part1(file: &str) -> io::Result<i32> {
  let (left, right) = read_data(file);
  let mut output: i32 = 0;
  
  for (idx, left_val) in left.iter().enumerate() {
    output += (left_val - right[idx]).abs()
  }

  Ok(output)
}

fn part2(file: &str) -> io::Result<i32> {
  let (left, right) = read_data(file);
  
  let mut map_right = HashMap::new();
  for item in &right {
    let count: &mut i32 = map_right.entry(item).or_insert(0);
    *count += 1;
  }

  let mut output: i32 = 0;
  for item in &left {
    let count = map_right.get(item).copied().unwrap_or(0);
    output += item*count;
  }

  Ok(output)
}

fn read_data(file: &str) -> (Vec<i32>, Vec<i32>) {
  let mut left: Vec<i32> = Vec::new();
  let mut right: Vec<i32> = Vec::new();

  let input: String = fs::read_to_string(file).unwrap();

  // line is a &str
  for line in input.lines() {
    let mut token_iter: std::str::SplitWhitespace = line.split_whitespace();

    if let (Some(first), Some(second)) = (
      token_iter.next().and_then(|s| s.parse::<i32>().ok()),
      token_iter.next().and_then(|s| s.parse::<i32>().ok())
    ) {
      left.push(first);
      right.push(second);
    }
  }

  left.sort();
  right.sort();

  (left, right)
}
