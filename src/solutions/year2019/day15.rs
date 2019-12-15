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
    let mut found = false;

    search
      .iter()
      .flat_map(|(pos, state)| Direction::iter_all().map(move |dir| (dir.move_position(&pos), dir, state)))
      .for_each(|(new_pos, dir, in_state)| {
        if map.get(&new_pos).is_none() {
          if let WaitingForInput(out_state, outputs) = in_state.run(vec![dir.to_input()]) {
            match outputs[0] {
              0 => {
                map.insert(new_pos, -1);
              }
              1 => {
                map.insert(new_pos, steps);
                new_search.push((new_pos, out_state));
              }
              2 => found = true,
              _ => unimplemented!(),
            }
          }
        }
      });

    if found {
      return steps;
    }

    search = new_search;
  }
}

fn bfs_all(mem: &[i64]) -> i64 {
  let mut steps = 0;

  let start_pos = (0, 0);
  let mut map = HashMap::new();
  map.insert(start_pos, 0);

  let mut search = vec![];
  let start_state = State::new_with_mem(mem);
  search.push((start_pos, start_state));

  let mut oxygen = (0, 0);

  while !search.is_empty() {
    steps += 1;

    let mut new_search = vec![];

    search
      .iter()
      .flat_map(|(pos, state)| Direction::iter_all().map(move |dir| (dir.move_position(&pos), dir, state)))
      .for_each(|(new_pos, dir, in_state)| {
        if map.get(&new_pos).is_none() {
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
                oxygen = new_pos;
                map.insert(new_pos, steps);
                new_search.push((new_pos, out_state));
              }
              _ => unimplemented!(),
            }
          }
        }
      });

    search = new_search;
  }

  let mut fill = vec![];
  fill.push(oxygen);
  let mut minutes = 0;
  let mut real_minutes = 0;

  while !fill.is_empty() {
    minutes += 1;

    let mut new_fill = vec![];

    fill
      .iter()
      .flat_map(|pos| Direction::iter_all().map(move |dir| (dir.move_position(&pos))))
      .for_each(|new_pos| {
        if let Some(x) = map.get_mut(&new_pos) {
          if *x > -1 {
            *x = -1;
            new_fill.push(new_pos);
            real_minutes = minutes;
          }
        }
      });

    fill = new_fill;
  }

  real_minutes
}

pub fn solution() {
  let input = std::fs::read_to_string("inputs/2019/15.txt").unwrap();
  let mem: Vec<i64> = input.split(',').map(|x| x.trim().parse::<i64>().unwrap()).collect();

  println!("Steps: {}", bfs_min_steps(&mem));
  println!("Time: {}", bfs_all(&mem));
}
