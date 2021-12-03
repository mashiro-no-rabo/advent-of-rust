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
]);
