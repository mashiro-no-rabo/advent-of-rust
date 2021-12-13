use itertools::Itertools;
use std::{collections::HashMap, fs};

type Pos = (u32, u32);

#[derive(Debug)]
struct Vent {
  start: Pos,
  end: Pos,
}

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/5.txt").unwrap();

  let vents: Vec<Vent> = content
    .trim()
    .lines()
    .map(|line| {
      let mut parts = line.split(" -> ");
      let start = parse_pos(&mut parts);
      let end = parse_pos(&mut parts);

      Vent { start, end }
    })
    .collect();

  // map of Pos to number of vents
  let mut grid = HashMap::new();
  let mut grid1 = grid.clone();
  vents.iter().for_each(|vent| {
    if vent.start.0 == vent.end.0 {
      let a = vent.start.1.min(vent.end.1);
      let b = vent.start.1.max(vent.end.1);

      for y in a..=b {
        count_grid(&mut grid, (vent.start.0, y));
        count_grid(&mut grid1, (vent.start.0, y));
      }
    } else if vent.start.1 == vent.end.1 {
      let a = vent.start.0.min(vent.end.0);
      let b = vent.start.0.max(vent.end.0);

      for x in a..=b {
        count_grid(&mut grid, (x, vent.start.1));
        count_grid(&mut grid1, (x, vent.start.1));
      }
    } else {
      // diagonal
      let x_change = if vent.start.0 < vent.end.0 {
        std::ops::Add::add
      } else {
        std::ops::Sub::sub
      };
      let y_change = if vent.start.1 < vent.end.1 {
        std::ops::Add::add
      } else {
        std::ops::Sub::sub
      };

      let mut cur = vent.start;
      loop {
        count_grid(&mut grid, cur);
        if cur == vent.end {
          break;
        }
        cur = (x_change(cur.0, 1), y_change(cur.1, 1));
      }
    }
  });

  println!("overlap: {}", grid1.values().filter(|n| **n > 1).count());
  println!("all overlaps: {}", grid.values().filter(|n| **n > 1).count());
}

fn parse_pos<'a>(str_iter: &mut impl Iterator<Item = &'a str>) -> Pos {
  str_iter
    .next()
    .unwrap()
    .split(",")
    .map(|x| x.parse().unwrap())
    .collect_tuple()
    .unwrap()
}

fn count_grid(grid: &mut HashMap<Pos, u32>, pos: Pos) {
  *grid.entry(pos).or_insert(0) += 1;
}
