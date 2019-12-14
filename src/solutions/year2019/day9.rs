use async_std::fs::File;
use async_std::io::BufReader;
use async_std::prelude::*;
use async_std::task;
use std::str;

use super::int_code::State;

fn run_test_mode(mem: &[i64]) -> i64 {
  State::new_with_mem(mem).run(vec![1]).get_first_output()
}

fn run_boost(mem: &[i64]) -> i64 {
  State::new_with_mem(mem).run(vec![2]).get_first_output()
}

pub fn solution() {
  task::block_on(async {
    let file = File::open("inputs/2019/9.txt").await.unwrap();
    let mem: Vec<i64> = BufReader::new(file)
      .split(b',')
      .filter_map(|x| x.ok())
      .filter_map(|x| str::from_utf8(&x).unwrap().trim().parse::<i64>().ok())
      .collect()
      .await;

    println!("BOOST keycode: {}", run_test_mode(&mem));
    println!("Distress signal: {}", run_boost(&mem));
  });
}
