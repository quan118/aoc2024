use std::collections::HashMap;
use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day11/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day11/part1").unwrap());
}

fn part1(file: &str) -> io::Result<usize> {
  let mut numbers = read_input(&file)?;
  let result = process(25, &numbers);
  Ok(result)
}

fn part2(file: &str) -> io::Result<usize> {
  let mut numbers = read_input(&file)?;
  let result = process(75, &numbers);
  Ok(result)
}

fn read_input(file: &str) -> io::Result<Vec<u64>> {
  let input = fs::read_to_string(file)?;
  let numbers = input
    .lines()
    .next()
    .unwrap()
    .split_whitespace()
    .map(|s| s.parse::<u64>().unwrap())
    .collect();
  Ok(numbers)
}

fn blink(numbers: Vec<u64>) -> Vec<u64> {
  let mut new_numbers: Vec<u64> = Vec::new();

  for number in numbers {
    if number == 0 {
      new_numbers.push(1);
    } else if number.to_string().len() % 2 == 0 {
      let s = number.to_string();
      let (left, right) = s.split_at(s.len() / 2);
      new_numbers.push(left.parse::<u64>().unwrap());
      new_numbers.push(right.parse::<u64>().unwrap());
    } else {
      new_numbers.push(number * 2024);
    }
  }

  new_numbers
}

fn count(number: u64, level: u32, memo: &mut HashMap<(u64, u32), usize>) -> usize {
  if level == 0 {
    return 1;
  }
  let key = (number, level);

  if let Some(&result) = memo.get(&key) {
    return result;
  }

  let mut result: usize = 0;
  let new_numbers = blink(vec![number]);
  for new_number in new_numbers {
    result += count(new_number, level - 1, memo);
  }
  memo.insert(key, result);

  result
}

fn process(max_level: u32, numbers: &Vec<u64>) -> usize {
  let mut memo: HashMap<(u64, u32), usize> = HashMap::new();
  let mut result: usize = 0;

  for number in numbers {
    result += count(*number, max_level, &mut memo);
  }

  result
}
