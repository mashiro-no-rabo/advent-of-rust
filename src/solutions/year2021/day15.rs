use priority_queue::PriorityQueue;
use std::{collections::HashMap, fs};

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/15.txt").unwrap();

  let rows = content.lines().count();
  let cols = content.lines().next().unwrap().len();
  let rc_to_idx = |rc: (usize, usize)| -> usize { rc.0 * cols + rc.1 };

  let risk = content.lines().fold(Vec::new(), |mut acc, line| {
    line.chars().for_each(|c| {
      acc.push(c.to_digit(10).unwrap());
    });

    acc
  });

  {
    // p1

    let mut path_risk = HashMap::new();
    path_risk.insert((0, 0), 0 as u32);

    let mut pq = PriorityQueue::new();
    pq.push((0, 0), 0);

    let mut visited = vec![false; risk.len()];
    let end = (rows - 1, cols - 1);

    loop {
      let (vp, vr) = pq.pop().unwrap();
      let vr = -vr as u32;

      for pos in neighbors(vp, rows, cols) {
        if !visited[rc_to_idx(pos)] {
          path_risk
            .entry(pos)
            .and_modify(|n_risk| {
              if *n_risk > vr + risk[rc_to_idx(pos)] {
                *n_risk = vr + risk[rc_to_idx(pos)];
              }
            })
            .or_insert(vr + risk[rc_to_idx(pos)]);

          if pq.change_priority(&pos, -(path_risk[&pos] as i32)).is_none() {
            pq.push(pos, -(path_risk[&pos] as i32));
          }
        }
      }

      visited[rc_to_idx(vp)] = true;

      if visited[rc_to_idx(end)] {
        break;
      }
    }

    println!("min risk: {}", path_risk.get(&end).unwrap());
  }

  {
    // p2

    let risk5 = {
      let mut r5 = vec![0; risk.len() * 5 * 5];
      for or in 0..rows {
        for oc in 0..cols {
          for dr in 0..5 {
            for dc in 0..5 {
              let new_idx = (dr * rows + or) * 5 * cols + dc * cols + oc;
              let mut new_risk = risk[rc_to_idx((or, oc))] + dr as u32 + dc as u32;
              if new_risk > 9 {
                new_risk -= 9;
              }
              r5[new_idx] = new_risk;
            }
          }
        }
      }

      r5
    };

    let risk = risk5;
    let rows = rows * 5;
    let cols = cols * 5;
    let rc_to_idx = |rc: (usize, usize)| -> usize { rc.0 * cols + rc.1 };

    let mut path_risk = HashMap::new();
    path_risk.insert((0, 0), 0 as u32);

    let mut pq = PriorityQueue::new();
    pq.push((0, 0), 0);

    let mut visited = vec![false; risk.len()];
    let end = (rows - 1, cols - 1);

    loop {
      let (vp, vr) = pq.pop().unwrap();
      let vr = -vr as u32;

      for pos in neighbors(vp, rows, cols) {
        let idx = rc_to_idx(pos);
        if !visited[idx] {
          path_risk
            .entry(pos)
            .and_modify(|n_risk| {
              if *n_risk > vr + risk[idx] {
                *n_risk = vr + risk[idx];
              }
            })
            .or_insert(vr + risk[idx]);

          if pq.change_priority(&pos, -(path_risk[&pos] as i32)).is_none() {
            pq.push(pos, -(path_risk[&pos] as i32));
          }
        }
      }

      visited[rc_to_idx(vp)] = true;

      if visited[rc_to_idx(end)] {
        break;
      }
    }

    println!("min risk 5x: {}", path_risk.get(&end).unwrap());
  }
}

fn neighbors(rc: (usize, usize), rows: usize, cols: usize) -> Vec<(usize, usize)> {
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
}
