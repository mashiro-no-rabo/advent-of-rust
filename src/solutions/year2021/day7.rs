use std::{collections::HashMap, fs};

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/7.txt").unwrap();

  let mut pos_count = HashMap::new();
  content.trim().split(",").for_each(|x| {
    let p = x.parse::<u32>().unwrap();
    *pos_count.entry(p).or_insert(0u32) += 1;
  });
  let end = *pos_count.keys().max().unwrap();

  println!(
    "min fuel: {}",
    (0..=end)
      .into_iter()
      .map(|move_to| { pos_count.iter().map(|(p, c)| fuel1(*p, move_to) * c).sum::<u32>() })
      .min()
      .unwrap()
  );

  println!(
    "min fuel2: {}",
    (0..=end)
      .into_iter()
      .map(|move_to| { pos_count.iter().map(|(p, c)| fuel2(*p, move_to) * c).sum::<u32>() })
      .min()
      .unwrap()
  );
}

fn fuel1(a: u32, b: u32) -> u32 {
  a.checked_sub(b).or(b.checked_sub(a)).unwrap()
}

fn fuel2(a: u32, b: u32) -> u32 {
  if a == b {
    return 0;
  }
  let steps = a.checked_sub(b).or(b.checked_sub(a)).unwrap();
  (1 + steps) * steps / 2
}
