macro_rules! solutions {
  ( $year_mod:ident, $year:expr => [$( $day_mod:ident $day:expr, )+] ) => {
    mod $year_mod;

    pub fn run(y: u32, d: u8) {
      $(
        if y == $year && d == $day {
          $year_mod::$day_mod::solution();
        }
      )*
    }

  };
}

solutions!(year2019, 2019 => [
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
]);
