use std::fs;

#[derive(Debug, Clone, Copy)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Debug, Clone, Copy)]
struct Step {
  len: i64,
  direction: Direction,
}

fn parse_step(step_str: &str) -> Step {
  let (dir, num) = step_str.split_at(1);

  use Direction::*;
  let direction = match dir {
    "U" => Up,
    "D" => Down,
    "L" => Left,
    "R" => Right,
    _ => unimplemented!(),
  };

  let len = num.parse().unwrap();

  Step { len, direction }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Orientation {
  Horizontal,
  Vertical,
}

// Line is either horizontal or vertical, thus location on one axis is fixed
//   and record start + end for the other axis
#[derive(Debug, Clone, Copy)]
struct Line {
  orient: Orientation,
  start: i64,
  end: i64,
  fixed_axis: i64,
}

#[derive(Debug, Clone, Copy)]
struct Location {
  x: i64,
  y: i64,
}

fn build_line(cur_loc: &Location, step: Step) -> Line {
  use Direction::*;
  use Orientation::*;

  let (orient, start, fixed_axis) = match step.direction {
    Up | Down => (Vertical, cur_loc.y, cur_loc.x),
    Left | Right => (Horizontal, cur_loc.x, cur_loc.y),
  };

  let end = match step.direction {
    Up | Right => start + step.len,
    Down | Left => start - step.len,
  };

  Line {
    orient,
    start,
    end,
    fixed_axis,
  }
}

// Wire is a series of lines
type Wire = Vec<Line>;

fn build_wire(wire_str: &str) -> Wire {
  let central_port = Location { x: 0, y: 0 };
  wire_str
    .split(',')
    .scan(central_port, |cur_loc, step_str| {
      let step = parse_step(step_str);
      let line = build_line(cur_loc, step);
      use Direction::*;
      match step.direction {
        Up | Down => {
          cur_loc.x = line.fixed_axis;
          cur_loc.y = line.end;
          Some(line)
        }
        Left | Right => {
          cur_loc.x = line.end;
          cur_loc.y = line.fixed_axis;
          Some(line)
        }
      }
    })
    .collect()
}

#[allow(clippy::ptr_arg)]
fn find_min_manhattan(wire1: &Wire, wire2: &Wire) -> i64 {
  let mut closest = std::i64::MAX;

  for line1 in wire1 {
    for line2 in wire2 {
      if (line1.orient != line2.orient)
      // start/end can be reverse of min/max, which is what we really want here
        && (line1.fixed_axis >= line2.start.min(line2.end))
        && (line1.fixed_axis <= line2.end.max(line2.start))
        && (line2.fixed_axis >= line1.start.min(line1.end))
        && (line2.fixed_axis <= line1.end.max(line1.start))
      {
        // line crosses, the location can be determined by the 2 fixed_axis
        // since manhattan distance doesn't care about exact point, just add them together
        let dist = line1.fixed_axis.abs() + line2.fixed_axis.abs();

        // remember the central port doesn't count
        if (dist > 0) && (dist < closest) {
          closest = dist
        }
      }
    }
  }

  closest
}

fn main() {
  println!("Advent of Rust!");

  let input = fs::read_to_string("inputs/2019/3.txt").unwrap();
  let mut lines = input.lines();
  let wire1 = build_wire(lines.next().unwrap());
  let wire2 = build_wire(lines.next().unwrap());

  println!("Solution 1: {}", find_min_manhattan(&wire1, &wire2));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let wire1 = build_wire("R8,U5,L5,D3");
    let wire2 = build_wire("U7,R6,D4,L4");

    assert_eq!(6, find_min_manhattan(&wire1, &wire2));
  }

  #[test]
  fn test2() {
    let wire1 = build_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    let wire2 = build_wire("U62,R66,U55,R34,D71,R55,D58,R83");

    assert_eq!(159, find_min_manhattan(&wire1, &wire2));
  }

  #[test]
  fn test3() {
    let wire1 = build_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    let wire2 = build_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

    assert_eq!(135, find_min_manhattan(&wire1, &wire2));
  }
}
