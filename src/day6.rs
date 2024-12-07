use std::collections::HashSet;
use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day6/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day6/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u64> {
  let (mut map, nrow, ncol, mut cur_row, mut cur_col) = read_map(file)?;
  let mut cur_dir = '^';
  let mut result: u64 = 1;

  loop {
    if let Some((new_row, new_col, new_dir)) = advance(cur_row, cur_col, cur_dir, nrow, ncol, &map)
    {
      if map[(new_row as usize) * ncol + new_col as usize] == '.' {
        result += 1;
        map[(new_row as usize) * ncol + new_col as usize] = new_dir;
      }
      cur_row = new_row;
      cur_col = new_col;
      cur_dir = new_dir;
    } else {
      break;
    }
  }

  Ok(result)
}

fn part2(file: &str) -> io::Result<u64> {
  let (mut map, nrow, ncol, mut cur_row, mut cur_col) = read_map(file)?;
  let mut cur_dir = '^';
  let mut result: u64 = 0;
  let mut visited_positions: HashSet<(u32, char)> = HashSet::new();
  let idx = (cur_row as usize) * ncol + cur_col as usize;
  visited_positions.insert((idx as u32, cur_dir));
  loop {
    if let Some((new_row, new_col, new_dir)) = advance(cur_row, cur_col, cur_dir, nrow, ncol, &map)
    {
      let idx = (new_row as usize) * ncol + new_col as usize;

      if map[idx] == '.' {
        // try to put an obstacle
        let mut visited_positions2 = visited_positions.clone();
        let mut map2 = map.clone();
        map2[idx] = '#';
        let mut cur_row2 = cur_row;
        let mut cur_col2 = cur_col;
        let mut cur_dir2 = cur_dir;
        loop {
          if let Some((new_row2, new_col2, new_dir2)) =
            advance(cur_row2, cur_col2, cur_dir2, nrow, ncol, &map2)
          {
            let idx2 = (new_row2 as usize) * ncol + new_col2 as usize;
            if visited_positions2.contains(&(idx2 as u32, new_dir2)) {
              result += 1;
              break;
            } else {
              visited_positions2.insert((idx2 as u32, new_dir2));
              map2[idx2] = new_dir2;
              cur_row2 = new_row2;
              cur_col2 = new_col2;
              cur_dir2 = new_dir2;
            }
          } else {
            break;
          }
        }
      }

      map[idx] = new_dir;
      cur_row = new_row;
      cur_col = new_col;
      cur_dir = new_dir;
      visited_positions.insert((idx as u32, new_dir));
    } else {
      break;
    }
  }

  Ok(result)
}

fn advance(
  cur_row: i32,
  cur_col: i32,
  cur_dir: char,
  nrow: usize,
  ncol: usize,
  map: &Vec<char>,
) -> Option<(i32, i32, char)> {
  if cur_row == 0 && cur_dir == '^'
    || cur_row == (nrow as i32) - 1 && cur_dir == 'v'
    || cur_col == 0 && cur_dir == '<'
    || cur_col == (ncol as i32) - 1 && cur_dir == '>'
  {
    return None;
  }
  let mut new_dir = cur_dir;
  let mut new_row = cur_row;
  let mut new_col = cur_col;
  if cur_dir == '^' {
    if map[((cur_row - 1) as usize) * ncol + cur_col as usize] == '#' {
      new_dir = '>';
    } else {
      new_row -= 1;
    }
  } else if cur_dir == '>' {
    if map[(cur_row as usize) * ncol + (cur_col as usize) + 1] == '#' {
      new_dir = 'v';
    } else {
      new_col += 1;
    }
  } else if cur_dir == 'v' {
    if map[((cur_row as usize) + 1) * ncol + (cur_col as usize)] == '#' {
      new_dir = '<';
    } else {
      new_row += 1;
    }
  } else if cur_dir == '<' {
    if map[(cur_row as usize) * ncol + (cur_col as usize) - 1] == '#' {
      new_dir = '^';
    } else {
      new_col -= 1;
    }
  }

  Some((new_row, new_col, new_dir))
}

fn read_map(file: &str) -> io::Result<(Vec<char>, usize, usize, i32, i32)> {
  let input = fs::read_to_string(file)?;
  let nrow = input.lines().count();
  let ncol = input.lines().next().unwrap().len();
  let map = input.chars().filter(|c| *c != '\n').collect::<Vec<char>>();
  let mut cur_row: i32 = 0;
  let mut cur_col: i32 = 0;

  for r in 0..nrow {
    for c in 0..ncol {
      let idx = r * ncol + c;
      if map[idx] == '^' {
        cur_row = r as i32;
        cur_col = c as i32;
      }
    }
  }

  Ok((map, nrow, ncol, cur_row, cur_col))
}
