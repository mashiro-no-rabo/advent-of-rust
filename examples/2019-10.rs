use std::fs;

type Position = (i64, i64);

#[derive(Debug)]
struct Fraction {
  a: i64,
  b: i64,
}

impl Fraction {
  fn from(in_a: i64, in_b: i64) -> Self {
    let mut i = 2;
    let mut a = in_a;
    let mut b = in_b;
    while (i <= a.abs()) && (i <= b.abs()) {
      while (a % i == 0) && (b % i == 0) {
        a /= i;
        b /= i;
      }
      i += 1;
    }

    Self { a, b }
  }

  fn blocks(&self, other: &Self) -> bool {
    if (self.a == 0) && (other.a == 0) {
      // when in straight line, need to ensure the direction in other axis is same
      (self.b >= 0) == (other.b >= 0)
    } else if (self.b == 0) && (other.b == 0) {
      // same as ^
      (self.a >= 0) == (other.a >= 0)
    } else {
      (self.a == other.a) && (self.b == other.b)
    }
  }
}

fn blocks(root: &Position, a: &Position, b: &Position) -> bool {
  Fraction::from(a.0 - root.0, a.1 - root.1).blocks(&Fraction::from(b.0 - root.0, b.1 - root.1))
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
        asteroids.push((x as i64, y as i64));
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
