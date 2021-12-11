use itertools::Itertools;
use std::fs;

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/11.txt").unwrap();

  let cols = content.trim().lines().next().unwrap().len();
  let rows = content.trim().lines().count();

  let mut octos = Vec::new();
  content.trim().lines().for_each(|line| {
    line.chars().for_each(|c| {
      octos.push(c.to_digit(10).unwrap());
    });
  });

  let mut flashes = 0;
  let mut step = 0;
  loop {
    step += 1;
    // increase
    octos.iter_mut().for_each(|o| {
      *o += 1;
    });

    // flash
    let mut flashing = Vec::new();
    for r in 0..rows {
      for c in 0..cols {
        if octos[r * cols + c] > 9 {
          flashing.push((r, c));
        }
      }
    }

    let mut flashed = Vec::new();
    while let Some((r, c)) = flashing.pop() {
      flashed.push((r, c));

      [-1, 0, 1]
        .iter()
        .cartesian_product([-1, 0, 1].iter())
        .filter_map(|(&dr, &dc)| {
          if dr == 0 && dc == 0 {
            None
          } else {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
              Some((nr as usize, nc as usize))
            } else {
              None
            }
          }
        })
        .for_each(|(nr, nc)| {
          octos[nr * cols + nc] += 1;
          if octos[nr * cols + nc] > 9 && !flashed.contains(&(nr, nc)) && !flashing.contains(&(nr, nc)) {
            flashing.push((nr, nc));
          }
        });
    }
    if step <= 100 {
      flashes += flashed.len();
    }

    if flashed.len() == rows * cols {
      break;
    }

    // reset
    octos.iter_mut().for_each(|o| {
      if *o > 9 {
        *o = 0;
      }
    });
  }

  println!("after 100 steps: {}", flashes);
  println!("sync steps: {}", step);
}
