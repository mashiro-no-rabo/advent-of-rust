use std::{collections::HashSet, fs};

type Axis = (i32, i32);

struct Cube {
  x: Axis,
  y: Axis,
  z: Axis,
}

struct OnCube {
  cube: Cube,
  offs: Vec<Cube>,
}

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/22.txt").unwrap();

  println!(
    "init cubes: {}",
    content
      .lines()
      .fold(HashSet::new(), |mut acc, line| {
        let mut parts = line.split_ascii_whitespace();
        let turn_on = parts.next().unwrap() == "on";
        let mut dims = parts.next().unwrap().split(",");
        let (x1, x2) = parse_dim(dims.next().unwrap());
        if (x1 > 50) || (x2 < -50) {
          return acc;
        }
        let (y1, y2) = parse_dim(dims.next().unwrap());
        if (y1 > 50) || (y2 < -50) {
          return acc;
        }
        let (z1, z2) = parse_dim(dims.next().unwrap());
        if (z1 > 50) || (z2 < -50) {
          return acc;
        }
        for x in x1..=x2 {
          for y in y1..=y2 {
            for z in z1..=z2 {
              if x >= -50 && x <= 50 && y >= -50 && y <= 50 && z >= -50 && z <= 50 {
                if turn_on {
                  acc.insert((x, y, z));
                } else {
                  acc.remove(&(x, y, z));
                }
              }
            }
          }
        }

        acc
      })
      .len(),
  );
}

fn parse_dim(dim: &str) -> Axis {
  let mut parts = dim.split_at(2).1.split("..");
  let start = parts.next().unwrap().parse::<i32>().unwrap();
  let end = parts.next().unwrap().parse::<i32>().unwrap();
  (start, end)
}
