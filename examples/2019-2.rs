use async_std::fs::File;
use async_std::io::BufReader;
use async_std::prelude::*;
use async_std::task;
use std::str;

fn compute(mem: &mut Vec<usize>) -> Result<(), ()> {
  // program counter
  let mut pc = 0;

  loop {
    match mem[pc] {
      99 => break,
      1 => {
        let op1 = mem[pc + 1];
        let op2 = mem[pc + 2];
        let output = mem[pc + 3];
        mem[output] = mem[op1] + mem[op2];
        pc += 4;
      }
      2 => {
        let op1 = mem[pc + 1];
        let op2 = mem[pc + 2];
        let output = mem[pc + 3];
        mem[output] = mem[op1] * mem[op2];
        pc += 4;
      }
      _ => return Err(()),
    }
  }

  Ok(())
}

fn part1(mem: &[usize]) {
  let mut part1_mem = mem.to_vec().clone();

  // Program 1 tweak:
  part1_mem[1] = 12;
  part1_mem[2] = 2;
  let _ = compute(&mut part1_mem);

  println!("Postion 0: {}", part1_mem[0]);
}

fn main() {
  task::block_on(async {
    let file = File::open("inputs/2019/2.txt").await.unwrap();
    let mem: Vec<usize> = BufReader::new(file)
      .split(b',')
      .filter_map(|x| x.ok())
      .filter_map(|x| str::from_utf8(&x).unwrap().trim().parse::<usize>().ok())
      .collect()
      .await;

    part1(&mem);
  });
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let mut state = vec![1, 0, 0, 0, 99];
    let _ = compute(&mut state);
    assert_eq!(vec![2, 0, 0, 0, 99], state);
  }

  #[test]
  fn test2() {
    let mut state = vec![2, 3, 0, 3, 99];
    let _ = compute(&mut state);
    assert_eq!(vec![2, 3, 0, 6, 99], state);
  }

  #[test]
  fn test3() {
    let mut state = vec![2, 4, 4, 5, 99, 0];
    let _ = compute(&mut state);
    assert_eq!(vec![2, 4, 4, 5, 99, 9801], state);
  }

  #[test]
  fn test4() {
    let mut state = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    let _ = compute(&mut state);
    assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], state);
  }
}
