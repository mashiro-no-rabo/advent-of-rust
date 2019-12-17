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
    2 => {
      let addr = (rbase as i64 + param) as usize;
      if addr as usize >= mem.len() {
        mem.resize(addr as usize + 1, 0);
      }
      mem[addr as usize]
    }
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
pub struct State {
  pc: usize,
  relative_base: usize,
  mem: Vec<i64>,
}

#[derive(Debug)]
pub enum RunResult {
  WaitingForInput(State, Vec<i64>),
  Halted(Vec<i64>),
}

impl State {
  pub fn from_file(path: &str) -> Self {
    let input = std::fs::read_to_string(path).unwrap();
    let mem: Vec<i64> = input.split(',').map(|x| x.trim().parse::<i64>().unwrap()).collect();
    Self::new_with_mem(&mem)
  }

  pub fn new_with_mem(mem: &[i64]) -> Self {
    Self {
      pc: 0,
      relative_base: 0,
      mem: mem.to_vec(),
    }
  }

  pub fn patch_memory(&mut self, addr: usize, value: i64) {
    self.mem[addr] = value;
  }

  pub fn run(&self, inputs: Vec<i64>) -> RunResult {
    use RunResult::*;
    let mut pc = self.pc; // program counter
    let mut mem = self.mem.clone();
    let mut relative_base = self.relative_base;

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
}

impl RunResult {
  pub fn get_first_output(&self) -> i64 {
    match self {
      Self::WaitingForInput(_, outputs) => *outputs.first().unwrap(),
      Self::Halted(outputs) => *outputs.first().unwrap(),
    }
  }
}
