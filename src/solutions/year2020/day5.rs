use std::fs;

pub fn solution() {
  let content = fs::read_to_string("inputs/2020/5.txt").unwrap();

  let ids: Vec<u32> = content.lines().map(seat_id).collect();

  let highest = ids.iter().max().unwrap();

  let mut my_id = 8;
  loop {
    if !ids.contains(&my_id) && ids.contains(&(my_id - 1)) && ids.contains(&(my_id + 1)) {
      break;
    }
    my_id += 1;
  }

  println!("Highest seat ID: {}", highest);
  println!("My seat ID: {}", my_id);
}

fn seat_id(line: &str) -> u32 {
  let (r, c) = seat(line);
  r * 8 + c
}

fn seat(line: &str) -> (u32, u32) {
  let (row_str, col_str) = line.split_at(7);
  let row = binary_walk(128, row_str);
  let col = binary_walk(8, col_str);
  (row, col)
}

fn binary_walk(range: u32, path: &str) -> u32 {
  path
    .trim()
    .chars()
    .fold((0, range - 1), |(l, h), ch| {
      let mov = (h - l + 1) / 2;
      match ch {
        'F' | 'L' => (l, h - mov),
        'B' | 'R' => (l + mov, h),
        _ => panic!("unexpected path"),
      }
    })
    .0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    assert_eq!((70, 7), seat("BFFFBBFRRR"));
    assert_eq!((14, 7), seat("FFFBBBFRRR"));
    assert_eq!((102, 4), seat("BBFFBBFRLL"));
  }
}
