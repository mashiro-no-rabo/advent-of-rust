use std::fs;

fn part1(input: &str) -> i64 {
  let pixels = input.bytes();

  let mut pos = 0; // position in a layer
  let mut zeros = 0; // num of 0 in current layer
  let mut ones = 0; // num of 1 ...
  let mut twos = 0; // num of 2 ...

  let mut min_zero = None; // overall fewest 0 digits
  let mut result = 0;

  for pixel in pixels {
    match pixel {
      b'0' => zeros += 1,
      b'1' => ones += 1,
      b'2' => twos += 1,
      _ => {}
    }

    pos += 1;
    if pos == 25 * 6 {
      match min_zero {
        None => {
          min_zero = Some(zeros);
          result = ones * twos;
        }
        Some(z) => {
          if zeros < z {
            min_zero = Some(zeros);
            result = ones * twos;
          }
        }
      }

      pos = 0;
      zeros = 0;
      ones = 0;
      twos = 0;
    }
  }

  result
}

fn part2(input: &str) {
  let pixels = input.bytes();

  let mut image = vec![2; 25 * 6];

  let mut pos = 0; // position in a layer
  for pixel in pixels {
    if image[pos] == 2 {
      image[pos] = pixel - b'0';
    }

    pos += 1;
    if pos == 25 * 6 {
      pos = 0;
    }
  }

  for row in 0..6 {
    for col in 0..25 {
      if image[row * 25 + col] > 0 {
        print!("{}", image[row * 25 + col]);
      } else {
        print!(" ");
      }
    }
    println!();
  }
}

fn main() {
  let input = fs::read_to_string("inputs/2019/8.txt").unwrap();

  println!("Solution 1: {}", part1(input.trim()));
  part2(input.trim());
}
