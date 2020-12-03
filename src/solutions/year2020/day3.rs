use std::fs;

pub fn solution() {
  let map = fs::read_to_string("inputs/2020/3.txt").unwrap();

  println!("Anwser 1: {}", rational_slope(map.trim()));
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
}
