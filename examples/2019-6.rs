use std::collections::{HashMap, VecDeque};
use std::fs;

fn parse_orbit(orbit: &str) -> (String, String) {
  let mut parts = orbit.split(')');
  let from = parts.next().unwrap().to_string();
  let to = parts.next().unwrap().to_string();
  (from, to)
}

fn part1(input: &str) -> i32 {
  // since this is a tree structure, we just record the distance from root
  let mut orbits = HashMap::new();
  orbits.insert("COM".to_string(), 0);

  let mut pairs: VecDeque<(String, String)> = input.trim().lines().map(|line| parse_orbit(&line)).collect();
  // need this to work with random ordering
  while !pairs.is_empty() {
    let (from, to) = pairs.pop_front().unwrap();

    // unwraps the reference return from HashMap
    let parent_count = match orbits.get(&from) {
      Some(parent_count) => Some(*parent_count),
      None => None,
    };

    if let Some(pc) = parent_count {
      orbits.insert(to, pc + 1);
    } else {
      pairs.push_back((from, to));
    }
  }

  // and sum all at the end
  orbits.iter().fold(0, |acc, (_, val)| acc + val)
}

fn main() {
  let input = fs::read_to_string("inputs/2019/6.txt").unwrap();
  println!("Total orbits: {}", part1(&input));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let input = "B)C\nCOM)B\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";

    assert_eq!(42, part1(&input));
  }
}
