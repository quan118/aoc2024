use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day7/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day7/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u64> {
  let input = fs::read_to_string(file)?;
  let mut result: u64 = 0;
  for line in input.lines() {
    let mut token_iter = line.split(":");
    let test_value = token_iter.next().unwrap().parse::<u64>().unwrap();
    let numbers: Vec<u64> = token_iter
      .next()
      .unwrap()
      .split_whitespace()
      .map(|e| e.parse::<u64>().unwrap())
      .collect();
    let mut operators = vec![0; numbers.len() - 1];
    let mut is_correct = check(&numbers, &operators, test_value);

    while !is_correct {
      let has_next_value = advance(&mut operators, 1);
      if !has_next_value {
        is_correct = false;
        break;
      }
      is_correct = check(&numbers, &operators, test_value);
    }

    if is_correct {
      result += test_value;
    }
  }

  Ok(result)
}

fn part2(file: &str) -> io::Result<u64> {
  let input = fs::read_to_string(file)?;
  let mut result: u64 = 0;
  for line in input.lines() {
    let mut token_iter = line.split(":");
    let test_value = token_iter.next().unwrap().parse::<u64>().unwrap();
    let numbers: Vec<u64> = token_iter
      .next()
      .unwrap()
      .split_whitespace()
      .map(|e| e.parse::<u64>().unwrap())
      .collect();
    let mut operators = vec![0; numbers.len() - 1];
    let mut is_correct = check(&numbers, &operators, test_value);

    while !is_correct {
      let has_next_value = advance(&mut operators, 2);
      if !has_next_value {
        is_correct = false;
        break;
      }
      is_correct = check(&numbers, &operators, test_value);
    }

    if is_correct {
      result += test_value;
    }
  }

  Ok(result)
}

fn check(numbers: &Vec<u64>, operators: &Vec<i32>, test_value: u64) -> bool {
  let mut total: u64 = numbers[0];
  for i in 0..operators.len() {
    if total > test_value {
      return false;
    }
    if operators[i] == 0 {
      total += numbers[i + 1];
    } else if operators[i] == 1 {
      total *= numbers[i + 1];
    } else if operators[i] == 2 {
      total = total * 10_u64.pow((numbers[i + 1]).to_string().len() as u32) + numbers[i + 1];
    }
  }

  total == test_value
}

// operator possible values: 0(+), 1(*), 2(||)
fn advance(operators: &mut Vec<i32>, max_value: i32) -> bool {
  let mut idx: i32 = operators.len() as i32 - 1;
  while idx >= 0 && operators[idx as usize] >= max_value {
    idx -= 1;
  }

  if idx < 0 {
    return false;
  } else {
    operators[idx as usize] += 1;
    for i in (idx as usize + 1)..operators.len() {
      operators[i] = 0;
    }
  }

  true
}
