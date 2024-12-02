use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day2/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day2/part1").unwrap());
}

fn part1(file: &str) -> io::Result<i32> {
  let input = fs::read_to_string(file)?;
  let mut result: i32 = 0;

  for line in input.lines() {
    let numbers: Vec<i32> = line
      .split_whitespace()
      .map(|s| s.parse::<i32>().unwrap())
      .collect();
    if check_report_safety(&numbers) {
      result += 1;
    }
  }

  Ok(result)
}

fn part2(file: &str) -> io::Result<i32> {
  let input = fs::read_to_string(file)?;
  let mut result: i32 = 0;

  for line in input.lines() {
    let numbers: Vec<i32> = line
      .split_whitespace()
      .map(|s| s.parse::<i32>().unwrap())
      .collect();

    let mut removable_candidates: Vec<usize> = Vec::new();
    let mut idx: usize = 1;
    while idx < numbers.len() - 1 {
      if (numbers[idx - 1] <= numbers[idx] && numbers[idx] >= numbers[idx + 1])
        || (numbers[idx - 1] >= numbers[idx] && numbers[idx] <= numbers[idx + 1])
      {
        removable_candidates.push(idx - 1);
        removable_candidates.push(idx);
        removable_candidates.push(idx + 1);
        break;
      }

      if (numbers[idx - 1] - numbers[idx]).abs() > 3 {
        removable_candidates.push(idx - 1);
        removable_candidates.push(idx);
        break;
      }

      if (numbers[idx + 1] - numbers[idx]).abs() > 3 {
        removable_candidates.push(idx + 1);
        removable_candidates.push(idx);
        break;
      }

      idx += 1;
    }

    let mut is_safe_report = true;
    if removable_candidates.len() > 0 {
      let mut is_safe = false;
      for candidate in &removable_candidates {
        let mut new_numbers = numbers.clone();
        new_numbers.remove(*candidate as usize);
        is_safe = check_report_safety(&new_numbers);
        if is_safe {
          break;
        }
      }
      is_safe_report = is_safe;
    } else {
      is_safe_report = check_report_safety(&numbers);
    }

    if is_safe_report {
      result += 1;
    }
  }

  Ok(result)
}

fn check_report_safety(numbers: &Vec<i32>) -> bool {
  let is_increasing = if numbers[0] < numbers[1] { 1 } else { -1 };
  for idx in 0..numbers.len() - 1 {
    let diff: i32 = numbers[idx + 1] - numbers[idx];
    let tmp: i32 = diff * is_increasing;
    if tmp < 1 || 3 < tmp {
      return false;
    }
  }

  true
}
