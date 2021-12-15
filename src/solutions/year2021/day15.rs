use std::{collections::HashMap, fs};

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/15.txt").unwrap();

  let rows = content.lines().count();
  let cols = content.lines().next().unwrap().len();
  let rc_to_idx = |rc: (usize, usize)| -> usize { rc.0 * rows + rc.1 };
  let neighbors = |rc: (usize, usize)| -> Vec<(usize, usize)> {
    let mut n = vec![];
    if rc.0 > 0 {
      n.push((rc.0 - 1, rc.1));
    }
    if rc.0 < rows - 1 {
      n.push((rc.0 + 1, rc.1));
    }
    if rc.1 > 0 {
      n.push((rc.0, rc.1 - 1));
    }
    if rc.1 < cols - 1 {
      n.push((rc.0, rc.1 + 1));
    }
    n
  };
  let end = (rows - 1, cols - 1);

  let risk = content.lines().fold(Vec::new(), |mut acc, line| {
    line.chars().for_each(|c| {
      acc.push(c.to_digit(10).unwrap());
    });

    acc
  });

  let mut path_risk = HashMap::new();
  path_risk.insert((0, 0), 0 as u32);

  let mut visited = vec![false; risk.len()];

  loop {
    let (&vp, &vr) = path_risk
      .iter()
      .filter(|&(&pos, _)| !visited[rc_to_idx(pos)])
      .min_by_key(|&(_, risk)| risk)
      .unwrap();

    for (nr, nc) in neighbors(vp) {
      if !visited[rc_to_idx((nr, nc))] {
        path_risk
          .entry((nr, nc))
          .and_modify(|n_risk| {
            if *n_risk > vr + risk[rc_to_idx((nr, nc))] {
              *n_risk = vr + risk[rc_to_idx((nr, nc))];
            }
          })
          .or_insert(vr + risk[rc_to_idx((nr, nc))]);
      }
    }

    visited[rc_to_idx(vp)] = true;

    if visited[rc_to_idx(end)] {
      break;
    }
  }

  println!("min risk: {}", path_risk.get(&(end)).unwrap());
}
