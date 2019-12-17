use std::collections::HashMap;

use super::int_code::{RunResult::*, State};
use super::map::*;

fn bfs_min_steps(st: State) -> i64 {
  let mut steps = 0;

  let start_pos = (0, 0);
  let mut map = HashMap::new();
  map.insert(start_pos, 0);

  let mut search = vec![];
  search.push((start_pos, st));

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

fn bfs_all(st: State) -> i64 {
  let mut steps = 0;

  let start_pos = (0, 0);
  let mut map = HashMap::new();
  map.insert(start_pos, 0);

  let mut search = vec![];
  search.push((start_pos, st));

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
  let state = State::from_file("inputs/2019/15.txt");

  println!("Steps: {}", bfs_min_steps(state.clone()));
  println!("Time: {}", bfs_all(state));
}
