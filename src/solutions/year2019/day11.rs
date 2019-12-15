use std::collections::HashMap;

use super::int_code::{RunResult::*, State};

type Position = (i64, i64);

fn turn_and_move(pos: Position, dir: u8, turn: u8) -> (Position, u8) {
  let new_dir = match turn {
    0 => (dir + 3) % 4,
    1 => (dir + 1) % 4,
    _ => unimplemented!(),
  };

  let new_pos = match new_dir {
    0 => (pos.0, pos.1 + 1),
    1 => (pos.0 + 1, pos.1),
    2 => (pos.0, pos.1 - 1),
    3 => (pos.0 - 1, pos.1),
    _ => unimplemented!(),
  };

  (new_pos, new_dir)
}

fn paint_count(mem: &[i64]) -> usize {
  let mut painted = HashMap::new();
  let mut pos = (0, 0);
  let mut dir = 0; // up, right, down, left: 0, 1, 2, 3
  let mut state = State::new_with_mem(mem);

  loop {
    let current_color = *painted.get(&pos).unwrap_or(&0);
    match state.run(vec![current_color]) {
      WaitingForInput(new_state, outputs) => {
        state = new_state;
        let color = outputs[0];
        let turn = outputs[1] as u8;
        painted.insert(pos, color);
        let (new_pos, new_dir) = turn_and_move(pos, dir, turn);
        pos = new_pos;
        dir = new_dir;
      }
      Halted(_) => break,
    }
  }

  painted.len()
}

fn paint_registration_identifier(mem: &[i64]) {
  let mut painted = HashMap::new();
  let mut pos = (0, 0);
  let mut dir = 0; // up, right, down, left: 0, 1, 2, 3
  let mut state = State::new_with_mem(mem);

  painted.insert(pos, 1);

  loop {
    let current_color = *painted.get(&pos).unwrap_or(&0);
    match state.run(vec![current_color]) {
      WaitingForInput(new_state, outputs) => {
        state = new_state;
        let color = outputs[0];
        let turn = outputs[1] as u8;
        painted.insert(pos, color);
        let (new_pos, new_dir) = turn_and_move(pos, dir, turn);
        pos = new_pos;
        dir = new_dir;
      }
      Halted(_) => break,
    }
  }

  let minx = painted.keys().map(|pos| pos.0).min().unwrap();
  let maxx = painted.keys().map(|pos| pos.0).max().unwrap();
  let miny = painted.keys().map(|pos| pos.1).min().unwrap();
  let maxy = painted.keys().map(|pos| pos.1).max().unwrap();

  // need to reverse Y axis
  for y in -maxy..=-miny {
    for x in minx..=maxx {
      if *painted.get(&(x, -y)).unwrap_or(&0) == 1 {
        print!("#");
      } else {
        print!(" ");
      }
    }

    println!();
  }
}

pub fn solution() {
  let input = std::fs::read_to_string("inputs/2019/11.txt").unwrap();
  let mem: Vec<i64> = input.split(',').map(|x| x.trim().parse::<i64>().unwrap()).collect();

  println!("Panels painted: {}", paint_count(&mem));
  paint_registration_identifier(&mem);
}
