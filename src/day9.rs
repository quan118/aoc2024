use std::fs;
use std::io;

pub fn solve() {
  println!("Part 1: {}", part1("inputs/day9/part1").unwrap());
  println!("Part 2: {}", part2("inputs/day9/part1").unwrap());
}

fn part1(file: &str) -> io::Result<u64> {
  let mut blocks = read_data(file)?;
  let mut first = 0;
  let mut last = blocks.len() - 1;

  loop {
    while blocks[first].1 == 0 && first < blocks.len() {
      first += 1;
    }

    while blocks[last].0.len() == 0 && last > 0 {
      last -= 1;
    }
    // println!("first:{} last:{}", first, last);
    if first >= last {
      break;
    }

    let block_to_move = *blocks[last].0.last().unwrap();
    let move_amount = blocks[first].1.min(block_to_move.0);
    blocks[first].1 -= move_amount;
    blocks[first].0.push((move_amount, block_to_move.1));

    let block_to_move_idx = blocks[last].0.len() - 1;
    blocks[last].0[block_to_move_idx].0 -= move_amount;
    blocks[last].1 += move_amount;

    if blocks[last].0[block_to_move_idx].0 == 0 {
      blocks[last].0.pop();
    }

    // println!("{:?}", blocks);
  }

  let mut checksum = 0;
  let mut pos = 0;
  for block in blocks {
    for segment in block.0 {
      for _ in 0..segment.0 {
        checksum += pos * segment.1;
        pos += 1;
      }
    }
  }
  Ok(checksum)
}

fn part2(file: &str) -> io::Result<u64> {
  let mut blocks = read_data(file)?;
  let mut last = blocks.len() - 1;
  if last % 2 == 1 {
    last -= 1;
  }

  while 1 < last {
    let mut first = 1;
    let size_of_last = blocks[last].0[0].0;
    let mut size_of_first = blocks[first].1;
    while size_of_first < size_of_last && first < last {
      first += 2;
      size_of_first = blocks[first].1;
    }

    if size_of_first >= size_of_last {
      blocks[first].1 -= size_of_last;
      let block_id = blocks[last].0[0].1;
      blocks[first].0.push((size_of_last, block_id));

      blocks[last].0.pop();
      blocks[last].1 += size_of_last;
    }

    last -= 2;
  }

  // println!("{:?}", blocks);

  let mut checksum = 0;
  let mut pos = 0;
  for block in blocks {
    for segment in block.0 {
      for _ in 0..segment.0 {
        checksum += pos * segment.1;
        pos += 1;
      }
    }
    for _ in 0..block.1 {
      pos += 1;
    }
  }
  Ok(checksum)
}

fn read_data(file: &str) -> io::Result<Vec<(Vec<(u64, u64)>, u64)>> {
  let input: String = fs::read_to_string(file)?;
  let blocks: Vec<(Vec<(u64, u64)>, u64)> = input
    .chars()
    .filter(|c| *c != '\n')
    .enumerate()
    .map(|(idx, c)| {
      if idx % 2 == 0 {
        let num = c.to_digit(10).unwrap() as u64;
        (vec![(num, (idx / 2) as u64)], 0)
      } else {
        (vec![], c.to_digit(10).unwrap() as u64)
      }
    })
    .collect();

  Ok(blocks)
}
