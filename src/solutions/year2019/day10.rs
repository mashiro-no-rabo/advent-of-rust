use std::cmp::Ordering;
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

  fn from_diff(diff: (i64, i64)) -> Self {
    Self::from(diff.0, diff.1)
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

  fn deg(&self) -> f64 {
    let mut ret = 90.0 - (self.b as f64).atan2(self.a as f64).to_degrees();

    if ret < 0.0 {
      ret += 360.0;
    }
    ret
  }
}

fn blocks(root: &Position, a: &Position, b: &Position) -> bool {
  Fraction::from(a.0 - root.0, a.1 - root.1).blocks(&Fraction::from(b.0 - root.0, b.1 - root.1))
}

fn compare(root: &Position, a: &Position, b: &Position) -> Ordering {
  let dega = Fraction::from(a.0 - root.0, root.1 - a.1).deg();
  let degb = Fraction::from(b.0 - root.0, root.1 - b.1).deg();
  if dega < degb {
    Ordering::Less
  } else {
    Ordering::Greater
  }
}

fn attempt_min(root: &Position, previous: &Position, attempt: &Position) -> Position {
  let diff_prev = (previous.0 - root.0, previous.1 - root.1);
  let prev_f = Fraction::from_diff(diff_prev);
  let diff_atmp = (attempt.0 - root.0, attempt.1 - root.1);
  let atmp_f = Fraction::from_diff(diff_atmp);

  if prev_f.blocks(&atmp_f) {
    let dist_prev = diff_prev.0.abs() + diff_prev.1.abs();
    let dist_atmp = diff_atmp.0.abs() + diff_atmp.1.abs();
    if dist_atmp < dist_prev {
      return (attempt.0, attempt.1);
    }
  }
  (previous.0, previous.1)
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

fn find_200th(asteroids: &[Position]) -> i64 {
  // fold to find max, start with 0
  let (_, data) = asteroids.iter().fold((0, None), |(prev_max, prev_data), station| {
    // take max over previous or current value
    let detectable = asteroids
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
      });

    if detectable.len() > prev_max {
      let new_max = detectable.len();
      let data = (detectable, station);
      (new_max, Some(data))
    } else {
      (prev_max, prev_data)
    }
  });

  // cheat here since we know we have > 200 asteroids detected already
  if let Some((detected, station)) = data {
    // count is correct (and station), but not exact asteroids
    // need to replace the detected with the closest asteroid
    let mut first_run: Vec<Position> = detected
      .iter()
      .map(|i| {
        asteroids.iter().fold(i.clone(), |prev_closest, x| {
          if x != station {
            attempt_min(&station, &prev_closest, &x)
          } else {
            prev_closest
          }
        })
      })
      .collect();

    // sort by angle
    first_run.sort_by(|i, j| compare(station, i, j));

    let (x, y) = first_run[200 - 1];
    x * 100 + y
  } else {
    0
  }
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

pub fn solution() {
  let input = fs::read_to_string("inputs/2019/10.txt").unwrap();
  let asteroids = parse_asteroids(&input);

  println!("Solution 1: {}", find_best_location(&asteroids));
  println!("Solution 2: {}", find_200th(&asteroids));
}
