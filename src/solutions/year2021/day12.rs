use std::{collections::HashMap, fs};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Cave {
  Big(String),
  Small(String),
  Double,
}

impl Cave {
  fn from_str(s: &str) -> Self {
    if s == s.to_uppercase() {
      Cave::Big(s.to_string())
    } else {
      Cave::Small(s.to_string())
    }
  }

  fn is_small(&self) -> bool {
    match self {
      Cave::Small(_) => true,
      _ => false,
    }
  }
}

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/12.txt").unwrap();

  let mut conns = HashMap::new();

  content.trim().lines().for_each(|line| {
    let mut parts = line.split("-");
    let a = Cave::from_str(parts.next().unwrap());
    let b = Cave::from_str(parts.next().unwrap());

    conns.entry(a.clone()).or_insert(Vec::new()).push(b.clone());
    conns.entry(b).or_insert(Vec::new()).push(a);
  });

  println!(
    "paths: {}",
    visit1(&Cave::Small("start".to_string()), Vec::new(), &conns)
  );

  println!(
    "paths2: {}",
    visit2(&Cave::Small("start".to_string()), Vec::new(), &conns)
  );
}

fn visit1(node: &Cave, mut visited: Vec<Cave>, conns: &HashMap<Cave, Vec<Cave>>) -> u32 {
  if node.is_small() {
    visited.push(node.clone());
  }

  conns.get(node).unwrap().iter().fold(0, |acc, n| {
    if *n == Cave::Small("end".to_string()) {
      acc + 1
    } else {
      if n.is_small() && visited.contains(n) {
        acc
      } else {
        acc + visit1(n, visited.clone(), conns)
      }
    }
  })
}

fn visit2(node: &Cave, mut visited: Vec<Cave>, conns: &HashMap<Cave, Vec<Cave>>) -> u32 {
  if node.is_small() {
    if visited.contains(node) {
      visited.push(Cave::Double);
    } else {
      visited.push(node.clone());
    }
  }

  conns.get(node).unwrap().iter().fold(0, |acc, n| {
    if *n == Cave::Small("end".to_string()) {
      acc + 1
    } else {
      if *n == Cave::Small("start".to_string()) {
        acc
      } else if n.is_small() && visited.contains(n) && visited.contains(&Cave::Double) {
        acc
      } else {
        acc + visit2(n, visited.clone(), conns)
      }
    }
  })
}
