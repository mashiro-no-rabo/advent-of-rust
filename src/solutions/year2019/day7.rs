use async_std::fs::File;
use async_std::io::BufReader;
use async_std::prelude::*;
use async_std::task;
use std::collections::VecDeque;
use std::str;

use super::int_code::{RunResult::*, State};

fn run_amplifiers_simple(mem: &[i64], settings: Vec<i64>) -> i64 {
  let mut signal = 0;
  for setting in settings {
    signal = State::new_with_mem(mem).run(vec![setting, signal]).get_first_output();
  }
  signal
}

fn run_amplifiers_feedback(mem: &[i64], settings: Vec<i64>) -> i64 {
  // Since there should be only 1 output from each amplifier after setup
  //   using a deque to pass around I/O should work
  let mut inputs = VecDeque::new();
  for x in settings {
    inputs.push_back(x);
  }
  inputs.push_back(0);

  let initial_state = State::new_with_mem(mem);
  let mut states = vec![initial_state; 5];

  loop {
    let mut new_states = vec![];
    for state in states {
      match state.run(vec![inputs.pop_front().unwrap()]) {
        WaitingForInput(s, outputs) => {
          new_states.push(s);
          if let Some(output) = outputs.first() {
            inputs.push_back(*output);
          }
        }
        Halted(outputs) => {
          inputs.push_back(*outputs.first().unwrap());
        }
      }
    }

    if new_states.is_empty() {
      break;
    } else {
      states = new_states;
    }
  }

  inputs.pop_front().unwrap()
}

pub fn solution() {
  task::block_on(async {
    let file = File::open("inputs/2019/7.txt").await.unwrap();
    let mem: Vec<i64> = BufReader::new(file)
      .split(b',')
      .filter_map(|x| x.ok())
      .filter_map(|x| str::from_utf8(&x).unwrap().trim().parse::<i64>().ok())
      .collect()
      .await;

    let mut variations1 = vec![];

    for a in 0..5 {
      for b in 0..5 {
        if b != a {
          for c in 0..5 {
            if c != a && c != b {
              for d in 0..5 {
                if d != a && d != b && d != c {
                  for e in 0..5 {
                    if e != a && e != b && e != c && e != d {
                      variations1.push(vec![a, b, c, d, e]);
                    }
                  }
                }
              }
            }
          }
        }
      }
    }

    println!(
      "Highest signal: {}",
      variations1
        .iter()
        .map(|ps| run_amplifiers_simple(&mem, ps.to_vec()))
        .max()
        .unwrap()
    );

    let mut variations2 = vec![];

    for a in 5..=9 {
      for b in 5..=9 {
        if b != a {
          for c in 5..=9 {
            if c != a && c != b {
              for d in 5..=9 {
                if d != a && d != b && d != c {
                  for e in 5..=9 {
                    if e != a && e != b && e != c && e != d {
                      variations2.push(vec![a, b, c, d, e]);
                    }
                  }
                }
              }
            }
          }
        }
      }
    }

    println!(
      "Highest signal: {}",
      variations2
        .iter()
        .map(|ps| run_amplifiers_feedback(&mem, ps.to_vec()))
        .max()
        .unwrap()
    );
  });
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let mem = vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0];
    let phase_settings = vec![4, 3, 2, 1, 0];
    assert_eq!(43210, run_amplifiers_simple(&mem, phase_settings));
  }

  #[test]
  fn test2() {
    let mem = vec![
      3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0,
    ];
    let phase_settings = vec![0, 1, 2, 3, 4];
    assert_eq!(54321, run_amplifiers_simple(&mem, phase_settings));
  }

  #[test]
  fn test3() {
    let mem = vec![
      3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31,
      4, 31, 99, 0, 0, 0,
    ];
    let phase_settings = vec![1, 0, 4, 3, 2];
    assert_eq!(65210, run_amplifiers_simple(&mem, phase_settings));
  }

  #[test]
  fn test4() {
    let mem = vec![
      3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5,
    ];
    let phase_settings = vec![9, 8, 7, 6, 5];
    assert_eq!(139_629_729, run_amplifiers_feedback(&mem, phase_settings));
  }

  #[test]
  fn test5() {
    let mem = vec![
      3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5, 54, 1105, 1, 12, 1,
      53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0,
      0, 10,
    ];
    let phase_settings = vec![9, 7, 8, 5, 6];
    assert_eq!(18216, run_amplifiers_feedback(&mem, phase_settings));
  }
}
