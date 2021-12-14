use itertools::{Itertools, MinMaxResult};
use std::{collections::HashMap, fs};

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/14.txt").unwrap();

  let mut lines = content.lines();
  let template = lines.next().unwrap();

  let pis = lines.skip(1).fold(HashMap::new(), |mut acc, line| {
    let mut parts = line.split(" -> ");
    let from = {
      let mut s = parts.next().unwrap().chars();
      let a = s.next().unwrap();
      let b = s.next().unwrap();
      (a, b)
    };
    let to = parts.next().unwrap().chars().next().unwrap();
    acc.insert(from, to);
    acc
  });

  let p1 = (0..10).fold(template.to_string(), |acc, _| {
    let mut new_acc = String::new();

    for (a, b) in acc.chars().tuple_windows() {
      new_acc.push(a);
      if let Some(c) = pis.get(&(a, b)) {
        new_acc.push(*c);
      }
    }

    new_acc.push(acc.chars().last().unwrap());
    new_acc
  });

  let count = p1.chars().fold(HashMap::new(), |mut acc, c| {
    *acc.entry(c).or_insert(0) += 1;
    acc
  });

  if let MinMaxResult::MinMax(a, z) = count.into_iter().minmax_by_key(|a| a.1) {
    println!("diff: {}", z.1 - a.1);
  }
}
