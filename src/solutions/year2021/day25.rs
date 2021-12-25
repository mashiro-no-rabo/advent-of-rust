use std::fs;

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/25.txt").unwrap();

  let rows = content.lines().count();
  let cols = content.lines().next().unwrap().len();

  let mut grid = content.lines().flat_map(str::chars).collect::<Vec<_>>();

  let mut steps = 0;
  loop {
    steps += 1;
    let mut moved = false;

    let mut ng = grid.clone();
    for r in 0..rows {
      for c in 0..cols {
        if grid[r * cols + c] == '>' && grid[r * cols + (c + 1) % cols] == '.' {
          ng[r * cols + c] = '.';
          ng[r * cols + (c + 1) % cols] = '>';
          moved = true;
        }
      }
    }

    let mut new_grid = ng.clone();
    for r in 0..rows {
      for c in 0..cols {
        if ng[r * cols + c] == 'v' && ng[((r + 1) % rows) * cols + c] == '.' {
          new_grid[r * cols + c] = '.';
          new_grid[((r + 1) % rows) * cols + c] = 'v';
          moved = true;
        }
      }
    }

    if moved {
      grid = new_grid;
    } else {
      break;
    }
  }

  println!("steps: {}", steps);
}
