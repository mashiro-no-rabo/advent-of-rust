use super::int_code::State;

fn run_test_mode(mem: &[i64]) -> i64 {
  State::new_with_mem(mem).run(vec![1]).get_first_output()
}

fn run_boost(mem: &[i64]) -> i64 {
  State::new_with_mem(mem).run(vec![2]).get_first_output()
}

pub fn solution() {
  let input = std::fs::read_to_string("inputs/2019/9.txt").unwrap();
  let mem: Vec<i64> = input.split(',').map(|x| x.trim().parse::<i64>().unwrap()).collect();

  println!("BOOST keycode: {}", run_test_mode(&mem));
  println!("Distress signal: {}", run_boost(&mem));
}
