// Could be a good place to implement ECS

use std::fs;

#[derive(Debug, Clone)]
struct Coordinates {
  x: i64,
  y: i64,
  z: i64,
}

impl Default for Coordinates {
  fn default() -> Self {
    Self { x: 0, y: 0, z: 0 }
  }
}

impl Coordinates {
  fn energy(&self) -> i64 {
    self.x.abs() + self.y.abs() + self.z.abs()
  }
}

#[derive(Debug, Clone)]
struct Moon {
  position: Coordinates,
  velocity: Coordinates,
}

impl Moon {
  fn new_static(x: i64, y: i64, z: i64) -> Self {
    let pos = Coordinates { x, y, z };
    Self {
      position: pos,
      velocity: Coordinates::default(),
    }
  }

  fn new_with_gravity(&self, others: &[Self]) -> Self {
    let mut vx = self.velocity.x;
    let mut vy = self.velocity.y;
    let mut vz = self.velocity.z;

    for other in others {
      if self.position.x < other.position.x {
        vx += 1;
      }
      if self.position.x > other.position.x {
        vx -= 1;
      }
      if self.position.y < other.position.y {
        vy += 1;
      }
      if self.position.y > other.position.y {
        vy -= 1;
      }
      if self.position.z < other.position.z {
        vz += 1;
      }
      if self.position.z > other.position.z {
        vz -= 1;
      }
    }

    Self {
      position: self.position.clone(),
      velocity: Coordinates { x: vx, y: vy, z: vz },
    }
  }

  fn apply_velocity(&mut self) {
    self.position.x += self.velocity.x;
    self.position.y += self.velocity.y;
    self.position.z += self.velocity.z;
  }
}

fn simulate(start: Vec<Moon>, steps: usize) -> Vec<Moon> {
  let mut moons = start;
  for _ in 0..steps {
    let mut new_moons = vec![];
    for i in 0..moons.len() {
      let this_moon = moons.remove(i);
      new_moons.push(this_moon.new_with_gravity(&moons));
      moons.insert(i, this_moon);
    }
    for m in new_moons.iter_mut() {
      m.apply_velocity();
    }
    moons = new_moons;
  }

  moons
}

fn total_energy(moons: &[Moon]) -> i64 {
  moons
    .iter()
    .map(|moon| moon.position.energy() * moon.velocity.energy())
    .sum()
}

fn parse_moons(input: &str) -> Vec<Moon> {
  let mut moons = vec![];
  for moon_str in input.lines() {
    let coords: Vec<i64> = moon_str
      .trim_matches(|c| c == '<' || c == '>')
      .split(',')
      .map(|co| co.split('=').last().unwrap().parse::<i64>().unwrap())
      .collect();
    let moon = Moon::new_static(coords[0], coords[1], coords[2]);
    moons.push(moon);
  }

  moons
}

fn main() {
  let input = fs::read_to_string("inputs/2019/12.txt").unwrap();
  let moons = parse_moons(&input);

  println!("Total energy: {}", total_energy(&simulate(moons, 1000)));
}
