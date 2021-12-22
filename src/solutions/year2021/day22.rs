use itertools::Itertools;
use std::{collections::HashSet, fs};

type Section = (i32, i32);

#[derive(Debug, Clone, Copy)]
struct Cube {
  x: Section,
  y: Section,
  z: Section,
}

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/22.txt").unwrap();

  println!(
    "init cubes: {}",
    content
      .lines()
      .fold(HashSet::new(), |mut acc, line| {
        let mut parts = line.split_ascii_whitespace();
        let turn_on = parts.next().unwrap() == "on";
        let mut sections = parts.next().unwrap().split(",");
        let (x1, x2) = parse_section(sections.next().unwrap());
        if (x1 > 50) || (x2 < -50) {
          return acc;
        }
        let (y1, y2) = parse_section(sections.next().unwrap());
        if (y1 > 50) || (y2 < -50) {
          return acc;
        }
        let (z1, z2) = parse_section(sections.next().unwrap());
        if (z1 > 50) || (z2 < -50) {
          return acc;
        }
        for x in x1..=x2 {
          for y in y1..=y2 {
            for z in z1..=z2 {
              if x >= -50 && x <= 50 && y >= -50 && y <= 50 && z >= -50 && z <= 50 {
                if turn_on {
                  acc.insert((x, y, z));
                } else {
                  acc.remove(&(x, y, z));
                }
              }
            }
          }
        }

        acc
      })
      .len(),
  );

  let mut cubes = content.lines().fold(Vec::new(), |mut acc, line| {
    let mut parts = line.split_ascii_whitespace();
    let turn_on = parts.next().unwrap() == "on";
    let mut sections = parts.next().unwrap().split(",");
    let x = parse_section(sections.next().unwrap());
    let y = parse_section(sections.next().unwrap());
    let z = parse_section(sections.next().unwrap());
    let cube = Cube { x, y, z };

    if turn_on {
      acc.push(cube);
    } else {
      acc = acc
        .into_iter()
        .flat_map(|pc| {
          if let Some(coll) = collide(pc, cube) {
            // collided with a off step, replace it with shards
            [
              [(coll.x.0, coll.x.1), (pc.x.0, coll.x.0 - 1), (coll.x.1 + 1, pc.x.1)],
              [(coll.y.0, coll.y.1), (pc.y.0, coll.y.0 - 1), (coll.y.1 + 1, pc.y.1)],
              [(coll.z.0, coll.z.1), (pc.z.0, coll.z.0 - 1), (coll.z.1 + 1, pc.z.1)],
            ]
            .into_iter()
            .multi_cartesian_product()
            .skip(1) // skip the collision cube, it's off
            .filter_map(|mut ss| {
              let z = ss.pop().unwrap();
              let y = ss.pop().unwrap();
              let x = ss.pop().unwrap();

              (section_valid(x) && section_valid(y) && section_valid(z)).then(|| Cube { x, y, z })
            })
            .collect()
          } else {
            // all good, just wrap it
            vec![pc]
          }
        })
        .collect();
    }

    acc
  });

  // resolve collides
  'resolve: loop {
    for i in 0..cubes.len() {
      for j in 0..i {
        if i != j {
          let nc = cubes[i];
          let pc = cubes[j];
          if let Some(coll) = collide(pc, nc) {
            cubes.remove(i);

            [
              [(coll.x.0, coll.x.1), (nc.x.0, coll.x.0 - 1), (coll.x.1 + 1, nc.x.1)],
              [(coll.y.0, coll.y.1), (nc.y.0, coll.y.0 - 1), (coll.y.1 + 1, nc.y.1)],
              [(coll.z.0, coll.z.1), (nc.z.0, coll.z.0 - 1), (coll.z.1 + 1, nc.z.1)],
            ]
            .into_iter()
            .multi_cartesian_product()
            .skip(1) // skip the collision cube, it's counted in `pc`
            .for_each(|mut ss| {
              let z = ss.pop().unwrap();
              let y = ss.pop().unwrap();
              let x = ss.pop().unwrap();

              if section_valid(x) && section_valid(y) && section_valid(z) {
                cubes.push(Cube { x, y, z });
              }
            });

            continue 'resolve;
          }
        }
      }
    }

    break;
  }

  println!(
    "final on: {}",
    cubes
      .into_iter()
      .map(|c| section_len(c.x) * section_len(c.y) * section_len(c.z))
      .sum::<u64>()
  );
}

fn parse_section(sec: &str) -> Section {
  let mut parts = sec.split_at(2).1.split("..");
  let start = parts.next().unwrap().parse::<i32>().unwrap();
  let end = parts.next().unwrap().parse::<i32>().unwrap();
  (start, end)
}

fn collide(a: Cube, b: Cube) -> Option<Cube> {
  let x = section_collide(a.x, b.x)?;
  let y = section_collide(a.y, b.y)?;
  let z = section_collide(a.z, b.z)?;

  Some(Cube { x, y, z })
}

fn section_collide(a: Section, b: Section) -> Option<Section> {
  if (b.0 >= a.0 && b.0 <= a.1) || (b.1 >= a.0 && b.1 <= a.1) {
    Some((a.0.max(b.0), a.1.min(b.1)))
  } else if b.0 <= a.0 && b.1 >= a.1 {
    Some((a.0, a.1))
  } else {
    None
  }
}

fn section_len(a: Section) -> u64 {
  (a.1 - a.0 + 1) as u64
}

fn section_valid(a: Section) -> bool {
  a.1 >= a.0
}
