use std::fs;
use std::io;
use regex::Regex;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day3/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day3/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u32> {
  let input = fs::read_to_string(file)?;
  let mut result: u32 = 0;
  let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

  for line in input.lines() {
    for m in re.find_iter(&line) {
      let s: &str = m.as_str();
      // println!("Found match: {}", &s);

      let tokens: Vec<&str> = s[4..s.len()-1].split(",").collect();
      let product = tokens.iter()
        .map(|x| x.parse::<u32>().unwrap())
        .reduce(|acc, x| acc*x)
        .unwrap();
      result += product;
    }
  }

  Ok(result)
}

fn part2(file: &str) -> io::Result<u32> {
  let input = fs::read_to_string(file)?;
  let mut result: u32 = 0;
  let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
  let mut is_enabled = true;

  for line in input.lines() {
    for m in re.find_iter(&line) {
      let s: &str = m.as_str();
      // println!("Found match: {}", &s);
      if s.starts_with("don't") {
        is_enabled = false;
      } else if s.starts_with("do") {
        is_enabled = true;
      } else {
        if is_enabled {
          let tokens: Vec<&str> = s[4..s.len()-1].split(",").collect();
          let product = tokens.iter()
            .map(|x| x.parse::<u32>().unwrap())
            .reduce(|acc, x| acc*x)
            .unwrap();
          result += product;
        }
      }
    }
  }

  Ok(result)
}