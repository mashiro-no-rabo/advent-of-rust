use async_std::fs::File;
use async_std::io::BufReader;
use async_std::prelude::*;
use async_std::task;
use std::str;

type InstParamMode = (u8, Vec<u8>);
fn parse_inst(code: i64) -> Result<InstParamMode, ()> {
  if code < 0 {
    return Err(());
  }

  let inst = code % 100;

  let mut param_modes = vec![0; 3];
  let mut modes = code / 100;
  let mut idx = 0;
  while modes > 9 {
    param_modes[idx] = (modes % 10) as u8;
    modes /= 10;
    idx += 1;
  }
  param_modes[idx] = modes as u8;

  Ok((inst as u8, param_modes))
}

fn read_param(param: i64, mode: u8, mem: &[i64]) -> i64 {
  match mode {
    0 => mem[param as usize],
    1 => param,
    _ => unimplemented!(),
  }
}

fn run_intcode(mem: &mut Vec<i64>, input: Vec<i64>) -> Result<Vec<i64>, ()> {
  let mut pc = 0; // program counter
  let mut input_idx = 0;
  let mut outputs = vec![];

  loop {
    // parse instruction
    let (inst, pmode) = parse_inst(mem[pc])?;

    match inst {
      99 => break,
      1 => {
        // plus
        let pstart = pc + 1;
        let op1 = read_param(mem[pstart], pmode[0], &mem);
        let op2 = read_param(mem[pstart + 1], pmode[1], &mem);
        let target = mem[pc + 3];
        mem[target as usize] = op1 + op2;
        pc += 4;
      }
      2 => {
        // multiply
        let pstart = pc + 1;
        let op1 = read_param(mem[pstart], pmode[0], &mem);
        let op2 = read_param(mem[pstart + 1], pmode[1], &mem);
        let target = mem[pc + 3];
        mem[target as usize] = op1 * op2;
        pc += 4;
      }
      3 => {
        // input
        let target = mem[pc + 1];
        mem[target as usize] = input[input_idx];
        input_idx += 1;
        pc += 2;
      }
      4 => {
        // output
        let output = read_param(mem[pc + 1], pmode[0], &mem);
        outputs.push(output);
        pc += 2;
      }
      5 => {
        // jump-if-true
        let op1 = read_param(mem[pc + 1], pmode[0], &mem);
        if op1 != 0 {
          pc = read_param(mem[pc + 2], pmode[1], &mem) as usize
        } else {
          pc += 3;
        }
      }
      6 => {
        // jump-if-false
        let op1 = read_param(mem[pc + 1], pmode[0], &mem);
        if op1 == 0 {
          pc = read_param(mem[pc + 2], pmode[1], &mem) as usize
        } else {
          pc += 3;
        }
      }
      7 => {
        // less-than
        let pstart = pc + 1;
        let op1 = read_param(mem[pstart], pmode[0], &mem);
        let op2 = read_param(mem[pstart + 1], pmode[1], &mem);
        let target = mem[pc + 3];
        mem[target as usize] = if op1 < op2 { 1 } else { 0 };
        pc += 4;
      }
      8 => {
        // equals
        let pstart = pc + 1;
        let op1 = read_param(mem[pstart], pmode[0], &mem);
        let op2 = read_param(mem[pstart + 1], pmode[1], &mem);
        let target = mem[pc + 3];
        mem[target as usize] = if op1 == op2 { 1 } else { 0 };
        pc += 4;
      }
      _ => return Err(()),
    }
  }

  Ok(outputs)
}

pub fn solution() {
  task::block_on(async {
    let file = File::open("inputs/2019/5.txt").await.unwrap();
    let mem: Vec<i64> = BufReader::new(file)
      .split(b',')
      .filter_map(|x| x.ok())
      .filter_map(|x| str::from_utf8(&x).unwrap().trim().parse::<i64>().ok())
      .collect()
      .await;

    let mut run1_mem = mem.clone();

    println!(
      "Diagnostic code for system 1: {}",
      run_intcode(&mut run1_mem, vec![1]).unwrap().last().unwrap()
    );

    let mut run2_mem = mem.clone();

    println!(
      "Diagnostic code for system 5: {}",
      run_intcode(&mut run2_mem, vec![5]).unwrap().last().unwrap()
    );
  });
}
