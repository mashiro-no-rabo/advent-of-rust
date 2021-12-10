use std::fs;

enum Part {
  One(u32),
  Two(Vec<char>),
}
use Part::*;

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/10.txt").unwrap();

  let results = content.trim().lines().map(|line| {
    let fr = line.chars().try_fold(Vec::new(), |mut q, c| match c {
      ')' | ']' | '}' | '>' => {
        let open = q.pop().unwrap();
        if close(open) == c {
          Ok(q)
        } else {
          Err(c)
        }
      }
      '(' | '[' | '{' | '<' => {
        q.push(c);
        Ok(q)
      }
      _ => unimplemented!(),
    });

    match fr {
      Ok(v) => Two(v.into_iter().rev().map(close).collect()),
      Err(corrupt) => match corrupt {
        ')' => One(3),
        ']' => One(57),
        '}' => One(1197),
        '>' => One(25137),
        _ => unimplemented!(),
      },
    }
  });

  let p1 = results
    .clone()
    .filter_map(|r| match r {
      One(n) => Some(n),
      _ => None,
    })
    .sum::<u32>();

  println!("syntax error score: {}", p1);

  let mut p2: Vec<u64> = results
    .filter_map(|r| match r {
      Two(v) => Some(v.into_iter().fold(0, |mut acc, c| {
        acc *= 5;
        acc += acs(c);
        acc
      })),
      _ => None,
    })
    .collect();
  let skip = p2.len() / 2;
  p2.sort();
  println!("auto complete score: {}", p2.into_iter().skip(skip).next().unwrap());
}

fn close(open: char) -> char {
  match open {
    '(' => ')',
    '[' => ']',
    '{' => '}',
    '<' => '>',
    _ => unimplemented!(),
  }
}

fn acs(c: char) -> u64 {
  match c {
    ')' => 1,
    ']' => 2,
    '}' => 3,
    '>' => 4,
    _ => unimplemented!(),
  }
}
