#![allow(dead_code)]
use std::fs;

#[derive(Debug, Clone)]
struct Board {
  nums: [u8; 25],
  called: [bool; 25],
  won: bool,
}

impl Board {
  fn from_lines<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Board {
    let mut board = Board {
      nums: [0; 25],
      called: [false; 25],
      won: false,
    };

    let mut idx = 0;
    for _ in 0..5 {
      let line = lines.next().unwrap();
      line.split_ascii_whitespace().for_each(|s| {
        board.nums[idx] = s.parse::<u8>().unwrap();
        idx += 1;
      })
    }

    board
  }

  // call a number, return whether the board won
  fn call(&mut self, c: u8) -> bool {
    if let Some(pos) = self.nums.iter().position(|&x| x == c) {
      self.called[pos] = true;
      for i in 0..5 {
        if self.row_win(i) || self.col_win(i) {
          self.won = true;
          return true;
        }
      }
    }

    false
  }

  fn row_win(&self, row: usize) -> bool {
    self.called[row * 5..(row + 1) * 5].iter().all(|&x| x)
  }

  fn col_win(&self, col: usize) -> bool {
    self.called.chunks(5).all(|row| *row.iter().nth(col).unwrap())
  }

  fn uncalled_sum(&self) -> u32 {
    let mut sum = 0;
    for i in 0..25 {
      if !self.called[i] {
        sum += self.nums[i] as u32;
      }
    }

    sum
  }
}

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/4.txt").unwrap();

  let mut lines = content.trim().lines();
  let calls = lines.next().unwrap().split(",").map(|s| s.parse::<u8>().unwrap());

  let mut boards = Vec::new();
  while lines.next().is_some() {
    boards.push(Board::from_lines(&mut lines));
  }

  let mut wins = 0;
  let bc = boards.len();
  for call in calls.clone() {
    for b in boards.iter_mut() {
      if !b.won && b.call(call) {
        if wins == 0 {
          println!("score: {}", call as u32 * b.uncalled_sum());
        }

        wins += 1;

        if wins == bc {
          println!("last win score: {}", call as u32 * b.uncalled_sum());
          return;
        }
      }
    }
  }
}
