use std::fs;

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/2.txt").unwrap();

  {
    let final_loc = content.trim().lines().fold((0, 0), |location, line| {
      let mut parts = line.split(" ");
      let cmd = parts.next().unwrap();
      let delta = parts.next().unwrap().parse::<u32>().unwrap();
      match cmd {
        "forward" => (location.0 + delta, location.1),
        "down" => (location.0, location.1 + delta),
        "up" => (location.0, location.1 - delta),
        _ => unimplemented!(),
      }
    });

    println!("Answer 1: {}", final_loc.0 * final_loc.1);
  }

  {
    let final_loc = content.trim().lines().fold((0, 0, 0), |location, line| {
      let mut parts = line.split(" ");
      let cmd = parts.next().unwrap();
      let delta = parts.next().unwrap().parse::<u32>().unwrap();
      match cmd {
        "forward" => (location.0 + delta, location.1, location.2 + (location.1 * delta)),
        "down" => (location.0, location.1 + delta, location.2),
        "up" => (location.0, location.1 - delta, location.2),
        _ => unimplemented!(),
      }
    });

    println!("Answer 2: {}", final_loc.0 * final_loc.2);
  }
}
