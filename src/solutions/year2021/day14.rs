use itertools::{Itertools, MinMaxResult};
use std::{cell::RefCell, collections::HashMap, fs, rc::Rc};

type Rules = HashMap<(char, char), char>;

type CharCount = HashMap<char, u64>;
type CacheKey0 = (char, char, char);
type CacheInner = HashMap<(CacheKey0, u8), CharCount>;
type Cache = Rc<RefCell<CacheInner>>;

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/14.txt").unwrap();

  let mut lines = content.lines();
  let template = lines.next().unwrap();

  let rules = lines.skip(1).fold(Rules::new(), |mut acc, line| {
    let mut parts = line.split(" -> ");
    let from = {
      let mut s = parts.next().unwrap().chars();
      let a = s.next().unwrap();
      let b = s.next().unwrap();
      (a, b)
    };
    let to = parts.next().unwrap().chars().next().unwrap();
    acc.insert(from, to);
    acc
  });

  println!("diff 10: {}", run(template, 10, &rules));
  println!("diff 40: {}", run(template, 40, &rules));
}

fn run(template: &str, end: u8, rules: &Rules) -> u64 {
  let cache = Rc::new(RefCell::new(HashMap::new()));
  let mut result = template
    .chars()
    .into_iter()
    .tuple_windows()
    .fold(CharCount::new(), |mut acc, (a, b, c)| {
      ins((a, b, c), 0, end, &rules, &cache).into_iter().for_each(|(k, v)| {
        *acc.entry(k).or_insert(0) += v;
      });
      acc
    });

  // need to remove the overlapped ones
  template
    .chars()
    .into_iter()
    .skip(1)
    .tuple_windows()
    .for_each(|(a, b, _)| {
      let mid_template = (a, rules.get(&(a, b)).unwrap().clone(), b);
      ins(mid_template, 1, end, &rules, &cache)
        .into_iter()
        .for_each(|(k, v)| {
          *result.get_mut(&k).unwrap() -= v;
        });
    });

  if let MinMaxResult::MinMax(a, z) = result.into_iter().minmax_by_key(|a| a.1) {
    z.1 - a.1
  } else {
    unimplemented!()
  }
}

fn ins(template: (char, char, char), step: u8, end: u8, rules: &Rules, cache: &Cache) -> CharCount {
  if let Some(ret) = cache.borrow().get(&(template, step)) {
    return ret.clone();
  }

  let mut c = CharCount::new();

  if step == end {
    *c.entry(template.0).or_insert(0) += 1;
    *c.entry(template.1).or_insert(0) += 1;
    *c.entry(template.2).or_insert(0) += 1;
  } else {
    let left_template = (
      template.0,
      rules.get(&(template.0, template.1)).unwrap().clone(),
      template.1,
    );
    let lc = ins(left_template, step + 1, end, rules, &cache.clone());
    lc.into_iter().for_each(|(k, v)| *c.entry(k).or_insert(0) += v);

    let right_template = (
      template.1,
      rules.get(&(template.1, template.2)).unwrap().clone(),
      template.2,
    );
    let rc = ins(right_template, step + 1, end, rules, &cache.clone());
    rc.into_iter().for_each(|(k, v)| *c.entry(k).or_insert(0) += v);

    *c.get_mut(&template.1).unwrap() -= 1;
  }

  cache.borrow_mut().insert((template, step), c.clone());

  c
}
