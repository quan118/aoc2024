use std::collections::HashSet;
use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day5/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day5/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u32> {
  let input = fs::read_to_string(file)?;

  let mut number_set: HashSet<u32> = HashSet::new();
  let mut is_reading_rules = true;
  let mut result: u32 = 0;

  for line in input.lines() {
    if line.trim() == "" {
      is_reading_rules = false;
      continue;
    }

    if is_reading_rules {
      let numbers: Vec<u32> = line.split("|").map(|e| e.parse::<u32>().unwrap()).collect();
      let hash_number = numbers[0] * 100 + numbers[1];
      number_set.insert(hash_number);
    } else {
      let numbers: Vec<u32> = line.split(",").map(|e| e.parse::<u32>().unwrap()).collect();
      let mut is_correct = true;
      'outer: for i in 0..numbers.len() - 1 {
        for j in i + 1..numbers.len() {
          let hash_number = numbers[j] * 100 + numbers[i];
          if number_set.contains(&hash_number) {
            is_correct = false;
            break 'outer;
          }
        }
      }

      if is_correct {
        result += numbers[numbers.len() / 2];
      }
    }
  }

  Ok(result)
}

fn part2(file: &str) -> io::Result<u32> {
  let input = fs::read_to_string(file)?;

  let mut number_set: HashSet<u32> = HashSet::new();
  let mut is_reading_rules = true;
  let mut result: u32 = 0;

  for line in input.lines() {
    if line.trim() == "" {
      is_reading_rules = false;
      continue;
    }

    if is_reading_rules {
      let numbers: Vec<u32> = line.split("|").map(|e| e.parse::<u32>().unwrap()).collect();
      let hash_number = numbers[0] * 100 + numbers[1];
      number_set.insert(hash_number);
    } else {
      let mut numbers: Vec<u32> = line.split(",").map(|e| e.parse::<u32>().unwrap()).collect();
      let mut is_correct = true;
      'outer: for i in 0..numbers.len() - 1 {
        for j in i + 1..numbers.len() {
          let hash_number = numbers[j] * 100 + numbers[i];
          if number_set.contains(&hash_number) {
            is_correct = false;
            break 'outer;
          }
        }
      }

      if !is_correct {
        numbers.sort_by(|a, b| {
          let hash_number = a * 100 + b;
          if number_set.contains(&hash_number) {
            std::cmp::Ordering::Less
          } else {
            std::cmp::Ordering::Greater
          }
        });
        result += numbers[numbers.len() / 2];
      }
    }
  }

  Ok(result)
}
