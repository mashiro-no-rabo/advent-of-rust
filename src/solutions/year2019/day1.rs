use async_std::fs::File;
use async_std::io::BufReader;
use async_std::prelude::*;
use async_std::stream;
use async_std::task;

fn calc_fuel(mass: i64) -> i64 {
  // 8 div 3 - 2 = 0, 9 div 3 - 2 = 1
  if mass < 9 {
    0
  } else {
    mass / 3 - 2
  }
}

pub fn solution() {
  task::block_on(async {
    let file = File::open("inputs/2019/1.txt").await.unwrap();
    let lines: Vec<i64> = BufReader::new(file)
      .lines()
      .filter_map(|x| x.ok())
      .filter_map(|x| x.parse::<i64>().ok())
      .collect()
      .await;

    let simple_sum = stream::from_iter(lines.clone()).map(calc_fuel).sum().await;

    println!("Simple fuel: {}", simple_sum);

    let real_sum = stream::from_iter(lines)
      .map(|mut x| {
        // Make a stream of fuel requirements for fuel
        stream::from_fn(move || {
          if x > 0 {
            x = calc_fuel(x);
            Some(x)
          } else {
            None
          }
        })
      })
      .flatten()
      .sum()
      .await;

    println!("Real fuel: {}", real_sum);
  })
}
