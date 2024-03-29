macro_rules! solutions {
  ( [$( $day_mod:ident $day:expr, )+] ) => {
    pub fn run(d: u8) {
      $(
        if d == $day {
          $day_mod::solution();
        }
      )*
    }
  };
}

mod year2021;
use year2021::*;

solutions!([
  day1 1,
  day2 2,
  day3 3,
  day4 4,
  day5 5,
  day6 6,
  day7 7,
  day8 8,
  day9 9,
  day10 10,
  day11 11,
  day12 12,
  day13 13,
  day14 14,
  day15 15,
  day16 16,
  day17 17,
  day18 18,
  day19 19,
  day20 20,
  day21 21,
  day22 22,
  day23 23,
  day24 24,
  day25 25,
]);
