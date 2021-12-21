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
