use std::fs;

#[derive(PartialEq, Eq, Hash)]
enum Val {
  One(u32),
  Two(u32, u32),
}

pub fn solution() {
  let content = fs::read_to_string("inputs/2020/1.txt").unwrap();

  let mut expecting = vec![];
  let mut multiply2 = None;

  let mut attempts: Vec<Val> = vec![];
  let mut multiply3 = None;

  for line in content.lines() {
    let x = line.parse::<u32>().unwrap();
    if multiply2.is_none() && expecting.contains(&x) {
      multiply2 = Some(x * (2020 - x));
    }
    expecting.push(2020 - x);

    if multiply3.is_none() {
      let mut extends = vec![];
      for atm in attempts.iter() {
        match atm {
          Val::One(a) => {
            if a + x < 2020 {
              extends.push(Val::Two(*a, x))
            }
          }
          Val::Two(a, b) => {
            if a + b + x == 2020 {
              multiply3 = Some(a * b * x);
            }
          }
        }
      }

      attempts.extend(extends);
      attempts.push(Val::One(x));
    }

    if multiply2.is_some() && multiply3.is_some() {
      break;
    }
  }

  println!("Multiply2: {}", multiply2.unwrap());
  println!("Multiply3: {}", multiply3.unwrap());
}
