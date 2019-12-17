use super::int_code::{RunResult::*, State};
use super::map::*;

fn align_params(st: &State) -> u64 {
  let mut scafs = vec![];

  if let Halted(outputs) = st.run(vec![]) {
    let mut y = 0;
    let mut x = 0;
    outputs.into_iter().for_each(|val| match val as u8 {
      b'#' | b'<' | b'>' | b'^' | b'v' => {
        scafs.push((x, y));
        x += 1;
      }
      b'\n' => {
        y += 1;
        x = 0;
      }
      _ => x += 1,
    })
  }

  scafs.clone().into_iter().fold(0, |acc, (x, y)| {
    if Direction::iter_all().all(|dir| scafs.contains(&dir.move_position(&(x, y)))) {
      acc + (x * y) as u64
    } else {
      acc
    }
  })
}

type Steps = u8;
#[derive(Debug, Clone, PartialEq)]
enum RobotMove {
  Raw(Turn, Steps),
  RoutineA,
  RoutineB,
  RoutineC,
}
use RobotMove::*;

type RobotMoves = Vec<RobotMove>;
type Routine = RobotMoves; // but only has Move()

impl RobotMove {
  fn valid_main(moves: &[RobotMove]) -> bool {
    moves.iter().all(|m| if let Raw(_, _) = m { false } else { true }) && moves.len() <= 10
  }

  fn display(moves: &[RobotMove]) -> String {
    moves
      .iter()
      .map(|m| match m {
        Raw(t, s) => {
          let mut out = t.to_input();
          out.push_str(&s.to_string());
          out
        }
        RoutineA => "A".to_string(),
        RoutineB => "B".to_string(),
        RoutineC => "C".to_string(),
      })
      .collect::<Vec<String>>()
      .as_slice()
      .join(",")
  }

  fn extract_routine(moves: &[RobotMove], r: RobotMove, len: usize) -> Option<(Routine, RobotMoves)> {
    if let Raw(_, _) = r {
      panic!("not extracting routine!")
    }

    let mut ret_moves = vec![];

    // strip off previous routines
    let mut routine: RobotMoves = moves.to_vec();
    while !routine.is_empty() {
      if let Raw(_, _) = routine[0] {
        break;
      } else {
        ret_moves.push(routine.remove(0));
      }
    }

    if len > moves.len() {
      return None;
    }

    // routine can't contain another routine
    if (0..len).any(|i| if let Raw(_, _) = routine[i] { false } else { true }) {
      return None;
    }

    // take the new routine
    let mut new_moves = routine.split_off(len);
    // and replace in-place
    new_moves.insert(0, r.clone());

    // replace any repeated moves with the new routine
    let mut idx = 1;
    while idx + len <= new_moves.len() {
      // new_moves may change length, so not using for-loop
      if (0..len).all(|j| new_moves[idx + j] == routine[j]) {
        (0..len).for_each(|_| {
          new_moves.remove(idx);
        });

        new_moves.insert(idx, r.clone());
      }

      idx += 1;
    }

    ret_moves.extend_from_slice(&new_moves);

    Some((routine, ret_moves))
  }

  fn intcode_input(moves: RobotMoves) -> Vec<i64> {
    let mut out: Vec<i64> = moves
      .iter()
      .flat_map(|m| match m {
        Raw(t, s) => {
          let mut out = t.to_input();
          out.push(',');
          out.push_str(&s.to_string());
          out.push(',');
          out.into_bytes()
        }
        RoutineA => vec![b'A', b','],
        RoutineB => vec![b'B', b','],
        RoutineC => vec![b'C', b','],
      })
      .map(|x| x as i64)
      .collect();

    // replace final comma with newline
    out.pop();
    out.push(b'\n' as i64);
    out
  }
}

fn collect_dust(st: &State) -> i64 {
  use std::collections::HashMap;
  use Direction::*;

  let mut scafs = HashMap::new();
  let mut robot_pos = (0, 0);
  let mut robot_dir = North;

  if let Halted(outputs) = st.run(vec![]) {
    println!(
      "Map\n===\n{}",
      String::from_utf8(outputs.clone().into_iter().map(|x| x as u8).collect::<Vec<u8>>()).unwrap()
    );

    let mut y = 0;
    let mut x = 0;
    outputs.into_iter().map(|x| x as u8).for_each(|val| match val {
      b'#' => {
        scafs.insert((x, y), false);
        x += 1;
      }
      b'<' | b'>' | b'^' | b'v' => {
        robot_pos = (x, y);
        robot_dir = Direction::from_byte(val);
        x += 1;
      }
      b'\n' => {
        y += 1;
        x = 0;
      }
      _ => x += 1,
    })
  }

  // build moves
  let mut moves = vec![];
  while scafs.iter().any(|(_, visited)| !visited) {
    // find which way to turn
    let next_dir = Direction::iter_all()
      .find(|dir| {
        scafs
          .get(&dir.move_position(&robot_pos))
          .map_or(false, |visited| !visited)
      })
      .unwrap();
    let turn = Turn::from_directions(&robot_dir, &next_dir);

    // walk until end of line
    let mut steps = 0;
    let mut next_pos = robot_pos;
    loop {
      let new_pos = next_dir.move_position(&next_pos);
      if let Some(visited) = scafs.get_mut(&new_pos) {
        *visited = true;
        steps += 1;
        next_pos = new_pos;
      } else {
        break;
      }
    }

    moves.push(Raw(turn, steps));
    robot_dir = next_dir;
    robot_pos = next_pos;
  }

  println!("Raw Moves: {}", RobotMove::display(&moves));

  // extract routines from moves
  let mut ra: Routine = vec![];
  let mut rb: Routine = vec![];
  let mut rc: Routine = vec![];
  let mut main: RobotMoves = vec![];

  // 20 characters = 10 raw move commands
  for la in 1..=10 {
    if let Some((a, amoves)) = RobotMove::extract_routine(&moves, RoutineA, la) {
      ra = a;
      for lb in 1..=10 {
        if let Some((b, bmoves)) = RobotMove::extract_routine(&amoves, RoutineB, lb) {
          rb = b;
          for lc in 1..=10 {
            if let Some((c, cmoves)) = RobotMove::extract_routine(&bmoves, RoutineC, lc) {
              rc = c;
              if RobotMove::valid_main(&cmoves) {
                main = cmoves;
                break;
              }
            }
          }

          if !main.is_empty() {
            break;
          }
        }
      }

      if !main.is_empty() {
        break;
      }
    }
  }

  let mut inputs = vec![];
  inputs.extend_from_slice(&RobotMove::intcode_input(main));
  inputs.extend_from_slice(&RobotMove::intcode_input(ra));
  inputs.extend_from_slice(&RobotMove::intcode_input(rb));
  inputs.extend_from_slice(&RobotMove::intcode_input(rc));
  inputs.extend_from_slice(&[b'n' as i64, b'\n' as i64]);

  // run program
  let mut rescue_robot = st.clone();
  rescue_robot.patch_memory(0, 2);
  if let Halted(outputs) = rescue_robot.run(inputs) {
    // can't be bothered to deal with any other output
    *outputs.last().unwrap()
  } else {
    0
  }
}

pub fn solution() {
  let state = State::from_file("inputs/2019/17.txt");

  println!("Alignment Parameters: {}", align_params(&state));
  println!("Space Dust: {}", collect_dust(&state));
}
