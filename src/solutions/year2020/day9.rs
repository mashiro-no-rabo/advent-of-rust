use std::collections::VecDeque;
use std::fs;

pub fn solution() {
  let content = fs::read_to_string("inputs/2020/9.txt").unwrap();
  let mut numbers = content.lines().map(|x| x.parse::<i64>().unwrap());

  let mut validators = VecDeque::with_capacity(25);
  let mut part1 = numbers.clone();
  let first_invalid = loop {
    let n = part1.next().unwrap();
    if validators.len() < 25 {
      validators.push_back(n);
    } else {
      if xmas_validate(&validators, n) {
        validators.pop_front();
        validators.push_back(n);
      } else {
        break n;
      }
    }
  };

  let mut sets = VecDeque::new();
  let weakness = loop {
    let n = numbers.next().unwrap();

    sets.push_back(n);
    while sets.iter().sum::<i64>() > first_invalid && sets.len() > 2 {
      sets.pop_front();
    }
    if sets.iter().sum::<i64>() == first_invalid && sets.len() >= 2 {
      break sets.iter().min().unwrap() + sets.iter().max().unwrap();
    }
  };

  println!("First invalid number: {}", first_invalid);
  println!("Encryption weakness: {}", weakness);
}

fn xmas_validate(validators: &VecDeque<i64>, num: i64) -> bool {
  validators.iter().enumerate().any(|(idx, val)| {
    let other = num - val;
    validators.iter().position(|x| *x == other).map_or(false, |p| p != idx)
      || validators.iter().rposition(|x| *x == other).map_or(false, |p| p != idx)
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::VecDeque;

  #[test]
  fn test1() {
    let mut validators = VecDeque::new();
    validators.push_back(1);
    validators.push_back(2);
    validators.push_back(3);
    validators.push_back(4);
    validators.push_back(5);

    assert_eq!(true, xmas_validate(&validators, 9));
    assert_eq!(false, xmas_validate(&validators, 10));
  }
}
