use super::int_code::{RunResult::*, State};

fn align_params(st: &State) -> u64 {
  let mut scafs = vec![];

  if let Halted(outputs) = st.run(vec![]) {
    let mut y = 0;
    let mut x = 0;
    outputs.into_iter().for_each(|val| match val as u8 {
      b'#' | b'<' | b'>' | b'^' | b'v' => {
        scafs.push((x, y));
        x += 1;
      }
      b'\n' => {
        y += 1;
        x = 0;
      }
      _ => x += 1,
    })
  }

  scafs.clone().into_iter().fold(0, |acc, (x, y)| {
    if (x > 0)
      && (y > 0)
      && scafs.contains(&(x - 1, y))
      && scafs.contains(&(x + 1, y))
      && scafs.contains(&(x, y - 1))
      && scafs.contains(&(x, y + 1))
    {
      acc + (x * y)
    } else {
      acc
    }
  })
}

pub fn solution() {
  let state = State::from_file("inputs/2019/17.txt");

  println!("Alignment Parameters: {}", align_params(&state));
}
