use std::collections::HashMap;
use std::fs;

// down is x, right is y

pub fn solution() {
  let content = fs::read_to_string("inputs/2020/11.txt").unwrap();
  let width = (content.lines().nth(0).unwrap().len()) as i32; // max y, y starts from 0
  let height = (content.lines().count()) as i32;
  let max_scale = width.max(height);

  let mut seats = HashMap::new();
  content.lines().enumerate().for_each(|(x, line)| {
    line.chars().enumerate().for_each(|(y, ch)| match ch {
      'L' => {
        // optimize, first round always all seast got occupied
        seats.insert((x as i32, y as i32), true);
      }
      _ => {}
    });
  });

  {
    // part 1
    let mut seats = seats.clone();

    loop {
      let mut new_seats = HashMap::new();
      let mut dirty = false;

      seats.iter().for_each(|(pos, occupied)| {
        let ns = count_neighbors(&seats, *pos);
        if *occupied && ns >= 4 {
          new_seats.insert(*pos, false);
          dirty |= true;
        } else if !*occupied && ns == 0 {
          new_seats.insert(*pos, true);
          dirty |= true;
        } else {
          new_seats.insert(*pos, *occupied);
        }
      });

      seats = new_seats;
      if !dirty {
        break;
      }
    }

    println!("Seats occupied: {}", seats.values().filter(|x| **x).count());
  }

  {
    // part2
    loop {
      let mut new_seats = HashMap::new();
      let mut dirty = false;

      seats.iter().for_each(|(pos, occupied)| {
        let ns = count_neighbors_better(&seats, *pos, max_scale);
        if *occupied && ns >= 5 {
          new_seats.insert(*pos, false);
          dirty |= true;
        } else if !*occupied && ns == 0 {
          new_seats.insert(*pos, true);
          dirty |= true;
        } else {
          new_seats.insert(*pos, *occupied);
        }
      });

      seats = new_seats;
      if !dirty {
        break;
      }
    }

    println!("Seats occupied2: {}", seats.values().filter(|x| **x).count());
  }
}

fn count_neighbors(seats: &HashMap<(i32, i32), bool>, pos: (i32, i32)) -> u8 {
  let mut count = 0;
  for xd in -1..=1 {
    for yd in -1..=1 {
      if xd != 0 || yd != 0 {
        match seats.get(&(pos.0 + xd, pos.1 + yd)) {
          Some(&true) => count += 1,
          _ => {}
        }
      }
    }
  }

  count
}

fn count_neighbors_better(seats: &HashMap<(i32, i32), bool>, pos: (i32, i32), max_scale: i32) -> u8 {
  let mut count = 0;

  for xd in -1..=1 {
    for yd in -1..=1 {
      if xd == 0 && yd == 0 {
        continue;
      }

      for scale in 1..=max_scale {
        match seats.get(&(pos.0 + (xd * scale), pos.1 + (yd * scale))) {
          Some(&true) => {
            count += 1;
            break;
          }
          Some(_) => {
            break;
          }
          _ => {}
        }
      }
    }
  }

  count
}
