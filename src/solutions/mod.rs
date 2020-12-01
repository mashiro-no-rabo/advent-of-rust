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

solutions!(year2020, 2020 => [
  day1 1,
]);
