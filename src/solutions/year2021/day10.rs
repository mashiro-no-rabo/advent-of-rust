use std::fs;

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/10.txt").unwrap();

  let p1 = content
    .trim()
    .lines()
    .filter_map(|line| {
      if let Err(corrupt) = line.chars().try_fold(Vec::new(), |mut q, c| match c {
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
      }) {
        match corrupt {
          ')' => Some(3),
          ']' => Some(57),
          '}' => Some(1197),
          '>' => Some(25137),
          _ => unimplemented!(),
        }
      } else {
        None
      }
    })
    .sum::<u32>();

  println!("score: {}", p1);
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
