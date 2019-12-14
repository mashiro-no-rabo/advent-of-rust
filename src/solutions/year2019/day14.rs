use std::collections::HashMap;

type Chemical = String;
type Production = (u64, Chemical);
type Products = HashMap<Chemical, u64>;
type ReactionInput = Vec<Production>;
type Reactions = HashMap<Chemical, (u64, ReactionInput)>;

fn one_fuel(racts: &Reactions) -> u64 {
  fuels(racts, 1)
}

fn bs_trillion_ore(racts: &Reactions) -> u64 {
  let mut lower = 0;
  let mut upper = 1_000_000_000_000;

  while upper - lower > 10 {
    let mid = lower + ((upper - lower) / 2);
    let mid_ores = fuels(racts, mid);

    if mid_ores > 1_000_000_000_000 {
      // search within [lower, mid]
      upper = mid;
    } else {
      lower = mid;
    }
  }

  while fuels(racts, lower + 1) <= 1_000_000_000_000 {
    lower += 1;
  }

  lower
}

fn fuels(racts: &Reactions, want: u64) -> u64 {
  let mut reqs: Products = HashMap::new();
  let mut extra: Products = HashMap::new();
  let mut ores = 0;

  if want > 0 {
    reqs.insert("FUEL".to_string(), want);
  }

  loop {
    if reqs.is_empty() {
      break;
    }

    let mut new_reqs: Products = HashMap::new();

    // reqs are outputs, map them to inputs
    // then combine same Chemicals (not replace)
    reqs.iter().for_each(|(chm, num)| {
      let (inputs, ex) = produce(racts, chm, *num);

      inputs.iter().for_each(|(n, c)| {
        hm_upsert(&mut new_reqs, c, *n);
      });

      // add extra
      if ex > 0 {
        hm_upsert(&mut extra, chm, ex);
      }
    });

    // try use extras
    use_extra(&mut new_reqs, &mut extra);

    // add up OREs
    if let Some(o) = new_reqs.remove("ORE") {
      ores += o;
    }

    // prepare next loop
    reqs = new_reqs;
  }

  ores
}

fn hm_upsert(hm: &mut Products, key: &str, val: u64) {
  if let Some(v) = hm.get_mut(key) {
    *v += val;
  } else {
    hm.insert(key.to_string(), val);
  }
}

fn use_extra(reqs: &mut Products, extra: &mut Products) {
  use std::cmp::Ordering::*;
  let mut req_remove = vec![];
  let mut extra_remove = vec![];

  for (chm, extra_val) in extra.iter_mut() {
    if let Some(req_val) = reqs.get_mut(chm) {
      match (*req_val).cmp(extra_val) {
        Less => {
          // reqs.remove(chm);
          req_remove.push(chm);
          *extra_val -= *req_val;
        }
        Equal => {
          // reqs.remove(chm);
          // extra.remove(chm);
          req_remove.push(chm);
          extra_remove.push(chm.to_string());
        }
        Greater => {
          *req_val -= *extra_val;
          // extra.remove(chm);
          extra_remove.push(chm.to_string());
        }
      }
    }
  }

  for k in req_remove {
    reqs.remove(k);
  }

  for k in extra_remove {
    extra.remove(&k);
  }
}

fn produce(racts: &Reactions, chm: &str, amount: u64) -> (Vec<Production>, u64) {
  let reaction = racts.get(chm).unwrap();
  let per_prod = reaction.0;

  // example: amount = 7, per_prod = 3 (each production produce 3, asking for 7)
  // partial => 7 % 3 = 1, extra => 3 - 1 = 2
  let partial = amount % per_prod;
  let productions = amount / per_prod + (if partial > 0 { 1 } else { 0 });
  let extra = if partial > 0 { per_prod - partial } else { 0 };

  let inputs = reaction
    .1
    .iter()
    .map(|(num, chm)| (num * productions, chm.to_string()))
    .collect();

  (inputs, extra)
}

fn parse_reactions(input: &str) -> Reactions {
  input
    .lines()
    .map(|ln| {
      let mut parts = ln.split(" => ");

      let rin = parts
        .next()
        .unwrap()
        .split(", ")
        .map(|p| parse_production(p))
        .collect::<ReactionInput>();

      let rout = parse_production(parts.next().unwrap());

      (rout.1, (rout.0, rin))
    })
    .collect()
}

fn parse_production(input: &str) -> Production {
  let mut parts = input.split_ascii_whitespace();
  (
    parts.next().unwrap().parse::<u64>().unwrap(),
    parts.next().unwrap().to_string(),
  )
}

pub fn solution() {
  let input = std::fs::read_to_string("inputs/2019/14.txt").unwrap();
  let reactions = parse_reactions(&input);

  println!("1 FUEL needs: {} OREs", one_fuel(&reactions));
  println!("1 trillion OREs can produce: {} FUEL", bs_trillion_ore(&reactions));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let input = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
    let reactions = parse_reactions(&input);
    assert_eq!(31, one_fuel(&reactions));
  }

  #[test]
  fn test2() {
    let input = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
    let reactions = parse_reactions(&input);
    assert_eq!(165, one_fuel(&reactions));
  }

  #[test]
  fn test3() {
    let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
    let reactions = parse_reactions(&input);
    assert_eq!(13312, one_fuel(&reactions));
    assert_eq!(82_892_753, bs_trillion_ore(&reactions));
  }

  #[test]
  fn test4() {
    let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";
    let reactions = parse_reactions(&input);
    assert_eq!(180_697, one_fuel(&reactions));
    assert_eq!(5_586_022, bs_trillion_ore(&reactions));
  }

  #[test]
  fn test5() {
    let input = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
    let reactions = parse_reactions(&input);
    assert_eq!(2_210_736, one_fuel(&reactions));
    assert_eq!(460_664, bs_trillion_ore(&reactions));
  }
}
