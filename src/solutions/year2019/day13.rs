use std::collections::HashMap;

use super::int_code::{RunResult::*, State};

fn count_block_tiles(mem: &[i64]) -> usize {
  if let Halted(outputs) = State::new_with_mem(mem).run(vec![]) {
    use std::collections::HashSet;
    let mut blocks = HashSet::new();
    let mut iter = outputs.iter();
    while let Some(x) = iter.next() {
      let y = iter.next().unwrap();
      let id = iter.next().unwrap();
      if *id == 2 {
        blocks.insert((*x, *y));
      }
    }
    blocks.len()
  } else {
    0
  }
}

fn auto_play(mem: &[i64]) -> i64 {
  let mut modified = mem.to_vec();
  modified[0] = 2;
  let mut state = State::new_with_mem(&modified);
  let mut input = vec![];

  let mut map = HashMap::new();
  let mut score = 0;

  loop {
    match state.run(input) {
      WaitingForInput(new_state, outputs) => {
        input = vec![process_outputs(&outputs, &mut map, &mut score)];
        if game_finished(&map) {
          break;
        }
        state = new_state;
      }
      Halted(outputs) => {
        process_outputs(&outputs, &mut map, &mut score);
        break;
      }
    }
  }

  score
}

const BLOCK: i64 = 2;
const PADDLE: i64 = 3;
const BALL: i64 = 4;

// returns next input
fn process_outputs(outputs: &[i64], map: &mut HashMap<(i64, i64), i64>, score: &mut i64) -> i64 {
  let mut ball = (0, 0);

  // update map
  let mut iter = outputs.iter();
  while let Some(px) = iter.next() {
    let x = *px;
    let y = *iter.next().unwrap();
    let id = *iter.next().unwrap();

    if x == -1 {
      // set score
      *score = id;
    }

    map.insert((x, y), id);
    if id == BALL {
      ball = (x, y);
    }
  }

  // calculate paddle movement
  let mut paddle = (0, 0);
  for (pos, id) in map.iter() {
    if *id == PADDLE {
      paddle = *pos;
      break;
    }
  }

  use std::cmp::Ordering::*;
  match paddle.0.cmp(&ball.0) {
    Equal => 0,
    Less => 1,
    Greater => -1,
  }
}

fn game_finished(map: &HashMap<(i64, i64), i64>) -> bool {
  map.values().all(|x| *x != BLOCK)
}

pub fn solution() {
  let input = std::fs::read_to_string("inputs/2019/13.txt").unwrap();
  let mem: Vec<i64> = input.split(',').map(|x| x.trim().parse::<i64>().unwrap()).collect();

  println!("Block tiles: {}", count_block_tiles(&mem));
  println!("Score: {}", auto_play(&mem));
}
