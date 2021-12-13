use std::{collections::HashSet, fs};

enum FoldPaper {
  Up(u32),
  Left(u32),
}

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/13.txt").unwrap();

  let mut lines = content.trim().lines();
  let dots = {
    let mut dots = HashSet::new();

    while let Some(line) = lines.next() {
      if line == "" {
        break;
      }
      let mut parts = line.split(",");
      let x = parts.next().unwrap().parse::<u32>().unwrap();
      let y = parts.next().unwrap().parse::<u32>().unwrap();
      dots.insert((x, y));
    }
    dots
  };

  let fps = lines.map(|line| {
    let s = line.split_ascii_whitespace().skip(2).next().unwrap();
    let (i, d) = s.split_at(2);
    let d = d.parse::<u32>().unwrap();
    match i {
      "y=" => FoldPaper::Up(d),
      "x=" => FoldPaper::Left(d),
      _ => unimplemented!(),
    }
  });

  let p1 = fps.clone().take(1).fold(dots.clone(), |mut acc, fp| {
    let mut new_acc = HashSet::new();
    for dot in acc.drain() {
      match fp {
        FoldPaper::Up(ys) => {
          if dot.1 > ys {
            new_acc.insert((dot.0, 2 * ys - dot.1));
          } else {
            new_acc.insert(dot);
          }
        }
        FoldPaper::Left(xs) => {
          if dot.0 > xs {
            new_acc.insert((2 * xs - dot.0, dot.1));
          } else {
            new_acc.insert(dot);
          }
        }
      }
    }
    new_acc
  });

  println!("fold once: {}", p1.len());

  let p2 = fps.fold(dots.clone(), |mut acc, fp| {
    let mut new_acc = HashSet::new();
    for dot in acc.drain() {
      match fp {
        FoldPaper::Up(ys) => {
          if dot.1 > ys {
            new_acc.insert((dot.0, 2 * ys - dot.1));
          } else {
            new_acc.insert(dot);
          }
        }
        FoldPaper::Left(xs) => {
          if dot.0 > xs {
            new_acc.insert((2 * xs - dot.0, dot.1));
          } else {
            new_acc.insert(dot);
          }
        }
      }
    }
    new_acc
  });

  let my = *p2.iter().map(|(_, y)| y).max().unwrap();
  let mx = *p2.iter().map(|(x, _)| x).max().unwrap();

  for y in 0..=my {
    for x in 0..=mx {
      if p2.contains(&(x, y)) {
        print!("#");
      } else {
        print!(" ");
      }
    }
    println!();
  }
}
