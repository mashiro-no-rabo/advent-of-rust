use std::{collections::HashMap, fs};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ALU {
  regs: [i64; 4],
}

#[derive(Debug, Clone, Copy)]
enum NumOrReg {
  Reg(usize),
  Num(i64),
}

#[derive(Debug, Clone, Copy)]
enum Inst {
  Input(usize),
  Add(usize, NumOrReg),
  Mul(usize, NumOrReg),
  Div(usize, NumOrReg),
  Mod(usize, NumOrReg),
  Eql(usize, NumOrReg),
}

impl Inst {
  fn from_line(line: &str) -> Self {
    let mut parts = line.split_whitespace();
    let op = parts.next().unwrap();
    let a = match parts.next().unwrap() {
      "w" => 0,
      "x" => 1,
      "y" => 2,
      "z" => 3,
      _ => unimplemented!(),
    };
    if op == "inp" {
      Inst::Input(a)
    } else {
      let b = match parts.next().unwrap() {
        "w" => NumOrReg::Reg(0),
        "x" => NumOrReg::Reg(1),
        "y" => NumOrReg::Reg(2),
        "z" => NumOrReg::Reg(3),
        n => NumOrReg::Num(n.parse().unwrap()),
      };
      match op {
        "add" => Inst::Add(a, b),
        "mul" => Inst::Mul(a, b),
        "div" => Inst::Div(a, b),
        "mod" => Inst::Mod(a, b),
        "eql" => Inst::Eql(a, b),
        _ => unimplemented!(),
      }
    }
  }
}

impl ALU {
  fn new() -> Self {
    ALU { regs: [0; 4] }
  }

  fn get(&self, nr: NumOrReg) -> i64 {
    match nr {
      NumOrReg::Reg(r) => self.regs[r],
      NumOrReg::Num(n) => n,
    }
  }

  fn exec(&mut self, inst: Inst) {
    match inst {
      Inst::Add(a, b) => self.regs[a] += self.get(b),
      Inst::Mul(a, b) => self.regs[a] *= self.get(b),
      Inst::Div(a, b) => self.regs[a] /= self.get(b),
      Inst::Mod(a, b) => self.regs[a] %= self.get(b),
      Inst::Eql(a, b) => self.regs[a] = if self.regs[a] == self.get(b) { 1 } else { 0 },
      Inst::Input(_) => unimplemented!(),
    }
  }
}

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/24.txt").unwrap();

  let insts: Vec<_> = content.lines().map(Inst::from_line).collect();

  let mut states = vec![(ALU::new(), (0, 0))];

  insts.into_iter().for_each(|inst| match inst {
    Inst::Input(a) => {
      let mut new_states: Vec<(ALU, (u64, u64))> = Vec::new();
      let mut idx: HashMap<ALU, usize> = HashMap::new();
      for (alu, (pmax, pmin)) in &states {
        for input in 1..=9 {
          let mut new_alu = alu.clone();
          new_alu.regs[a] = input;
          let nmax = pmax * 10 + input as u64;
          let nmin = pmin * 10 + input as u64;

          if let Some(idx) = idx.get(&new_alu) {
            new_states[*idx].1 .0 = new_states[*idx].1 .0.max(nmax);
            new_states[*idx].1 .1 = new_states[*idx].1 .1.min(nmin);
          } else {
            idx.insert(new_alu.clone(), new_states.len());
            new_states.push((new_alu, (nmax, nmin)));
          }
        }
      }
      states = new_states;
    }
    inst => {
      for (alu, _) in &mut states {
        alu.exec(inst);
      }
    }
  });

  let (max, min) = states
    .into_iter()
    .filter_map(|(alu, mm)| (alu.regs[3] == 0).then(|| mm))
    .reduce(|(pmax, pmin), (nmax, nmin)| (pmax.max(nmax), pmin.min(nmin)))
    .unwrap();

  println!("max: {}, min: {}", max, min);
}
