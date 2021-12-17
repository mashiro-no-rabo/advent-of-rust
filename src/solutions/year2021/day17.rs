use std::{collections::HashSet, fs};

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/17.txt").unwrap();

  let mut parts = content.trim().split(", ");
  let parse_range = |s: &str| -> (i32, i32) {
    let mut parts = s.split("..");
    (
      parts.next().unwrap().parse().unwrap(),
      parts.next().unwrap().parse().unwrap(),
    )
  };
  let (x_start, x_end) = parse_range(&parts.next().unwrap()["target area: x=".len()..]);
  let (y_low, y_high) = parse_range(&parts.next().unwrap()[2..]);

  let mut starts = HashSet::new();
  for x in x_start..=x_end {
    for y in y_low..=y_high {
      for steps in 1..5000 {
        if let Some(s) = find_start(x, y, steps) {
          starts.insert(s);
        }
      }
    }
  }

  let hy = starts.iter().map(|&(_, y)| y * (y + 1) / 2).max().unwrap();

  println!("highest y: {}", hy);
  println!("unique starts: {}", starts.len());
}

fn find_start(x: i32, y: i32, steps: i32) -> Option<(i32, i32)> {
  let tmp = y + ((steps - 1) * steps / 2);
  let dy = if tmp % steps == 0 {
    tmp / steps
  } else {
    return None;
  };

  let min_dx = (((x as f64) * 8. + 1.).sqrt() - 1.) / 2.;
  let x_stop = min_dx.floor() == min_dx;

  let tmp = x + ((steps - 1) * steps / 2);
  let x = tmp / steps;

  if x >= steps {
    if tmp % steps == 0 {
      return Some((x, dy));
    }
  } else if x_stop {
    return Some((min_dx as i32, dy));
  }

  None
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let mut starts = HashSet::new();
    for x in 20..=30 {
      for y in -10..=-5 {
        for steps in 1..5000 {
          if let Some(s) = find_start(x, y, steps) {
            starts.insert(s);
          }
        }
      }
    }

    let hy = starts.iter().map(|&(_, y)| y * (y + 1) / 2).max().unwrap();

    assert_eq!(45, hy);
    assert_eq!(112, starts.len());
  }
}
