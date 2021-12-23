use std::{collections::HashMap, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Amp {
  Empty,
  Amber,
  Bronze,
  Copper,
  Desert,
}

impl Amp {
  fn from_char(c: char) -> Option<Amp> {
    match c {
      '.' => Some(Amp::Empty),
      'A' => Some(Amp::Amber),
      'B' => Some(Amp::Bronze),
      'C' => Some(Amp::Copper),
      'D' => Some(Amp::Desert),
      _ => None,
    }
  }

  fn target_col(&self) -> u8 {
    match self {
      Amp::Amber => 2,
      Amp::Bronze => 4,
      Amp::Copper => 6,
      Amp::Desert => 8,
      _ => unimplemented!(),
    }
  }

  fn move_cost(&self, src: Pos, dst: Pos) -> u32 {
    let steps = if src.1 == dst.1 {
      // same col (should only ever move down)
      dst.0 - src.0
    } else {
      // move to hallway first, then columns
      src.0 + dst.0 + src.1.max(dst.1) - src.1.min(dst.1)
    };

    let mul = match self {
      Amp::Amber => 1,
      Amp::Bronze => 10,
      Amp::Copper => 100,
      Amp::Desert => 1000,
      _ => unimplemented!(),
    };

    steps as u32 * mul
  }
}

type Pos = (u8, u8);
type Grid = HashMap<Pos, Amp>;

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/23.txt").unwrap();

  let mut grid = Grid::new();
  content.lines().skip(1).enumerate().for_each(|(row, line)| {
    line.chars().skip(1).enumerate().for_each(|(col, c)| {
      if let Some(amp) = Amp::from_char(c) {
        grid.insert((row as u8, col as u8), amp);
      }
    });
  });

  println!("min cost: {}", min_cost(grid.clone()).unwrap());

  let a = grid.insert((2, 2), Amp::Desert).unwrap();
  grid.insert((4, 2), a);
  let a = grid.insert((2, 4), Amp::Copper).unwrap();
  grid.insert((4, 4), a);
  let a = grid.insert((2, 6), Amp::Bronze).unwrap();
  grid.insert((4, 6), a);
  let a = grid.insert((2, 8), Amp::Amber).unwrap();
  grid.insert((4, 8), a);
  grid.insert((3, 2), Amp::Desert);
  grid.insert((3, 4), Amp::Bronze);
  grid.insert((3, 6), Amp::Amber);
  grid.insert((3, 8), Amp::Copper);

  println!("min cost4: {}", min_cost4(grid.clone()).unwrap());
}

fn complete(grid: &Grid) -> bool {
  *grid.get(&(1, 2)).unwrap() == Amp::Amber
    && *grid.get(&(2, 2)).unwrap() == Amp::Amber
    && *grid.get(&(1, 4)).unwrap() == Amp::Bronze
    && *grid.get(&(2, 4)).unwrap() == Amp::Bronze
    && *grid.get(&(1, 6)).unwrap() == Amp::Copper
    && *grid.get(&(2, 6)).unwrap() == Amp::Copper
    && *grid.get(&(1, 8)).unwrap() == Amp::Desert
    && *grid.get(&(2, 8)).unwrap() == Amp::Desert
}

const VALID_HALLWAY: [u8; 7] = [0, 1, 3, 5, 7, 9, 10];

fn valid_moves(grid: &Grid) -> Vec<(Pos, Pos)> {
  let mut ret = Vec::new();

  // check amps in the hallway
  for col in VALID_HALLWAY {
    let src = (0, col);
    let amp = *grid.get(&src).unwrap();
    if amp != Amp::Empty {
      let tc = amp.target_col();
      let (l, r) = if col < tc { (col + 1, tc) } else { (tc, col - 1) };
      // the hallway to target must be empty
      if (l..r).all(|c| *grid.get(&(0, c)).unwrap() == Amp::Empty) {
        if *grid.get(&(1, tc)).unwrap() == Amp::Empty {
          let inner = *grid.get(&(2, tc)).unwrap();
          if inner == Amp::Empty {
            // if both target spots are empty, it must go to the inner one otherwise it will block
            ret.push((src, (2, tc)));
          } else if inner == amp {
            // if inner is also the target, we can go, otherwise we'll block it
            ret.push((src, (1, tc)));
          }
        }
      }
    }
  }

  // check amps in the outer burrow (row = 1)
  for col in [2, 4, 6, 8] {
    let src = (1, col);
    let amp = *grid.get(&src).unwrap();

    if amp != Amp::Empty {
      let tc = amp.target_col();
      if (tc == col) && (*grid.get(&(2, col)).unwrap() == amp) {
        // this burrow is done
        continue;
      }

      // try move to target position first
      let mut skip_hallway = false;
      let (l, r) = if col < tc { (col + 1, tc) } else { (tc, col - 1) };
      if (l..=r).all(|c| *grid.get(&(0, c)).unwrap() == Amp::Empty) {
        if *grid.get(&(1, tc)).unwrap() == Amp::Empty {
          let inner = *grid.get(&(2, tc)).unwrap();
          if inner == Amp::Empty {
            // if both target spots are empty, it must go to the inner one otherwise it will block
            ret.push((src, (2, tc)));
            skip_hallway = true;
          } else if inner == amp {
            // if inner is also the target, we can go, otherwise we'll block it
            ret.push((src, (1, tc)));
            skip_hallway = true;
          }
        }
      }

      // if not, find valid positions in hallway
      if !skip_hallway {
        // right
        VALID_HALLWAY
          .clone()
          .into_iter()
          .skip_while(|&c| c < col)
          .take_while(|&c| *grid.get(&(0, c)).unwrap() == Amp::Empty)
          .for_each(|c| ret.push((src, (0, c))));

        // left
        VALID_HALLWAY
          .clone()
          .into_iter()
          .rev()
          .skip_while(|&c| c > col)
          .take_while(|&c| *grid.get(&(0, c)).unwrap() == Amp::Empty)
          .for_each(|c| ret.push((src, (0, c))));
      }
    }
  }

  // check amps in the inner burrow (row = 2)
  for col in [2, 4, 6, 8] {
    let src = (2, col);
    let amp = *grid.get(&src).unwrap();

    if amp != Amp::Empty
    // don't move amps already in final position
    && amp.target_col() != col
    // the outer burrow must be empty
    && *grid.get(&(1, col)).unwrap() == Amp::Empty
    {
      let tc = amp.target_col();
      // try move to target position first
      let mut skip_hallway = false;
      let (l, r) = if col < tc { (col + 1, tc) } else { (tc, col - 1) };
      if (l..=r).all(|c| *grid.get(&(0, c)).unwrap() == Amp::Empty) {
        if *grid.get(&(1, tc)).unwrap() == Amp::Empty {
          let inner = *grid.get(&(2, tc)).unwrap();
          if inner == Amp::Empty {
            // if both target spots are empty, it must go to the inner one otherwise it will block
            ret.push((src, (2, tc)));
            skip_hallway = true;
          } else if inner == amp {
            // if inner is also the target, we can go, otherwise we'll block it
            ret.push((src, (1, tc)));
            skip_hallway = true;
          }
        }
      }

      // if not, find valid positions in hallway
      if !skip_hallway {
        // right
        VALID_HALLWAY
          .clone()
          .into_iter()
          .skip_while(|&c| c < col)
          .take_while(|&c| *grid.get(&(0, c)).unwrap() == Amp::Empty)
          .for_each(|c| ret.push((src, (0, c))));

        // left
        VALID_HALLWAY
          .clone()
          .into_iter()
          .rev()
          .skip_while(|&c| c > col)
          .take_while(|&c| *grid.get(&(0, c)).unwrap() == Amp::Empty)
          .for_each(|c| ret.push((src, (0, c))));
      }
    }
  }

  ret
}

fn min_cost(grid: Grid) -> Option<u32> {
  min_cost_prune(grid, 0, None)
}

fn min_cost_prune(grid: Grid, cur_cost: u32, mut cur_min: Option<u32>) -> Option<u32> {
  if complete(&grid) {
    return Some(cur_cost);
  }

  for (src, dst) in valid_moves(&grid) {
    let mut new_grid = grid.clone();
    let amp = new_grid.insert(src, Amp::Empty).unwrap();
    let new_cost = cur_cost + amp.move_cost(src, dst);
    if let Some(cm) = cur_min {
      if new_cost >= cm {
        continue;
      }
    }

    new_grid.insert(dst, amp);

    if let Some(new_min) = min_cost_prune(new_grid, new_cost, cur_min) {
      cur_min = Some(cur_min.map_or(new_min, |cm| cm.min(new_min)));
    }
  }

  cur_min
}

fn valid_moves4(grid: &Grid) -> Vec<(Pos, Pos)> {
  let mut ret = Vec::new();

  // check amps in the hallway
  for col in VALID_HALLWAY {
    let src = (0, col);
    let amp = *grid.get(&src).unwrap();
    if amp != Amp::Empty {
      let tc = amp.target_col();
      let (l, r) = if col < tc { (col + 1, tc) } else { (tc, col - 1) };
      // the hallway to target must be empty
      if (l..r).all(|c| *grid.get(&(0, c)).unwrap() == Amp::Empty) {
        for tr in [4, 3, 2, 1] {
          // below targets must be all filled with same amp, and above must be empty
          if (1..=tr).all(|r| *grid.get(&(r, tc)).unwrap() == Amp::Empty)
            && (tr + 1..=4).all(|r| *grid.get(&(r, tc)).unwrap() == amp)
          {
            ret.push((src, (tr, tc)));
            break;
          }
        }
      }
    }
  }

  // check amps in the burrows
  for row in [1, 2, 3, 4] {
    for col in [2, 4, 6, 8] {
      let src = (row, col);
      let amp = *grid.get(&src).unwrap();

      if amp != Amp::Empty
        // only move if it or something below is not in the final position
        && (row..=4).any(|r| grid.get(&(r, col)).unwrap().target_col() != col)
        // and the space above must be empty
        && (0..row).all(|r| *grid.get(&(r, col)).unwrap() == Amp::Empty)
      {
        let tc = amp.target_col();
        if (tc == col) && (*grid.get(&(2, col)).unwrap() == amp) {
          // this burrow is done
          continue;
        }

        // try move to target position first
        let mut skip_hallway = false;
        let (l, r) = if col < tc { (col + 1, tc) } else { (tc, col - 1) };
        if (l..=r).all(|c| *grid.get(&(0, c)).unwrap() == Amp::Empty) {
          for tr in [4, 3, 2, 1] {
            // below targets must be all filled with same amp, and above must be empty
            if (1..=tr).all(|r| *grid.get(&(r, tc)).unwrap() == Amp::Empty)
              && (tr + 1..=4).all(|r| *grid.get(&(r, tc)).unwrap() == amp)
            {
              ret.push((src, (tr, tc)));
              skip_hallway = true;
              break;
            }
          }
        }

        // if not, find valid positions in hallway
        if !skip_hallway {
          // right
          VALID_HALLWAY
            .clone()
            .into_iter()
            .skip_while(|&c| c < col)
            .take_while(|&c| *grid.get(&(0, c)).unwrap() == Amp::Empty)
            .for_each(|c| ret.push((src, (0, c))));

          // left
          VALID_HALLWAY
            .clone()
            .into_iter()
            .rev()
            .skip_while(|&c| c > col)
            .take_while(|&c| *grid.get(&(0, c)).unwrap() == Amp::Empty)
            .for_each(|c| ret.push((src, (0, c))));
        }
      }
    }
  }

  ret
}

fn min_cost4(grid: Grid) -> Option<u32> {
  min_cost_prune4(grid, 0, None)
}

fn min_cost_prune4(grid: Grid, cur_cost: u32, mut cur_min: Option<u32>) -> Option<u32> {
  if complete(&grid) {
    return Some(cur_cost);
  }

  for (src, dst) in valid_moves4(&grid) {
    let mut new_grid = grid.clone();
    let amp = new_grid.insert(src, Amp::Empty).unwrap();
    let new_cost = cur_cost + amp.move_cost(src, dst);
    if let Some(cm) = cur_min {
      if new_cost >= cm {
        continue;
      }
    }

    new_grid.insert(dst, amp);

    if let Some(new_min) = min_cost_prune4(new_grid, new_cost, cur_min) {
      cur_min = Some(cur_min.map_or(new_min, |cm| cm.min(new_min)));
    }
  }

  cur_min
}
