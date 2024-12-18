use std::collections::HashMap;
use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day15/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day15/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u64> {
  let (mut map, w, h, dirs) = read_data(&file, false)?;

  simulate(&mut map, w, h, &dirs);

  Ok(get_sum(&map, w, h, false))
}

fn part2(file: &str) -> io::Result<u64> {
  let (mut map, w, h, dirs) = read_data(&file, true)?;

  simulate2(&mut map, w, h, &dirs);

  Ok(get_sum(&map, w, h, true))
}

fn simulate(map: &mut Vec<char>, w: usize, h: usize, directions: &Vec<char>) {
  let (mut rx, mut ry) = get_robot_location(&map, w, h);
  let dir_map: HashMap<char, (i32, i32)> =
    HashMap::from([('<', (-1, 0)), ('^', (0, -1)), ('>', (1, 0)), ('v', (0, 1))]);

  for &d in directions {
    let (mut dx, mut dy) = dir_map[&d];
    let mut x = rx as i32;
    let mut y = ry as i32;

    loop {
      x += dx;
      y += dy;
      let new_idx = (y * (w as i32) + x) as usize;
      // println!("x:{} y:{} new_idx:{}", x, y,  new_idx);

      if map[new_idx] == '#' {
        break;
      } else if map[new_idx] == '.' {
        while x != rx as i32 || y != ry as i32 {
          map[(y * (w as i32) + x) as usize] = map[((y - dy) * (w as i32) + (x - dx)) as usize];
          x -= dx;
          y -= dy;
        }
        map[ry * w + rx] = '.';
        rx = (rx as i32 + dx) as usize;
        ry = (ry as i32 + dy) as usize;
        break;
      }
    }
  }
}

fn simulate2(map: &mut Vec<char>, w: usize, h: usize, directions: &Vec<char>) {
  let (mut rx, mut ry) = get_robot_location(&map, w, h);
  let dir_map: HashMap<char, (i32, i32)> =
    HashMap::from([('<', (-1, 0)), ('^', (0, -1)), ('>', (1, 0)), ('v', (0, 1))]);

  for &d in directions {
    let (dx, dy) = dir_map[&d];

    let x = rx as i32 + dx;
    let y = ry as i32 + dy;
    if dx != 0 && can_move_horizontal(&map, w, h, x, y, dx) {
      move_horizontal(map, w, h, x, y, dx);
      map[ry * w + rx] = '.';
      rx = x as usize;
      ry = y as usize;
      map[ry * w + rx] = '@';
    } else if dy != 0 && can_move_vertical(&map, w, h, x, y, dy) {
      move_vertical(map, w, h, x, y, dy);
      map[ry * w + rx] = '.';
      rx = x as usize;
      ry = y as usize;
      map[ry * w + rx] = '@';
    }
  }
}

fn move_horizontal(map: &mut Vec<char>, w: usize, h: usize, x: i32, y: i32, dx: i32) {
  // find empty space
  let orig_idx = y as usize * w + x as usize;
  let mut idx = orig_idx as i32;
  while map[idx as usize] != '.' {
    idx += dx;
  }
  while idx as usize != orig_idx {
    map[idx as usize] = map[(idx - dx) as usize];
    idx -= dx;
  }
  map[orig_idx] = '.';
}

fn can_move_horizontal(map: &Vec<char>, w: usize, h: usize, x: i32, y: i32, dx: i32) -> bool {
  let idx = y as usize * w + x as usize;
  if map[idx] == '.' {
    return true;
  } else if map[idx] == '#' {
    return false;
  }

  return can_move_horizontal(&map, w, h, x + 2 * dx, y, dx);
}

fn move_vertical(map: &mut Vec<char>, w: usize, h: usize, x: i32, y: i32, dy: i32) {
  let idx0 = y as usize * w + x as usize;
  if map[idx0] == '.' {
    return;
  }
  let x1 = if map[idx0] == '[' { x + 1 } else { x - 1 };
  let idx1 = y as usize * w + x1 as usize;
  move_vertical(map, w, h, x, y + dy, dy);
  move_vertical(map, w, h, x1, y + dy, dy);
  map[(y + dy) as usize * w + x as usize] = map[idx0];
  map[(y + dy) as usize * w + x1 as usize] = map[idx1];
  map[idx0] = '.';
  map[idx1] = '.';
}

fn can_move_vertical(map: &Vec<char>, w: usize, h: usize, x: i32, y: i32, dy: i32) -> bool {
  let idx0 = y as usize * w + x as usize;
  if map[idx0] == '.' {
    return true;
  } else if map[idx0] == '#' {
    return false;
  }
  let x2 = if map[idx0] == '[' { x + 1 } else { x - 1 };
  return can_move_vertical(&map, w, h, x, y + dy, dy)
    && can_move_vertical(&map, w, h, x2, y + dy, dy);
}

fn get_robot_location(map: &Vec<char>, w: usize, h: usize) -> (usize, usize) {
  let mut rx: usize = 0;
  let mut ry: usize = 0;
  for i in 0..map.len() {
    if map[i] == '@' {
      ry = i / w;
      rx = i % w;
      break;
    }
  }

  (rx, ry)
}

fn get_sum(map: &Vec<char>, w: usize, h: usize, double_the_wide: bool) -> u64 {
  let mut sum: u64 = 0;
  for y in 0..h {
    for x in 0..w {
      if map[y * w + x] == if double_the_wide { '[' } else { 'O' } {
        sum += (100 * y + x) as u64;
      }
    }
  }

  sum
}

fn read_data(
  file: &str,
  double_the_wide: bool,
) -> io::Result<(Vec<char>, usize, usize, Vec<char>)> {
  let input = fs::read_to_string(file)?;
  let mut map: Vec<char> = Vec::new();
  let mut w: usize = 0;
  let mut h: usize = 0;
  let mut dirs: Vec<char> = Vec::new();
  let mut reading_state = 0; // 0: read map; 1: read directions
  for line in input.lines() {
    if line.len() == 0 {
      reading_state = 1;
      continue;
    }
    if reading_state == 0 {
      if w == 0 {
        w = line.len();
        if double_the_wide {
          w *= 2;
        }
      }
      h += 1;
      for c in line.chars() {
        match c {
          '@' => {
            if double_the_wide {
              map.extend(['@', '.'])
            } else {
              map.push('@')
            }
          }
          '#' => {
            if double_the_wide {
              map.extend(['#', '#'])
            } else {
              map.push('#')
            }
          }
          'O' => {
            if double_the_wide {
              map.extend(['[', ']'])
            } else {
              map.push('O')
            }
          }
          '.' => {
            if double_the_wide {
              map.extend(['.', '.'])
            } else {
              map.push('.')
            }
          }
          _ => println!("This should not be happened"),
        }
      }
    } else {
      dirs.extend(line.chars());
    }
  }

  Ok((map, w, h, dirs))
}

fn print_map(map: &Vec<char>, w: usize, h: usize) {
  for y in 0..h {
    for x in 0..w {
      print!("{}", map[y * w + x]);
    }
    println!();
  }
  println!();
}
