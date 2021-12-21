use std::fs;

#[derive(Debug, Clone, Copy)]
struct Player {
  pos: u32,
  score: u32,
}

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/21.txt").unwrap();

  let mut lines = content.lines();
  let p1 = Player {
    pos: lines.next().unwrap().split_at(28).1.parse::<u32>().unwrap(),
    score: 0,
  };
  let p2 = Player {
    pos: lines.next().unwrap().split_at(28).1.parse::<u32>().unwrap(),
    score: 0,
  };

  println!("deterministic dice: {}", play_deterministic(p1, p2));

  let (a, b) = play_dirac(p1, p2, true);
  println!("dirac dice: {}", a.max(b));
}

fn play_deterministic(mut p1: Player, mut p2: Player) -> u32 {
  let mut dice_times = 0;
  let mut roll = || -> u32 {
    let ret = dice_times % 100 + 1;
    dice_times += 1;
    ret
  };

  let loser_score = loop {
    let step = roll() + roll() + roll();
    p1.pos = (p1.pos + step % 10 - 1) % 10 + 1;
    p1.score += p1.pos;
    if p1.score >= 1000 {
      break p2.score;
    }

    let step = roll() + roll() + roll();
    p2.pos = (p2.pos + step % 10 - 1) % 10 + 1;
    p2.score += p2.pos;
    if p2.score >= 1000 {
      break p1.score;
    }
  };

  dice_times * loser_score
}

const DIRAC: [(u32, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn play_dirac(p1: Player, p2: Player, play_p1: bool) -> (u64, u64) {
  let mut ret = (0, 0);

  if play_p1 {
    for (step, us) in DIRAC {
      let mut new_p1 = p1;
      new_p1.pos = (new_p1.pos + step - 1) % 10 + 1;
      new_p1.score += new_p1.pos;
      if new_p1.score >= 21 {
        ret.0 += us;
      } else {
        let (us1, us2) = play_dirac(new_p1, p2, false);
        ret.0 += us * us1;
        ret.1 += us * us2;
      }
    }
  } else {
    for (step, us) in DIRAC {
      let mut new_p2 = p2;
      new_p2.pos = (new_p2.pos + step - 1) % 10 + 1;
      new_p2.score += new_p2.pos;
      if new_p2.score >= 21 {
        ret.1 += us;
      } else {
        let (us1, us2) = play_dirac(p1, new_p2, true);
        ret.0 += us * us1;
        ret.1 += us * us2;
      }
    }
  }

  ret
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_examples() {
    let p1 = Player { pos: 4, score: 0 };
    let p2 = Player { pos: 8, score: 0 };
    assert_eq!(play_dirac(p1, p2, true), (444356092776315, 341960390180808));
  }
}
