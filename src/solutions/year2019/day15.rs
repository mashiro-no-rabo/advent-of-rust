use std::collections::HashMap;

use super::int_code::{RunResult::*, State};

#[derive(Debug)]
enum Direction {
  North,
  South,
  West,
  East,
}
use Direction::*;

impl Direction {
  fn to_input(&self) -> i64 {
    match self {
      North => 1,
      South => 2,
      West => 3,
      East => 4,
    }
  }

  fn move_position(&self, pos: &Position) -> Position {
    match self {
      North => (pos.0, pos.1 - 1),
      South => (pos.0, pos.1 + 1),
      West => (pos.0 - 1, pos.1),
      East => (pos.0 + 1, pos.1),
    }
  }

  fn iter_all() -> DirectionIter {
    DirectionIter(0)
  }
}

struct DirectionIter(u8);

impl Iterator for DirectionIter {
  type Item = Direction;

  fn next(&mut self) -> Option<Self::Item> {
    self.0 += 1;

    match self.0 {
      1 => Some(North),
      2 => Some(South),
      3 => Some(West),
      4 => Some(East),
      _ => None,
    }
  }
}

type Position = (i64, i64);

fn bfs_min_steps(mem: &[i64]) -> i64 {
  let mut steps = 0;

  let start_pos = (0, 0);
  let mut map = HashMap::new();
  map.insert(start_pos, 0);

  let mut search = vec![];
  let start_state = State::new_with_mem(mem);
  search.push((start_pos, start_state));

  loop {
    steps += 1;

    let mut new_search = vec![];

    let attempts = search
      .iter()
      .flat_map(|(pos, state)| Direction::iter_all().map(move |dir| (dir.move_position(&pos), dir, state)));

    for (new_pos, dir, in_state) in attempts {
      if map.get(&new_pos).is_some() {
        continue;
      }

      if let WaitingForInput(out_state, outputs) = in_state.run(vec![dir.to_input()]) {
        match outputs[0] {
          0 => {
            map.insert(new_pos, -1);
          }
          1 => {
            map.insert(new_pos, steps);
            new_search.push((new_pos, out_state));
          }
          2 => {
            return steps;
          }
          _ => unimplemented!(),
        }
      }
    }

    search = new_search;
  }
}

pub fn solution() {
  let input = std::fs::read_to_string("inputs/2019/15.txt").unwrap();
  let mem: Vec<i64> = input.split(',').map(|x| x.trim().parse::<i64>().unwrap()).collect();

  println!("Steps: {}", bfs_min_steps(&mem));
}
