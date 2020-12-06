use std::collections::HashSet;
use std::fs;

pub fn solution() {
  let content = fs::read_to_string("inputs/2020/6.txt").unwrap();

  let (counts_any, _) =
    content
      .lines()
      .chain(vec![""].into_iter())
      .fold((Vec::new(), HashSet::new()), |(mut qc, mut cur), line| {
        if line.is_empty() {
          qc.push(cur.len());
          (qc, HashSet::new())
        } else {
          line.chars().for_each(|ch| {
            cur.insert(ch);
          });

          (qc, cur)
        }
      });

  let (counts_all, _) = content
    .lines()
    .chain(vec![""].into_iter())
    .fold((Vec::new(), None), |(mut qc, cur), line| {
      if line.is_empty() {
        qc.push((cur.unwrap() as HashSet<char>).len());
        (qc, None)
      } else {
        let new_cur: HashSet<char> = match cur {
          Some(cur) => {
            let s: HashSet<char> = line.chars().collect();
            (cur as HashSet<char>).intersection(&s).cloned().collect()
          }
          None => line.chars().collect(),
        };

        (qc, Some(new_cur))
      }
    });

  println!("Sum any: {}", counts_any.iter().sum::<usize>());
  println!("Sum all: {}", counts_all.iter().sum::<usize>());
}
