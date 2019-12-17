// Generic 2D map stuff

#[derive(Debug, PartialEq)]
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

  pub fn from_byte(b: u8) -> Self {
    match b {
      b'^' => North,
      b'v' => South,
      b'<' => West,
      b'>' => East,
      _ => unimplemented!(),
    }
  }

  pub fn turn(&self, t: Turn) -> Self {
    use Turn::*;
    if t == Right {
      match self {
        North => East,
        East => South,
        South => West,
        West => North,
      }
    } else {
      match self {
        North => West,
        West => South,
        South => East,
        East => North,
      }
    }
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

#[derive(Debug, Clone, PartialEq)]
pub enum Turn {
  Left,
  Right,
}

impl Turn {
  pub fn from_directions(prev: &Direction, next: &Direction) -> Self {
    use Turn::*;
    if prev.turn(Right) == *next {
      Right
    } else if prev.turn(Left) == *next {
      Left
    } else {
      unimplemented!()
    }
  }

  pub fn to_input(&self) -> String {
    use Turn::*;
    match self {
      Left => "L".to_string(),
      Right => "R".to_string(),
    }
  }
}
