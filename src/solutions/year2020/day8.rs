use std::collections::HashSet;
use std::fs;

enum Inst {
  Acc(i32),
  Jmp(i32),
  Nop(i32),
}
use Inst::*;

pub fn solution() {
  let content = fs::read_to_string("inputs/2020/8.txt").unwrap();

  let prog: Vec<Inst> = content
    .lines()
    .map(|line| {
      let mut parts = line.split_ascii_whitespace();
      let op = parts.next().unwrap();
      let arg = parts.next().unwrap().parse::<i32>().unwrap();
      match op {
        "acc" => Acc(arg),
        "jmp" => Jmp(arg),
        "nop" => Nop(arg),
        _ => unimplemented!(),
      }
    })
    .collect();

  {
    // part 1
    let mut acc = 0;
    let mut pc = 0; // Program Counter
    let mut visited = HashSet::new();
    loop {
      visited.insert(pc);
      match prog.get(pc).unwrap() {
        Acc(arg) => {
          acc += arg;
          pc += 1;
        }
        Jmp(arg) => {
          pc = (pc as i32 + arg) as usize;
        }
        Nop(_) => {
          pc += 1;
        }
      }
      if visited.contains(&pc) {
        break;
      }
    }

    println!("Accumulator before loop: {}", acc);
  }

  {
    // part 2
    let mut acc = 0;
    let mut pc = 0; // Program Counter
    let mut visited = HashSet::new();
    let ret = loop {
      visited.insert(pc);
      match prog.get(pc).unwrap() {
        Acc(arg) => {
          acc += arg;
          pc += 1;
        }
        Jmp(arg) => {
          // attempt nop
          if let Some(ret) = terminate(&prog, acc, pc + 1, visited.clone()) {
            break ret;
          }

          // unsucessful, continue with original code
          pc = (pc as i32 + arg) as usize;
        }
        Nop(arg) => {
          // attempt jmp
          if let Some(ret) = terminate(&prog, acc, (pc as i32 + arg) as usize, visited.clone()) {
            break ret;
          }

          // unsucessful, continue with original code
          pc += 1;
        }
      }
    };

    println!("Accumulator after termination: {}", ret);
  }
}

fn terminate(prog: &Vec<Inst>, mut acc: i32, mut pc: usize, mut visited: HashSet<usize>) -> Option<i32> {
  loop {
    visited.insert(pc);
    match prog.get(pc).unwrap() {
      Acc(arg) => {
        acc += arg;
        pc += 1;
      }
      Jmp(arg) => {
        pc = (pc as i32 + arg) as usize;
      }
      Nop(_) => {
        pc += 1;
      }
    }

    // terminated
    if pc == prog.len() - 1 {
      break Some(acc);
    }

    // looped
    if visited.contains(&pc) {
      break None;
    }
  }
}
