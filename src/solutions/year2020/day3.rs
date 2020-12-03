use std::fs;

pub fn solution() {
  let map = fs::read_to_string("inputs/2020/3.txt").unwrap();
  let map = map.trim();

  println!("Anwser 1: {}", rational_slope(map));
  println!("Anwser 2: {}", slopes_multiply(map));
}

fn rational_slope(map: &str) -> u32 {
  map
    .lines()
    .fold((0, 0), |(next_x, count), line| {
      let chars: Vec<char> = line.trim().chars().collect();
      let new_x = (next_x + 3) % chars.len();
      if chars.get(next_x).unwrap() == &'#' {
        (new_x, count + 1)
      } else {
        (new_x, count)
      }
    })
    .1
}

fn slopes_multiply(map: &str) -> u64 {
  let travels = map
    .lines()
    .enumerate()
    .fold((0, 0, 0, 0, 0), |(t11, t31, t51, t71, t12), (idx, line)| {
      let chars: Vec<char> = line.trim().chars().collect();
      let len = chars.len();

      let new_t12 = {
        if idx % 2 == 0 {
          new_count(t12, chars.get((idx / 2) % len).unwrap())
        } else {
          t12
        }
      };

      (
        new_count(t11, chars.get(idx % len).unwrap()),
        new_count(t31, chars.get(idx * 3 % len).unwrap()),
        new_count(t51, chars.get(idx * 5 % len).unwrap()),
        new_count(t71, chars.get(idx * 7 % len).unwrap()),
        new_t12,
      )
    });

  travels.0 * travels.1 * travels.2 * travels.3 * travels.4
}

fn new_count(count: u64, ch: &char) -> u64 {
  if ch == &'#' {
    count + 1
  } else {
    count
  }
}

mod tests {
  use super::*;

  #[test]
  fn test1() {
    let map = "
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
      .trim();

    assert_eq!(7, rational_slope(map));
  }

  #[test]
  fn test2() {
    let map = "
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
      .trim();

    assert_eq!(336, slopes_multiply(map));
  }
}
