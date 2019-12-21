use std::collections::{HashMap, HashSet};

use super::map::*;

#[derive(Debug, Clone, PartialEq)]
enum Tile {
  Open,
  Portal(char, char),
}
use Tile::*;

impl Tile {
  fn is_start_portal(&self) -> bool {
    match self {
      Portal('A', 'A') => true,
      _ => false,
    }
  }
}

type Map = HashMap<Position, Tile>;

fn parse_map(input: &str) -> Map {
  let mut portals = HashMap::new();
  let mut map = HashMap::new();
  for (y, line) in input.lines().enumerate() {
    for (x, ch) in line.chars().enumerate() {
      let pos = (x as i64, y as i64);
      match ch {
        'A'..='Z' => {
          portals.insert(pos, ch);
          if let Some(other_pos) = Direction::iter_all()
            .map(|dir| dir.move_position(&pos))
            .find(|pos| portals.contains_key(&pos))
          {
            // find portal and insert both
            let other_ch = *portals.get(&other_pos).unwrap();
            let portal = if ch < other_ch {
              Portal(ch, other_ch)
            } else {
              Portal(other_ch, ch)
            };

            map.insert(pos, portal.clone());
            map.insert(other_pos, portal);
          }
        }
        '.' => {
          map.insert(pos, Open);
        }
        _ => {}
      }
    }
  }

  map
}

fn steps_aa_zz(map: &Map) -> u64 {
  // find start
  let start = map
    .iter()
    .filter(|(_, tile)| tile.is_start_portal())
    .flat_map(|(pos, _)| {
      Direction::iter_all()
        .map(|dir| dir.move_position(&pos))
        .collect::<Vec<Position>>()
    })
    .find(|pos| map.get(&pos).map_or(false, |tile| *tile == Open))
    .unwrap();

  let mut steps = 0;
  let mut search = HashSet::new();
  search.insert(start);
  let mut visited = HashSet::new();

  loop {
    let mut at_exit = false;

    let mut new_search = HashSet::new();
    for pos in search {
      visited.insert(pos);
      // let tile = map.get(&pos).unwrap().clone();

      Direction::iter_all()
        .map(|dir| dir.move_position(&pos))
        .filter(|p| map.contains_key(&p) && !visited.contains(&p))
        .for_each(|p| {
          if *map.get(&p).unwrap() == Portal('Z', 'Z') {
            at_exit = true;
          }
          new_search.insert(p);
        });
    }

    if at_exit {
      break;
    }

    // map portals to other end, & mark visited
    search = new_search
      .into_iter()
      .map(|pos| {
        let tile = map.get(&pos).unwrap().clone();
        if let Portal(_, _) = tile {
          let mut other_end = (0, 0);
          map.iter().for_each(|(p, t)| {
            if *t == tile {
              visited.insert(*p);

              // only consider other portals
              if *p != pos {
                let ampt = Direction::iter_all()
                  .map(|dir| dir.move_position(p))
                  .find(|pp| map.get(&pp).map_or(false, |tt| *tt == Open));
                if let Some(p) = ampt {
                  other_end = p;
                }
              }
            }
          });
          other_end
        } else {
          pos
        }
      })
      .collect();

    steps += 1;
  }

  steps
}

pub fn solution() {
  let input = std::fs::read_to_string("inputs/2019/20.txt").unwrap();
  let map = parse_map(&input);

  println!("Steps AA -> ZZ: {}", steps_aa_zz(&map));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let input = "         A
         A
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z    ";

    let map = parse_map(input);

    assert_eq!(23, steps_aa_zz(&map));
  }

  #[test]
  fn test2() {
    let input = "                   A
                   A
  #################.#############
  #.#...#...................#.#.#
  #.#.#.###.###.###.#########.#.#
  #.#.#.......#...#.....#.#.#...#
  #.#########.###.#####.#.#.###.#
  #.............#.#.....#.......#
  ###.###########.###.#####.#.#.#
  #.....#        A   C    #.#.#.#
  #######        S   P    #####.#
  #.#...#                 #......VT
  #.#.#.#                 #.#####
  #...#.#               YN....#.#
  #.###.#                 #####.#
DI....#.#                 #.....#
  #####.#                 #.###.#
ZZ......#               QG....#..AS
  ###.###                 #######
JO..#.#.#                 #.....#
  #.#.#.#                 ###.#.#
  #...#..DI             BU....#..LF
  #####.#                 #.#####
YN......#               VT..#....QG
  #.###.#                 #.###.#
  #.#...#                 #.....#
  ###.###    J L     J    #.#.###
  #.....#    O F     P    #.#...#
  #.###.#####.#.#####.#####.###.#
  #...#.#.#...#.....#.....#.#...#
  #.#####.###.###.#.#.#########.#
  #...#.#.....#...#.#.#.#.....#.#
  #.###.#####.###.###.#.#.#######
  #.#.........#...#.............#
  #########.###.###.#############
           B   J   C
           U   P   P               ";

    let map = parse_map(input);
    assert_eq!(58, steps_aa_zz(&map));
  }
}
