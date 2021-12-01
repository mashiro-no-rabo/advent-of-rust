use itertools::Itertools;
use std::fs;

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/1.txt").unwrap();

  {
    let mut lines = content.trim().lines();
    let first = lines.next().unwrap().parse::<u32>().unwrap();

    let incs = lines.fold((first, 0), |prev, line| {
      let cur = line.parse::<u32>().unwrap();
      if cur > prev.0 {
        (cur, prev.1 + 1)
      } else {
        (cur, prev.1)
      }
    });
    println!("Increases: {}", incs.1);
  }

  {
    let mut wins = content
      .trim()
      .lines()
      .map(|line| line.parse::<u32>().unwrap())
      .tuple_windows::<(_, _, _)>();
    let first = wins.next().unwrap();
    let incs = wins.fold((first, 0), |prev, win| {
      if win_sum(win) > win_sum(prev.0) {
        (win, prev.1 + 1)
      } else {
        (win, prev.1)
      }
    });

    println!("Increases with sliding window: {}", incs.1);
  }
}

fn win_sum((a, b, c): (u32, u32, u32)) -> u32 {
  a + b + c
}
