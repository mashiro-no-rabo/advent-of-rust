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
]);
