use async_std::fs::File;
use async_std::io::BufReader;
use async_std::prelude::*;
use async_std::task;
use std::str;

type InstParamMode = (u8, Vec<u8>);
fn parse_inst(code: i64) -> InstParamMode {
  if code < 0 {
    unimplemented!()
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

  (inst as u8, param_modes)
}

fn read_param(param: i64, mode: u8, mem: &mut Vec<i64>, rbase: usize) -> i64 {
  match mode {
    0 => {
      if param as usize >= mem.len() {
        mem.resize(param as usize + 1, 0);
      }
      mem[param as usize]
    }
    1 => param,
    2 => mem[(rbase as i64 + param) as usize],
    _ => unimplemented!(),
  }
}

fn read_address(param: i64, mode: u8, rbase: usize) -> usize {
  match mode {
    0 => param as usize,
    2 => (rbase as i64 + param) as usize,
    // immediate mode can't be used as address
    _ => unimplemented!(),
  }
}

fn write_address(mem: &mut Vec<i64>, address: usize, value: i64) {
  if address >= mem.len() {
    mem.resize(address + 1, 0);
  }
  mem[address] = value;
}

#[derive(Debug, Clone)]
struct State {
  pc: usize,
  relative_base: usize,
  mem: Vec<i64>,
}

impl State {
  fn new_with_mem(mem: &[i64]) -> Self {
    Self {
      pc: 0,
      relative_base: 0,
      mem: mem.to_vec(),
    }
  }
}

enum RunResult {
  WaitingForInput(State, Vec<i64>),
  Halted(Vec<i64>),
}
use RunResult::*;

fn run_intcode(state: &State, inputs: Vec<i64>) -> RunResult {
  let mut pc = state.pc; // program counter
  let mut mem = state.mem.clone();
  let mut relative_base = state.relative_base;

  let mut input_idx = 0;
  let mut outputs = vec![];

  loop {
    // parse instruction
    let (inst, pmode) = parse_inst(mem[pc]);

    match inst {
      99 => return Halted(outputs),
      1 => {
        // plus
        let pstart = pc + 1;
        let op1 = read_param(mem[pstart], pmode[0], &mut mem, relative_base);
        let op2 = read_param(mem[pstart + 1], pmode[1], &mut mem, relative_base);
        let target = read_address(mem[pstart + 2], pmode[2], relative_base);
        write_address(&mut mem, target, op1 + op2);
        pc += 4;
      }
      2 => {
        // multiply
        let pstart = pc + 1;
        let op1 = read_param(mem[pstart], pmode[0], &mut mem, relative_base);
        let op2 = read_param(mem[pstart + 1], pmode[1], &mut mem, relative_base);
        let target = read_address(mem[pstart + 2], pmode[2], relative_base);
        write_address(&mut mem, target, op1 * op2);
        pc += 4;
      }
      3 => {
        // input
        if input_idx < inputs.len() {
          let target = read_address(mem[pc + 1], pmode[0], relative_base);
          write_address(&mut mem, target, inputs[input_idx]);
          input_idx += 1;
          pc += 2;
        } else {
          let paused_state = State { pc, relative_base, mem };
          return WaitingForInput(paused_state, outputs);
        }
      }
      4 => {
        // output
        let output = read_param(mem[pc + 1], pmode[0], &mut mem, relative_base);
        outputs.push(output);
        pc += 2;
      }
      5 => {
        // jump-if-true
        let op1 = read_param(mem[pc + 1], pmode[0], &mut mem, relative_base);
        if op1 != 0 {
          pc = read_param(mem[pc + 2], pmode[1], &mut mem, relative_base) as usize
        } else {
          pc += 3;
        }
      }
      6 => {
        // jump-if-false
        let op1 = read_param(mem[pc + 1], pmode[0], &mut mem, relative_base);
        if op1 == 0 {
          pc = read_param(mem[pc + 2], pmode[1], &mut mem, relative_base) as usize
        } else {
          pc += 3;
        }
      }
      7 => {
        // less-than
        let pstart = pc + 1;
        let op1 = read_param(mem[pstart], pmode[0], &mut mem, relative_base);
        let op2 = read_param(mem[pstart + 1], pmode[1], &mut mem, relative_base);
        let target = read_address(mem[pstart + 2], pmode[2], relative_base);
        write_address(&mut mem, target, if op1 < op2 { 1 } else { 0 });
        pc += 4;
      }
      8 => {
        // equals
        let pstart = pc + 1;
        let op1 = read_param(mem[pstart], pmode[0], &mut mem, relative_base);
        let op2 = read_param(mem[pstart + 1], pmode[1], &mut mem, relative_base);
        let target = read_address(mem[pstart + 2], pmode[2], relative_base);
        write_address(&mut mem, target, if op1 == op2 { 1 } else { 0 });
        pc += 4;
      }
      9 => {
        // relative base offset
        let offset = read_param(mem[pc + 1], pmode[0], &mut mem, relative_base);
        relative_base = (relative_base as i64 + offset) as usize;
        pc += 2;
      }
      _ => unimplemented!(),
    }
  }
}

fn count_block_tiles(mem: &[i64]) -> usize {
  let state = State::new_with_mem(mem);
  if let Halted(outputs) = run_intcode(&state, vec![]) {
    use std::collections::HashSet;
    let mut blocks = HashSet::new();
    let mut iter = outputs.iter();
    while let Some(x) = iter.next() {
      let y = iter.next().unwrap();
      let id = iter.next().unwrap();
      if *id == 2 {
        blocks.insert((*x, *y));
      }
    }
    blocks.len()
  } else {
    0
  }
}

fn main() {
  task::block_on(async {
    let file = File::open("inputs/2019/13.txt").await.unwrap();
    let mem: Vec<i64> = BufReader::new(file)
      .split(b',')
      .filter_map(|x| x.ok())
      .filter_map(|x| str::from_utf8(&x).unwrap().trim().parse::<i64>().ok())
      .collect()
      .await;

    println!("Block tiles: {}", count_block_tiles(&mem));
  });
}
