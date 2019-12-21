use super::int_code::State;

fn included(beam: &State, x: i64, y: i64) -> bool {
  beam.run(vec![x, y]).get_first_output() == 1
}

fn count_50(state: &State) -> usize {
  let mut count = 0;
  for x in 0..50 {
    for y in 0..50 {
      if included(state, x, y) {
        count += 1;
      }
    }
  }

  count
}

fn find_beam(beam: &State, y: i64) -> (i64, i64) {
  let mut x_start = 0;
  let mut x_end = 0;

  (x_start, x_end)
}

// test from bottom left (x, y)
fn santa_fits(state: &State, bl_x: i64, bl_y: i64) -> bool {
  [
    (bl_x, bl_y),
    (bl_x, bl_y - 99),
    (bl_x + 99, bl_y - 99),
    (bl_x + 99, bl_y),
  ]
  .iter()
  .all(|(x, y)| state.run(vec![*x, *y]).get_first_output() == 1)
}

fn best_fit(state: &State) -> u64 {
  let mut bl_x = 100;
  let mut bl_y = 100;
  while !santa_fits(state, bl_x, bl_y) {
    bl_x += 100;
    bl_y += 100;
  }

  (bl_x as u64) * 10_000 + (bl_y as u64)
}

pub fn solution() {
  let state = State::from_file("inputs/2019/19.txt");

  println!("Points in 50x50 area: {}", count_50(&state));
  println!("Best fit: {}", best_fit(&state));
}
