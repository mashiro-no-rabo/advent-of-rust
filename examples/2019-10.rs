use std::fs;

use fraction::Fraction;

type Position = (usize, usize);

fn blocks(root: &Position, a: &Position, b: &Position) -> bool {
  let ax = a.0 as i64 - root.0 as i64;
  let ay = a.1 as i64 - root.1 as i64;
  let bx = b.0 as i64 - root.0 as i64;
  let by = b.1 as i64 - root.1 as i64;

  // ensure they are in the same quadrant, Fraction does not handle signed integers
  if (ax >= 0) == (bx >= 0) && (ay >= 0) == (by >= 0) {
    let diffa = Fraction::new(ax.abs() as u64, ay.abs() as u64);
    let diffb = Fraction::new(bx.abs() as u64, by.abs() as u64);
    diffa == diffb
  } else {
    false
  }
}

fn find_best_location(asteroids: &[Position]) -> usize {
  // fold to find max, start with 0
  asteroids.iter().fold(0, |prev_max, station| {
    // take max over previous or current value
    prev_max.max(
      asteroids
        .iter()
        // fold to detectable asteroids
        .fold(vec![], |mut detectable, x| {
          // ignore station itself
          if x == station {
            return detectable;
          }

          // check if any already detected asteroid blocks the attempted one
          let blocked = detectable.iter().any(|y| blocks(station, x, y));
          if !blocked {
            detectable.push(x.clone())
          }
          detectable
        })
        // count the length, and pass to `max`
        .len(),
    )
  })
}

fn parse_asteroids(map: &str) -> Vec<Position> {
  let mut asteroids = vec![];
  for (y, line) in map.lines().enumerate() {
    for (x, byte) in line.bytes().enumerate() {
      if byte == b'#' {
        asteroids.push((x, y));
      }
    }
  }
  asteroids
}

fn main() {
  let input = fs::read_to_string("inputs/2019/10.txt").unwrap();
  let asteroids = parse_asteroids(&input);

  println!("Solution 1: {}", find_best_location(&asteroids));
}
