// Generic 2D map stuff

#[derive(Debug)]
pub enum Direction {
  North,
  South,
  West,
  East,
}
use Direction::*;

impl Direction {
  pub fn to_input(&self) -> i64 {
    match self {
      North => 1,
      South => 2,
      West => 3,
      East => 4,
    }
  }

  pub fn move_position(&self, pos: &Position) -> Position {
    match self {
      North => (pos.0, pos.1 - 1),
      South => (pos.0, pos.1 + 1),
      West => (pos.0 - 1, pos.1),
      East => (pos.0 + 1, pos.1),
    }
  }

  pub fn iter_all() -> DirectionIter {
    DirectionIter(0)
  }
}

pub struct DirectionIter(u8);

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

pub type Position = (i64, i64);
