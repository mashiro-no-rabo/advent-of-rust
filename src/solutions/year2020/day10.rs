use std::fs;

pub fn solution() {
  let content = fs::read_to_string("inputs/2020/10.txt").unwrap();
  let mut adapters: Vec<_> = content.lines().map(|x| x.parse::<u32>().unwrap()).collect();
  adapters.sort();

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
