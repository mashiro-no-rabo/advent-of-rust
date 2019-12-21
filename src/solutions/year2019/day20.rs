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

fn is_inner_portal(pos: &Position, height: usize, width: usize) -> bool {
  (pos.0 > 1) && (pos.0 < (width - 2) as i64) && (pos.1 > 1) && (pos.1 < (height - 2) as i64)
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

fn find_start(map: &Map) -> Position {
  map
    .iter()
    .filter(|(_, tile)| tile.is_start_portal())
    .flat_map(|(pos, _)| {
      Direction::iter_all()
        .map(|dir| dir.move_position(&pos))
        .collect::<Vec<Position>>()
    })
    .find(|pos| map.get(&pos).map_or(false, |tile| *tile == Open))
    .unwrap()
}

fn steps_aa_zz(map: &Map) -> u64 {
  // find start
  let start = find_start(map);

  let mut steps = 0;
  let mut search = HashSet::new();
  search.insert(start);
  let mut visited = HashSet::new();

  loop {
    let mut at_exit = false;

    let mut new_search = HashSet::new();
    for pos in search {
      visited.insert(pos);
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

fn step_recursion(map: &Map, raw_input: &str) -> u64 {
  let height = raw_input.trim().lines().count();
  let width = raw_input.lines().map(|l| l.len()).max().unwrap();

  // find start
  let start = find_start(map);

  let mut steps = 0;
  let mut search = HashSet::new();
  search.insert((start, 0));
  let mut visited = HashSet::new();

  loop {
    let mut at_exit = false;

    let mut new_search = HashSet::new();
    for (pos, lvl) in search {
      visited.insert((pos, lvl));
      Direction::iter_all()
        .map(|dir| dir.move_position(&pos))
        .filter(|p| {
          if let Some(Portal('Z', 'Z')) = map.get(p) {
            if lvl == 0 {
              at_exit = true;
            }
          }
          map.contains_key(p) && !visited.contains(&(*p, lvl))
        })
        .for_each(|p| {
          new_search.insert((p, lvl));
        });
    }

    if at_exit {
      break;
    }

    // map portals to other end, & mark visited
    search = new_search
      .into_iter()
      .filter_map(|(pos, lvl)| {
        let tile = map.get(&pos).unwrap().clone();
        let next_lvl = if is_inner_portal(&pos, height, width) {
          lvl + 1
        } else {
          lvl - 1
        };

        if let Portal(_, _) = tile {
          let mut other_end = None;
          map.iter().for_each(|(p, t)| {
            if *t == tile {
              visited.insert((*p, next_lvl));

              // only consider other portals
              if *p != pos {
                let ampt = Direction::iter_all()
                  .map(|dir| dir.move_position(p))
                  .find(|pp| map.get(&pp).map_or(false, |tt| *tt == Open));
                if let Some(p) = ampt {
                  other_end = Some(p);
                }
              }
            }
          });

          if let Some(oe) = other_end {
            Some((oe, next_lvl))
          } else {
            None
          }
        } else {
          Some((pos, lvl))
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
  println!("Outermost AA -> ZZ: {}", step_recursion(&map, &input));
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
    assert_eq!(26, step_recursion(&map, input));
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

  #[test]
  fn test3() {
    let input = "             Z L X W       C
             Z P Q B       K
  ###########.#.#.#.#######.###############
  #...#.......#.#.......#.#.......#.#.#...#
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###
  #.#...#.#.#...#.#.#...#...#...#.#.......#
  #.###.#######.###.###.#.###.###.#.#######
  #...#.......#.#...#...#.............#...#
  #.#########.#######.#.#######.#######.###
  #...#.#    F       R I       Z    #.#.#.#
  #.###.#    D       E C       H    #.#.#.#
  #.#...#                           #...#.#
  #.###.#                           #.###.#
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#
CJ......#                           #.....#
  #######                           #######
  #.#....CK                         #......IC
  #.###.#                           #.###.#
  #.....#                           #...#.#
  ###.###                           #.#.#.#
XF....#.#                         RF..#.#.#
  #####.#                           #######
  #......CJ                       NM..#...#
  ###.#.#                           #.###.#
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#
  #.....#        F   Q       P      #.#.#.#
  ###.###########.###.#######.#########.###
  #.....#...#.....#.......#...#.....#.#...#
  #####.#.###.#######.#######.###.###.#.#.#
  #.......#.......#.#.#.#.#...#...#...#.#.#
  #####.###.#####.#.#.#.#.###.###.#.###.###
  #.......#.....#.#...#...............#...#
  #############.#.#.###.###################
               A O F   N
               A A D   M                     ";

    let map = parse_map(input);
    assert_eq!(396, step_recursion(&map, input));
  }
}
