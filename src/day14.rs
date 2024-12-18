use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day14/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day14/part1").unwrap());
}

fn part1(file: &str) -> io::Result<i64> {
  let data = read_data(&file)?;
  // println!("{:?}", data);
  let W: usize = 101;
  let H: usize = 103;
  let DURATION = 100;
  let mut final_state: Vec<(i32, i32)> = Vec::new();
  let mut room: Vec<i32> = vec![0; W * H];

  for robot in data {
    let (x, y) = teleport(robot.0, robot.1, robot.2, robot.3, W, H, DURATION);
    // println!("x:{} y:{}", x, y);
    room[(y as usize) * W + (x as usize)] += 1;
  }

  let result = get_safety_factor(&room, W, H);

  Ok(result)
}

fn part2(file: &str) -> io::Result<i64> {
  let data = read_data(&file)?;
  // println!("{:?}", data);
  let W: usize = 101;
  let H: usize = 103;
  let mut seconds: i64 = 0;
  let mut final_state: Vec<(i32, i32)> = Vec::new();

  loop {
    if seconds > 100000 {
      break;
    }
    seconds += 1;
    let mut room: Vec<i32> = vec![0; W * H];
    for robot in data.iter() {
      let (x, y) = teleport(robot.0, robot.1, robot.2, robot.3, W, H, seconds as i32);
      room[(y as usize) * W + (x as usize)] += 1;
    }

    if detech_christmas_tree(&room, W, H) {
      println!("{}", seconds);
      print_map(&room, W, H);
      break;
    }
  }

  Ok(seconds)
}

fn teleport(px: i32, py: i32, vx: i32, vy: i32, w: usize, h: usize, duration: i32) -> (i32, i32) {
  let new_x = ((px + vx * duration) % (w as i32) + (w as i32)) % (w as i32);
  let new_y = ((py + vy * duration) % (h as i32) + (h as i32)) % (h as i32);

  (new_x, new_y)
}

fn detech_christmas_tree(room: &Vec<i32>, w: usize, h: usize) -> bool {
  let mid_x = w / 2;
  for y in 0..h {
    for x in 0..mid_x {
      let mut is_continues = true;

      for i in 0..10 {
        if room[y * w + x + i] == 0 {
          is_continues = false;
          break;
        }
      }
      if is_continues {
        return true;
      }
    }
  }

  false
}

fn get_safety_factor(room: &Vec<i32>, w: usize, h: usize) -> i64 {
  let mid_x = w / 2;
  let mid_y = h / 2;
  let mut quads = [0, 0, 0, 0];

  for y in 0..mid_y {
    for x in 0..mid_x {
      quads[0] += room[y * w + x]
    }

    for x in mid_x + 1..w {
      quads[1] += room[y * w + x]
    }
  }

  for y in mid_y + 1..h {
    for x in 0..mid_x {
      quads[2] += room[y * w + x]
    }

    for x in mid_x + 1..w {
      quads[3] += room[y * w + x]
    }
  }

  (quads[0] * quads[1] * quads[2] * quads[3]) as i64
}

fn read_data(file: &str) -> io::Result<Vec<(i32, i32, i32, i32)>> {
  let input = fs::read_to_string(file)?;
  let mut output: Vec<(i32, i32, i32, i32)> = Vec::new();

  for line in input.lines() {
    let parts: Vec<i32> = line
      .replace("p=", "")
      .replace("v=", "")
      .split_whitespace()
      .flat_map(|s| s.split(","))
      .map(|num| num.parse::<i32>().unwrap())
      .collect();
    output.push((parts[0], parts[1], parts[2], parts[3]));
  }

  Ok(output)
}

fn print_map(room: &Vec<i32>, w: usize, h: usize) {
  for y in 0..h {
    for x in 0..w {
      print!("{}", if room[y * w + x] == 0 { '.' } else { '#' });
    }
    println!();
  }
  println!();
}
