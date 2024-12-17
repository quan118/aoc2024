use num_integer::Integer;
use num_rational::Rational64;
use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day13/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day13/part1").unwrap());
}

fn part1(file: &str) -> io::Result<i64> {
  let data: Vec<i64> = read_data(&file, false)?;

  Ok(process(&data))
}

fn part2(file: &str) -> io::Result<i64> {
  let data: Vec<i64> = read_data(&file, true)?;

  Ok(process(&data))
}

fn process(data: &Vec<i64>) -> i64 {
  let mut result: i64 = 0;

  for i in 0..data.len() / 6 {
    let x1 = Rational64::from(data[i * 6]);
    let y1 = Rational64::from(data[i * 6 + 1]);
    let x2 = Rational64::from(data[i * 6 + 2]);
    let y2 = Rational64::from(data[i * 6 + 3]);
    let X = Rational64::from(data[i * 6 + 4]);
    let Y = Rational64::from(data[i * 6 + 5]);

    let det = x1 * y2 - x2 * y1;
    if det == Rational64::from(0) {
      println!("No unique solution exists");
      continue;
    }

    let t = (X * y2 - x2 * Y) / det;
    let s = (x1 * Y - X * y1) / det;

    if t.is_integer() && s.is_integer() {
      result += t.to_integer() * 3 + s.to_integer();
    }
  }

  result
}

fn read_data(file: &str, fix_conversion_error: bool) -> io::Result<Vec<i64>> {
  let input = fs::read_to_string(file)?;
  let mut output: Vec<i64> = Vec::new();

  for line in input.lines() {
    if line.len() == 0 {
      continue;
    }
    if line.starts_with("Button") {
      let parts: Vec<&str> = line.split(":").nth(1).unwrap().split(",").collect();
      // println!("{:?}", parts);
      for part in parts {
        let number = part.split("+").nth(1).unwrap().parse::<i64>().unwrap();
        output.push(number);
      }
    } else if line.starts_with("Prize") {
      let parts: Vec<&str> = line.split(":").nth(1).unwrap().split(",").collect();
      for part in parts {
        let mut number = part.split("=").nth(1).unwrap().parse::<i64>().unwrap();
        if fix_conversion_error {
          number += 10000000000000;
        }
        output.push(number);
      }
    }
  }

  Ok(output)
}
