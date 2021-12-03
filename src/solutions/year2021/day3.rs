use std::fs;

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/3.txt").unwrap();

  let lines = content.trim().lines();

  let line_count = lines.clone().count();
  let len = lines.clone().next().unwrap().len();

  let counter = lines.clone().fold(vec![0; len], |mut acc, line| {
    for (i, c) in line.char_indices() {
      if c == '1' {
        acc[i] += 1;
      }
    }
    acc
  });

  let mut gamma_rate = Vec::new();
  let mut epsilon_rate = Vec::new();
  counter.iter().for_each(|c| {
    if *c > line_count / 2 {
      gamma_rate.push(1);
      epsilon_rate.push(0);
    } else {
      gamma_rate.push(0);
      epsilon_rate.push(1);
    }
  });

  println!(
    "power consumption: {}",
    bin_to_decimal(&gamma_rate) * bin_to_decimal(&epsilon_rate)
  );

  let oxygen = {
    let mut pos = 0;
    let mut remain: Vec<_> = lines.clone().collect();
    loop {
      let ones = remain.iter().fold(0, |acc, l| {
        if l.chars().nth(pos).unwrap() == '1' {
          acc + 1
        } else {
          acc
        }
      });

      let prefer = if ones >= (remain.len() - ones) { '1' } else { '0' };

      remain = remain
        .into_iter()
        .filter(|l| l.chars().nth(pos).unwrap() == prefer)
        .collect();

      if remain.len() == 1 {
        break;
      }

      pos += 1;
    }

    remain[0].to_string()
  };

  let co2 = {
    let mut pos = 0;
    let mut remain: Vec<_> = lines.clone().collect();
    loop {
      let zeros = remain.iter().fold(0, |acc, l| {
        if l.chars().nth(pos).unwrap() == '0' {
          acc + 1
        } else {
          acc
        }
      });

      let prefer = if zeros <= (remain.len() - zeros) { '0' } else { '1' };

      remain = remain
        .into_iter()
        .filter(|l| l.chars().nth(pos).unwrap() == prefer)
        .collect();

      if remain.len() == 1 {
        break;
      }

      pos += 1;
    }

    remain[0].to_string()
  };

  println!(
    "life support: {}",
    bin_str_to_decimal(&oxygen) * bin_str_to_decimal(&co2)
  );
}

fn bin_to_decimal(binary: &Vec<u8>) -> u32 {
  let (_, result) = binary.iter().rfold((1, 0), |mut acc, &b| {
    acc.1 += b as u32 * acc.0;
    acc.0 *= 2;
    acc
  });

  result
}

fn bin_str_to_decimal(binary: &str) -> u32 {
  let (_, result) = binary.bytes().rfold((1, 0), |mut acc, c| {
    acc.1 += (c - b'0') as u32 * acc.0;
    acc.0 *= 2;
    acc
  });

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn bin_test() {
    let a = vec![1, 0, 1, 1, 0];
    assert_eq!(22, bin_to_decimal(&a));
  }

  #[test]
  fn bin_str_test() {
    assert_eq!(22, bin_str_to_decimal("10110"));
  }
}
