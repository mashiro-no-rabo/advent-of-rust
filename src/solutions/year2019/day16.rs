fn fft(signal_str: &str, phases: usize) -> Vec<i64> {
  let mut signal: Vec<i64> = signal_str.trim().bytes().map(|x| (x - b'0') as i64).collect();
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

// I though the offset need to take after 100 phases,
// turns out it's just the input, and I have noticed
// later part have some trick very early in the morning
// FML!
fn fml_fft(signal_str: &str, phases: usize) -> String {
  let signal: Vec<u64> = signal_str.trim().bytes().map(|x| (x - b'0') as u64).collect();

  // grab the offset from input
  let offset = {
    let mut out = 0;
    let mut mag = 1;
    for idx in 1..=7 {
      out += signal[7 - idx] * mag;
      mag *= 10;
    }
    out
  };

  // it should fall in the latter half
  if (offset as usize) < (signal.len() * 5000) {
    panic!("FML!");
  }

  // how much to skip within a signal cycle
  let idx_offset = offset as usize % signal.len();
  let pattern_len = (signal.len() * 10000) - (offset as usize);
  let mut pattern = vec![1; pattern_len];
  for _ in 0..phases - 1 {
    // pattern[idx, phase] = pattern[idx-1, phase] + pattern[idx, phase-1]
    for idx in 1..pattern_len {
      pattern[idx] += pattern[idx - 1];
      pattern[idx] %= 10;
    }
  }

  {
    let mut bytes = vec![];
    for idx in 0..8 {
      let sig = signal
        .iter()
        .cycle()
        .skip(idx_offset)
        .skip(idx)
        .take(pattern_len - idx)
        .zip(pattern.iter())
        .fold(0, |acc, (s, p)| acc + (s * p))
        % 10;
      bytes.push(sig as u8 + b'0');
    }

    String::from_utf8(bytes).unwrap()
  }
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

  println!("First eight: {}", signal_to_str(&fft(&input, 100), 8));
  println!("FML eight: {}", fml_fft(&input, 100));
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

  #[test]
  fn test_fml_fft() {
    assert_eq!("84462026", &fml_fft("03036732577212944063491565474664", 100));
  }

  #[test]
  fn test_fml_fft2() {
    assert_eq!("78725270", &fml_fft("02935109699940807407585447034323", 100));
  }

  #[test]
  fn test_fml_fft3() {
    assert_eq!("53553731", &fml_fft("03081770884921959731165446850517", 100));
  }

  fn parse_expected(input: &str) -> Vec<i64> {
    input.bytes().map(|x| (x - b'0') as i64).collect()
  }
}
