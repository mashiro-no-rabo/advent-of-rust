use std::collections::{HashMap, VecDeque};
use std::fs;

fn parse_orbit(orbit: &str) -> (String, String) {
  let mut parts = orbit.split(')');
  let from = parts.next().unwrap().to_string();
  let to = parts.next().unwrap().to_string();
  (from, to)
}

fn part1(input: &str) -> i64 {
  let mut orbits = HashMap::new();
  orbits.insert("COM".to_string(), 0);

  let mut pairs: VecDeque<(String, String)> = input.trim().lines().map(|line| parse_orbit(&line)).collect();
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

  orbits.iter().fold(0, |acc, (_, val)| acc + val)
}

fn part2(input: &str) -> i64 {
  let pairs: Vec<(String, String)> = input.trim().lines().map(|line| parse_orbit(&line)).collect();

  let mut transfers = 0;
  let mut reachable = vec!["YOU".to_string()];
  let target = "SAN".to_string();
  let mut visited = vec![];
  while !reachable.contains(&target) {
    transfers += 1;
    // need to exclude visisted objects, since we treat orbits as undirected
    visited.extend(reachable.clone());

    reachable = reachable
      .iter()
      // expand each object to more reachable objects
      .map(|obj| {
        pairs
          .iter()
          .filter_map(|(from, to)| {
            // expand on both directions
            if from == obj && !visited.contains(to) {
              Some(to.clone())
            } else if to == obj && !visited.contains(from) {
              Some(from.clone())
            } else {
              None
            }
          })
          .collect::<Vec<String>>()
      })
      // collapse all vectors
      .flatten()
      .collect();
  }

  // excluding the first and final step
  transfers - 2
}

pub fn solution() {
  let input = fs::read_to_string("inputs/2019/6.txt").unwrap();
  println!("Total orbits: {}", part1(&input));
  println!("Transfers required: {}", part2(&input));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let input = "B)C\nCOM)B\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";

    assert_eq!(42, part1(&input));
  }

  #[test]
  fn test2() {
    let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";

    assert_eq!(4, part2(&input));
  }
}
