use std::{
  collections::{HashMap, HashSet},
  fs,
};

type WireToSeg = HashMap<char, char>;

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/8.txt").unwrap();

  let l_1478 = [2, 3, 4, 7];

  let p1 = content
    .trim()
    .lines()
    .map(|line| {
      let digits = line.split(" | ").skip(1).next().unwrap();

      digits
        .split_ascii_whitespace()
        .filter(|d| l_1478.contains(&d.len()))
        .count()
    })
    .sum::<usize>();

  println!("1478: {}", p1);

  let p2 = content
    .trim()
    .lines()
    .map(|line| {
      let mut parts = line.split(" | ");
      let wts = decrypt(parts.next().unwrap());
      decode(parts.next().unwrap(), &wts)
    })
    .sum::<u32>();

  println!("sum: {}", p2);
}

fn decode(segs: &str, wts: &WireToSeg) -> u32 {
  let mut stw = HashMap::<char, char>::new();
  wts.iter().for_each(|(k, v)| {
    stw.insert(*v, *k);
  });

  segs
    .split_ascii_whitespace()
    .map(|seg| {
      let mut real_seg: Vec<char> = seg.chars().map(|c| stw.get(&c).unwrap().clone()).collect();
      real_seg.sort();
      let real_seg: String = real_seg.into_iter().collect();

      match real_seg.as_str() {
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" => 2,
        "acdfg" => 3,
        "bcdf" => 4,
        "abdfg" => 5,
        "abdefg" => 6,
        "acf" => 7,
        "abcdefg" => 8,
        "abcdfg" => 9,
        _ => unimplemented!(),
      }
    })
    .rev()
    .fold((1, 0), |acc, d| {
      let add = acc.0 * d;
      (acc.0 * 10, acc.1 + add)
    })
    .1
}

fn decrypt(signals: &str) -> WireToSeg {
  let mut sig1 = None;
  let mut sig4 = None;
  let mut sig7 = None;
  let mut sig8 = None;
  let mut sig_069 = vec![];
  let mut sig_235 = vec![];

  signals.split_ascii_whitespace().for_each(|sig| {
    let mut s = HashSet::new();
    sig.clone().chars().for_each(|c| {
      s.insert(c);
    });

    match sig.len() {
      2 => sig1 = Some(s),
      4 => sig4 = Some(s),
      3 => sig7 = Some(s),
      7 => sig8 = Some(s),
      6 => sig_069.push(s),
      5 => sig_235.push(s),
      _ => unimplemented!(),
    }
  });

  let sig1 = sig1.unwrap();
  let sig4 = sig4.unwrap();
  let sig7 = sig7.unwrap();
  let sig8 = sig8.unwrap();

  let mut wts = WireToSeg::new();
  wts.insert('a', sig7.difference(&sig1).next().unwrap().clone());
  for s in sig_069.iter() {
    for c in sig8.difference(s) {
      if sig7.contains(c) {
        wts.insert('c', c.clone());
      }
    }
  }
  {
    let ac = HashSet::from([wts[&'a'], wts[&'c']]);
    wts.insert('f', sig7.difference(&ac).next().unwrap().clone());
  }
  {
    let dce: HashSet<char> = sig_069
      .iter()
      .map(|s| sig8.difference(s).next().unwrap().clone())
      .collect();
    wts.insert('e', dce.difference(&sig4).next().unwrap().clone());
  }
  {
    wts.insert(
      'g',
      sig8
        .difference(&sig4)
        .filter(|&&c| c != wts[&'a'] && c != wts[&'e'])
        .next()
        .unwrap()
        .clone(),
    );
  }
  {
    let tmp: HashSet<char> = sig_235
      .iter()
      .fold(sig8.clone(), |acc, s| acc.intersection(s).cloned().collect());

    wts.insert(
      'd',
      tmp
        .into_iter()
        .filter(|&c| c != wts[&'a'] && c != wts[&'g'])
        .next()
        .unwrap(),
    );
  }
  {
    wts.insert(
      'b',
      sig4
        .into_iter()
        .filter(|&c| c != wts[&'c'] && c != wts[&'d'] && c != wts[&'f'])
        .next()
        .unwrap(),
    );
  }

  wts
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_simple() {
    let wts = decrypt("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab");
    assert_eq!(
      wts,
      HashMap::from([
        ('a', 'd'),
        ('b', 'e'),
        ('c', 'a'),
        ('d', 'f'),
        ('e', 'g'),
        ('f', 'b'),
        ('g', 'c')
      ])
    );

    assert_eq!(decode("cdfeb fcadb cdfeb cdbaf", &wts), 5353);
  }
}
