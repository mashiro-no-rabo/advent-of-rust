use std::fs;

#[derive(Copy, Clone)]
enum Direction {
  North,
  South,
  East,
  West,
}
use Direction::*;

impl Direction {
  fn turn_left(&mut self, deg: i32) {
    for _ in 1..=(deg / 90) {
      *self = match *self {
        North => West,
        West => South,
        South => East,
        East => North,
      }
    }
  }

  fn turn_right(&mut self, deg: i32) {
    for _ in 1..=(deg / 90) {
      *self = match *self {
        North => East,
        East => South,
        South => West,
        West => North,
      }
    }
  }
}

struct Ship {
  x: i32,
  y: i32,
  dir: Direction,
}

impl Ship {
  fn new() -> Self {
    Self { x: 0, y: 0, dir: East }
  }

  fn forward(&mut self, val: i32) {
    match self.dir {
      North => self.x += val,
      South => self.x -= val,
      East => self.y += val,
      West => self.y -= val,
    }
  }

  fn forward_waypoint(&mut self, wp: &Waypoint, val: i32) {
    self.x += val * wp.x;
    self.y += val * wp.y;
  }
}

struct Waypoint {
  x: i32,
  y: i32,
}

impl Waypoint {
  fn turn_left(&mut self, deg: i32) {
    for _ in 1..=(deg / 90) {
      let new_x = self.y;
      self.y = -self.x;
      self.x = new_x;
    }
  }

  fn turn_right(&mut self, deg: i32) {
    for _ in 1..=(deg / 90) {
      let new_x = -self.y;
      self.y = self.x;
      self.x = new_x;
    }
  }
}

pub fn solution() {
  let content = fs::read_to_string("inputs/2020/12.txt").unwrap();
  {
    // part 1
    let mut ship = Ship::new();
    content.lines().for_each(|line| {
      let (action, val) = line.split_at(1);
      let val = val.parse::<i32>().unwrap();

      match action {
        "N" => ship.x += val,
        "S" => ship.x -= val,
        "E" => ship.y += val,
        "W" => ship.y -= val,
        "L" => ship.dir.turn_left(val),
        "R" => ship.dir.turn_right(val),
        "F" => ship.forward(val),
        _ => unimplemented!(),
      }
    });

    println!("Manhattan distance: {}", ship.x.abs() + ship.y.abs());
  }

  {
    // part 2
    let mut ship = Ship::new();
    let mut waypoint = Waypoint { x: 1, y: 10 };
    content.lines().for_each(|line| {
      let (action, val) = line.split_at(1);
      let val = val.parse::<i32>().unwrap();

      match action {
        "N" => waypoint.x += val,
        "S" => waypoint.x -= val,
        "E" => waypoint.y += val,
        "W" => waypoint.y -= val,
        "L" => waypoint.turn_left(val),
        "R" => waypoint.turn_right(val),
        "F" => ship.forward_waypoint(&waypoint, val),
        _ => unimplemented!(),
      }
    });

    println!("Manhattan distance w/ waypoint: {}", ship.x.abs() + ship.y.abs());
  }
}
