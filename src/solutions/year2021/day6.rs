use std::fs;

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/6.txt").unwrap();

  let mut states = vec![0; 9];
  content.trim().split(",").for_each(|x| {
    let state = x.parse::<usize>().unwrap();
    states[state] += 1;
  });

  for days in 1..=256 {
    let creators = states.remove(0);
    states.push(creators);
    states[6] += creators;

    if days == 80 {
      println!("after 80 days: {}", states.iter().sum::<u64>());
    }
  }

  println!("after 256 days: {}", states.iter().sum::<u64>());
}
