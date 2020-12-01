use std::collections::HashSet;
use std::fs;

pub fn solution() {
  let content = fs::read_to_string("inputs/2020/1.txt").unwrap();

  let mut expecting = HashSet::new();
  let mut multiply = None;

  for line in content.lines() {
    let x = line.parse::<u32>().unwrap();
    if expecting.contains(&x) {
      multiply = Some(x * (2020 - x));
      break;
    }

    expecting.insert(2020 - x);
  }

  println!("Multiply: {}", multiply.unwrap());
}
