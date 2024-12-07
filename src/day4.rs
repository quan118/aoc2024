use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day4/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day4/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u64> {
  let (data, nrow, ncol) = read_file(file)?;
  let mut result = 0;
  const WORDS: [char; 4] = ['X', 'M', 'A', 'S'];
  let dirs = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
  ];

  for r in 0..nrow {
    for c in 0..ncol {
      for d in dirs {
        let mut matches = true;
        for i in 0..4 {
          let r_pos: i32 = r as i32 + d.0 * i as i32;
          let c_pos: i32 = c as i32 + d.1 * i as i32;
          if r_pos < 0
            || r_pos >= nrow as i32
            || c_pos < 0
            || c_pos >= ncol as i32
            || data[(r_pos as usize) * ncol + (c_pos as usize)] != WORDS[i]
          {
            matches = false;
            break;
          }
        }
        if matches {
          result += 1
        }
      }
    }
  }

  Ok(result)
}

fn part2(file: &str) -> io::Result<u64> {
  let (data, nrow, ncol) = read_file(file)?;
  let mut result = 0;

  for r in 0..nrow {
    for c in 0..ncol {
      let center = r * ncol + c;
      if data[center] != 'A'
        || (r as i32) - 1 < 0
        || r + 1 >= nrow
        || (c as i32) - 1 < 0
        || c + 1 >= ncol
      {
        continue;
      }
      let top_left: usize = (r - 1) * ncol + (c - 1);
      let top_right: usize = (r - 1) * ncol + (c + 1);
      let bottom_left: usize = (r + 1) * ncol + (c - 1);
      let bottom_right: usize = (r + 1) * ncol + (c + 1);

      if ((data[top_left] == 'M' && data[bottom_right] == 'S')
        || (data[top_left] == 'S' && data[bottom_right] == 'M'))
        && ((data[top_right] == 'M' && data[bottom_left] == 'S')
          || (data[top_right] == 'S' && data[bottom_left] == 'M'))
      {
        result += 1;
      }
    }
  }

  Ok(result)
}

fn read_file(file: &str) -> io::Result<(Vec<char>, usize, usize)> {
  let input = fs::read_to_string(file)?;
  let nrow = input.lines().count();
  let ncol = input.lines().next().unwrap().len();
  let data = input.chars().filter(|c| *c != '\n').collect::<Vec<char>>();

  Ok((data, nrow, ncol))
}
