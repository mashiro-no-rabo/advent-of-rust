fn fft(signal_str: &str, phases: usize) -> Vec<i64> {
  let mut signal: Vec<i64> = signal_str.bytes().map(|x| (x - b'0') as i64).collect();
  let siglen = signal_str.len();
  for _ in 0..phases {
    let mut new_signal = vec![];
    for pos in 1..=siglen {
      let sum = signal
        .iter()
        .zip(build_pattern(pos))
        .fold(0, |acc, (x, y)| acc + (*x * y));

      new_signal.push(sum.abs() % 10);
    }

    signal = new_signal;
  }
  signal
}

fn build_pattern(pos: usize) -> impl Iterator<Item = i64> {
  use std::iter::repeat;
  repeat(0)
    .take(pos)
    .chain(repeat(1).take(pos))
    .chain(repeat(0).take(pos))
    .chain(repeat(-1).take(pos))
    .cycle()
    .skip(1)
}

fn signal_to_str(signal: &[i64], len: usize) -> String {
  String::from_utf8(signal.iter().take(len).map(|x| *x as u8 + b'0').collect::<Vec<u8>>()).unwrap()
}

pub fn solution() {
  let input = std::fs::read_to_string("inputs/2019/16.txt").unwrap();

  println!("First eight: {}", signal_to_str(&fft(input.trim(), 100), 8));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_pattern1() {
    assert_eq!(vec![1, 0, -1, 0], build_pattern(1).take(4).collect::<Vec<i64>>());
  }

  #[test]
  fn test_fft() {
    let expected = parse_expected("48226158");
    assert_eq!(expected[..8], fft("12345678", 1)[..8]);
  }

  #[test]
  fn test_fft_100() {
    let expected = parse_expected("24176176");
    assert_eq!(expected[..8], fft("80871224585914546619083218645595", 100)[..8]);
  }

  fn parse_expected(input: &str) -> Vec<i64> {
    input.bytes().map(|x| (x - b'0') as i64).collect()
  }
}
