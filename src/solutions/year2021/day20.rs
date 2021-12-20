use std::{collections::HashMap, fs};

// cheat
const SIZE: i32 = 100;

type Grid = HashMap<(i32, i32), bool>;

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/20.txt").unwrap();

  let mut lines = content.lines();
  let enhance = lines.next().unwrap().chars().map(|c| c == '#').collect::<Vec<_>>();

  let mut grid = HashMap::new();
  lines.skip(1).enumerate().for_each(|(y, line)| {
    line.char_indices().for_each(|(x, c)| {
      grid.insert((x as i32, y as i32), c == '#');
    });
  });

  for i in -50..150 {
    for j in -50..150 {
      if i < 0 || j < 0 || i >= SIZE || j >= SIZE {
        grid.insert((i, j), false);
      }
    }
  }

  let mut step = 0;
  while step < 50 {
    step += 1;
    let new_grid = grid
      .keys()
      .map(|pos| (*pos, enhance[map_pixel(&grid, *pos, step)]))
      .collect::<Grid>();

    grid = new_grid;

    if step == 2 {
      println!("2x lit: {}", grid.values().filter(|&&b| b).count());
    }
  }

  println!("50x lit: {}", grid.values().filter(|&&b| b).count());
}

const ORDER: [(i32, i32); 9] = [
  (1, 1),
  (0, 1),
  (-1, 1),
  (1, 0),
  (0, 0),
  (-1, 0),
  (1, -1),
  (0, -1),
  (-1, -1),
];

fn map_pixel(grid: &Grid, (x, y): (i32, i32), step: usize) -> usize {
  // cheat, infinite grid goes false (step 1) -> true -> false
  let infi = (step + 1) % 2;

  let mut ret = 0;
  let mut m = 1;
  for (dx, dy) in ORDER.iter() {
    let p = (x + dx, y + dy);
    let d = grid.get(&p).map_or(infi, |&l| if l { 1 } else { 0 });

    ret += m * d;
    m *= 2;
  }

  ret
}
