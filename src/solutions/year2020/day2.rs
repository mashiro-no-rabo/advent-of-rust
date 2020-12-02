use std::fs;

pub fn solution() {
  let content = fs::read_to_string("inputs/2020/2.txt").unwrap();
  let valid = content.lines().filter(|l| verify(l)).count();

  println!("Valid passwords: {}", valid);
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

  occurs >= min && occurs <= max
}

mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(true, verify("1-3 a: abcde"));
  }
}
