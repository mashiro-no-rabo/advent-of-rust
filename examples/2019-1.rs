use async_std::fs::File;
use async_std::io::BufReader;
use async_std::prelude::*;
use async_std::stream::{self, IntoStream, Stream, StreamExt};
use async_std::task;

async fn calc_fuel(mass_str: &str) -> isize {
  match mass_str.parse::<isize>() {
    Ok(mass) => mass / 3 - 2,
    Err(_) => 0,
  }
}

async fn fuel_fuel(fuel: isize) -> isize {
  if fuel < 9 {
    0
  } else {
    fuel / 3 - 2
  }
}

fn main() {
  task::block_on(async {
    let file = File::open("inputs/2019/1_1.txt").await.unwrap();
    let mut lines = BufReader::new(file).lines();
    let s = stream::from_iter(lines);
    s.filter_map(|x| x.parse::<isize>().ok()).map(|x| fuel_fuel(x));

    let x = s.cloned();

    let mut sum = 0;
    let mut fuels = vec![];

    while let Some(line) = lines.next().await {
      let fuel = calc_fuel(&line.unwrap()).await;
      sum += fuel;
      fuels.push(fuel);
    }

    println!("Simple fuel: {}", x.sum().await);

    while let Some(fuel) = fuels.pop() {
      let ff = fuel_fuel(fuel).await;
      if ff > 0 {
        sum += ff;
        fuels.push(ff);
      }
    }

    println!("Real fuel: {}", sum);
  })
}
