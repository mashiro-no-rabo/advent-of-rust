use std::fs;

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/9.txt").unwrap();

  let cols = content.trim().lines().next().unwrap().len();
  let rows = content.trim().lines().count();

  let mut height = Vec::with_capacity(rows * cols);
  content.trim().lines().for_each(|line| {
    line.chars().for_each(|c| {
      height.push(c.to_digit(10).unwrap());
    });
  });

  let cur = |r: usize, c: usize| height[r * cols + c];
  let up = |r: usize, c: usize| height[(r - 1) * cols + c];
  let down = |r: usize, c: usize| height[(r + 1) * cols + c];
  let left = |r: usize, c: usize| height[r * cols + c - 1];
  let right = |r: usize, c: usize| height[r * cols + c + 1];

  let mut p1 = 0;
  let mut basin_starts = Vec::new();

  for r in 0..rows {
    for c in 0..cols {
      let mut lowest = true;
      if r > 0 {
        lowest = up(r, c) > cur(r, c);
      }
      if lowest && (r < rows - 1) {
        lowest = down(r, c) > cur(r, c);
      }
      if lowest && (c > 0) {
        lowest = left(r, c) > cur(r, c);
      }
      if lowest && (c < cols - 1) {
        lowest = right(r, c) > cur(r, c);
      }
      if lowest {
        p1 += 1 + cur(r, c);
        basin_starts.push((r, c));
      }
    }
  }

  println!("sum: {}", p1);

  let mut basin_sizes = basin_starts
    .into_iter()
    .map(|(r, c)| {
      let mut rest = vec![(r, c)];
      let mut basin = Vec::new();
      while let Some((r, c)) = rest.pop() {
        if r > 0 && up(r, c) != 9 && !basin.contains(&(r - 1, c)) && !rest.contains(&(r - 1, c)) {
          rest.push((r - 1, c));
        }
        if (r < rows - 1) && down(r, c) != 9 && !basin.contains(&(r + 1, c)) && !rest.contains(&(r + 1, c)) {
          rest.push((r + 1, c));
        }
        if (c > 0) && left(r, c) != 9 && !basin.contains(&(r, c - 1)) && !rest.contains(&(r, c - 1)) {
          rest.push((r, c - 1));
        }
        if (c < cols - 1) && right(r, c) != 9 && !basin.contains(&(r, c + 1)) && !rest.contains(&(r, c + 1)) {
          rest.push((r, c + 1));
        }

        basin.push((r, c));
      }

      basin.len() as u32
    })
    .collect::<Vec<_>>();
  basin_sizes.sort();
  println!("multiply: {}", basin_sizes.iter().rev().take(3).product::<u32>());
}
