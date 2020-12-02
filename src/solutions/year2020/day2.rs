use std::fs;

pub fn solution() {
  let content = fs::read_to_string("inputs/2020/2.txt").unwrap();
  let valid = content.lines().filter(|l| verify(l)).count();
  let valid2 = content.lines().filter(|l| verify2(l)).count();

  println!("Valid passwords: {}", valid);
  println!("Valid passwords2: {}", valid2);
}

fn verify(line: &str) -> bool {
  let mut parts = line.split(':');
  let policy = parts.next().unwrap();
  let pwd = parts.next().unwrap();

  let mut parts = policy.split(' ');
  let count = parts.next().unwrap();
  let letter = parts.next().unwrap().chars().last().unwrap();

  let mut parts = count.split('-');
  let min = parts.next().unwrap().parse::<usize>().unwrap();
  let max = parts.next().unwrap().parse::<usize>().unwrap();

  let occurs = pwd.chars().filter(|c| c == &letter).count();

  (occurs >= min) & (occurs <= max)
}

fn verify2(line: &str) -> bool {
  let mut parts = line.split(':');
  let policy = parts.next().unwrap();
  let pwd = parts.next().unwrap().trim();

  let mut parts = policy.split(' ');
  let count = parts.next().unwrap();
  let letter = parts.next().unwrap().chars().last().unwrap();

  let mut parts = count.split('-');
  let pos1 = parts.next().unwrap().parse::<usize>().unwrap();
  let pos2 = parts.next().unwrap().parse::<usize>().unwrap();

  let occur1 = pwd.chars().skip(pos1 - 1).next().unwrap() == letter;
  let occur2 = pwd.chars().skip(pos2 - 1).next().unwrap() == letter;

  occur1 ^ occur2
}

mod tests {
  use super::*;

  #[test]
  fn test1() {
    assert_eq!(true, verify("1-3 a: abcde"));
  }

  #[test]
  fn test2() {
    assert_eq!(true, verify2("1-3 a: abcde"));
    assert_eq!(true, verify2("1-3 a: cbade"));
    assert_eq!(false, verify2("1-3 b: cdefg"));
    assert_eq!(false, verify2("2-9 c: ccccccccc"));
  }
}
