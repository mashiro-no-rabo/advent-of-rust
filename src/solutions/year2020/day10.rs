use std::collections::HashMap;
use std::fs;

pub fn solution() {
  let content = fs::read_to_string("inputs/2020/10.txt").unwrap();
  let mut adapters: Vec<_> = content.lines().map(|x| x.parse::<u32>().unwrap()).collect();
  adapters.sort();

  {
    // part 1
    let (diff1, _, diff3, _) = adapters
      .iter()
      .cloned()
      .fold((0, 0, 0, 0), |(d1, d2, d3, prev), a| match a - prev {
        1 => (d1 + 1, d2, d3, a),
        2 => (d1, d2 + 1, d3, a),
        3 => (d1, d2, d3 + 1, a),
        _ => unimplemented!(),
      });

    println!("Multiply: {}", diff1 * (diff3 + 1));
  }

  {
    // part 2
    let mut ways = HashMap::new();
    ways.insert(0 as i32, 1 as u64);
    let w = adapters
      .into_iter()
      .map(|n| {
        let n_1 = *(ways.get(&(n as i32 - 1)).unwrap_or(&0));
        let n_2 = *(ways.get(&(n as i32 - 2)).unwrap_or(&0));
        let n_3 = *(ways.get(&(n as i32 - 3)).unwrap_or(&0));
        let n_ways = n_1 + n_2 + n_3;
        ways.insert(n as i32, n_ways);

        n_ways
      })
      .last()
      .unwrap();

    println!("Ways: {}", w);
  }
}
