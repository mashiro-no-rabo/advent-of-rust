use itertools::Itertools;
use std::{collections::HashSet, fs};

type Pos = (i32, i32, i32);
type PosSet = HashSet<Pos>;

#[derive(Debug, Clone)]
struct Scanner {
  // id: u8,
  beacons: PosSet,
  aligned: Option<(PosSet, PosSet, Pos)>,
}

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/19.txt").unwrap();

  let mut lines = content.lines();
  let mut scanners = Vec::new();

  while let Some(_) = lines.next() {
    let mut s = Scanner {
      beacons: HashSet::new(),
      aligned: None,
    };

    while let Some(line) = lines.next() {
      if line.is_empty() {
        break;
      }
      let mut parts = line.split(",");
      let x = parts.next().unwrap().parse::<i32>().unwrap();
      let y = parts.next().unwrap().parse::<i32>().unwrap();
      let z = parts.next().unwrap().parse::<i32>().unwrap();
      s.beacons.insert((x, y, z));
    }

    scanners.push(s);
  }
  scanners[0].aligned = Some((scanners[0].beacons.clone(), scanners[0].beacons.clone(), (0, 0, 0)));

  let mut resolvers = vec![0];

  while let Some(idx) = resolvers.pop() {
    let resolver = scanners[idx].clone();

    scanners
      .iter_mut()
      .enumerate()
      .filter(|(_, s)| s.aligned.is_none())
      .for_each(|(idx, mut s)| {
        if let Some(align) = try_overlap(&resolver, &s) {
          s.aligned = Some(align);
          resolvers.push(idx);
        }
      });

    if scanners.iter().all(|s| s.aligned.is_some()) {
      break;
    }
  }

  println!(
    "beacons: {}",
    scanners
      .iter()
      .fold(HashSet::new(), |mut acc, s| {
        s.aligned.as_ref().unwrap().1.iter().for_each(|&p| {
          acc.insert(p);
        });

        acc
      })
      .len()
  );

  println!(
    "max dist: {}",
    scanners
      .iter()
      .map(|s| { s.aligned.as_ref().unwrap().2 })
      .permutations(2)
      .map(|mut pp| {
        let p1 = pp.pop().unwrap();
        let p2 = pp.pop().unwrap();

        (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs()
      })
      .max()
      .unwrap()
  );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AxisMap {
  X,
  NegX,
  Y,
  NegY,
  Z,
  NegZ,
}

impl AxisMap {
  fn map(&self, pos: Pos) -> i32 {
    match self {
      AxisMap::X => pos.0,
      AxisMap::NegX => -pos.0,
      AxisMap::Y => pos.1,
      AxisMap::NegY => -pos.1,
      AxisMap::Z => pos.2,
      AxisMap::NegZ => -pos.2,
    }
  }
}

type Alignment = (AxisMap, AxisMap, AxisMap);

const ALIGNMENT_MAP: [Alignment; 48] = [
  (AxisMap::X, AxisMap::Y, AxisMap::Z),
  (AxisMap::X, AxisMap::Y, AxisMap::NegZ),
  (AxisMap::X, AxisMap::Z, AxisMap::Y),
  (AxisMap::X, AxisMap::Z, AxisMap::NegY),
  (AxisMap::X, AxisMap::NegY, AxisMap::Z),
  (AxisMap::X, AxisMap::NegY, AxisMap::NegZ),
  (AxisMap::X, AxisMap::NegZ, AxisMap::Y),
  (AxisMap::X, AxisMap::NegZ, AxisMap::NegY),
  (AxisMap::Y, AxisMap::X, AxisMap::Z),
  (AxisMap::Y, AxisMap::X, AxisMap::NegZ),
  (AxisMap::Y, AxisMap::Z, AxisMap::X),
  (AxisMap::Y, AxisMap::Z, AxisMap::NegX),
  (AxisMap::Y, AxisMap::NegX, AxisMap::Z),
  (AxisMap::Y, AxisMap::NegX, AxisMap::NegZ),
  (AxisMap::Y, AxisMap::NegZ, AxisMap::X),
  (AxisMap::Y, AxisMap::NegZ, AxisMap::NegX),
  (AxisMap::Z, AxisMap::X, AxisMap::Y),
  (AxisMap::Z, AxisMap::X, AxisMap::NegY),
  (AxisMap::Z, AxisMap::Y, AxisMap::X),
  (AxisMap::Z, AxisMap::Y, AxisMap::NegX),
  (AxisMap::Z, AxisMap::NegX, AxisMap::Y),
  (AxisMap::Z, AxisMap::NegX, AxisMap::NegY),
  (AxisMap::Z, AxisMap::NegY, AxisMap::X),
  (AxisMap::Z, AxisMap::NegY, AxisMap::NegX),
  (AxisMap::NegX, AxisMap::Y, AxisMap::Z),
  (AxisMap::NegX, AxisMap::Y, AxisMap::NegZ),
  (AxisMap::NegX, AxisMap::Z, AxisMap::Y),
  (AxisMap::NegX, AxisMap::Z, AxisMap::NegY),
  (AxisMap::NegX, AxisMap::NegY, AxisMap::Z),
  (AxisMap::NegX, AxisMap::NegY, AxisMap::NegZ),
  (AxisMap::NegX, AxisMap::NegZ, AxisMap::Y),
  (AxisMap::NegX, AxisMap::NegZ, AxisMap::NegY),
  (AxisMap::NegY, AxisMap::X, AxisMap::Z),
  (AxisMap::NegY, AxisMap::X, AxisMap::NegZ),
  (AxisMap::NegY, AxisMap::Z, AxisMap::X),
  (AxisMap::NegY, AxisMap::Z, AxisMap::NegX),
  (AxisMap::NegY, AxisMap::NegX, AxisMap::Z),
  (AxisMap::NegY, AxisMap::NegX, AxisMap::NegZ),
  (AxisMap::NegY, AxisMap::NegZ, AxisMap::X),
  (AxisMap::NegY, AxisMap::NegZ, AxisMap::NegX),
  (AxisMap::NegZ, AxisMap::X, AxisMap::Y),
  (AxisMap::NegZ, AxisMap::X, AxisMap::NegY),
  (AxisMap::NegZ, AxisMap::Y, AxisMap::X),
  (AxisMap::NegZ, AxisMap::Y, AxisMap::NegX),
  (AxisMap::NegZ, AxisMap::NegX, AxisMap::Y),
  (AxisMap::NegZ, AxisMap::NegX, AxisMap::NegY),
  (AxisMap::NegZ, AxisMap::NegY, AxisMap::X),
  (AxisMap::NegZ, AxisMap::NegY, AxisMap::NegX),
];

fn realign(poses: &PosSet, align: &Alignment) -> PosSet {
  poses
    .iter()
    .map(|&pos| (align.0.map(pos), align.1.map(pos), align.2.map(pos)))
    .collect()
}

fn try_overlap(resolver: &Scanner, other: &Scanner) -> Option<(PosSet, PosSet, Pos)> {
  let a = resolver.aligned.as_ref().unwrap();

  for am in ALIGNMENT_MAP {
    let attempt = realign(&other.beacons, &am);

    for ref_a in a.0.iter() {
      for ref_b in attempt.iter() {
        let dx = ref_a.0 - ref_b.0;
        let dy = ref_a.1 - ref_b.1;
        let dz = ref_a.2 - ref_b.2;

        let b = attempt.iter().map(|&p| (p.0 + dx, p.1 + dy, p.2 + dz)).collect();

        if a.0.intersection(&b).count() >= 12 {
          return Some((attempt, b, (a.2 .0 + dx, a.2 .1 + dy, a.2 .2 + dz)));
        }
      }
    }
  }

  None
}
