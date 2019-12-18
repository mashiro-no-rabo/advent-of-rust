use std::collections::HashMap;

use super::map::*;

#[derive(Debug, Clone)]
enum Tile {
  Empty,
  Key(char),
}
use Tile::*;

type Distance = Option<u64>;

#[derive(Debug, Clone)]
struct Maze {
  start: Position,
  map: HashMap<Position, Tile>,
  // map: Vec<Position>,
  doors: HashMap<char, Position>,
  keys: HashMap<char, (Position, Distance)>,
}

impl Maze {
  fn from_str(input: &str) -> Self {
    let mut sp = None;
    let mut map = HashMap::new();
    let mut doors = HashMap::new();
    let mut keys = HashMap::new();
    input.trim().lines().enumerate().for_each(|(y, line)| {
      line.chars().enumerate().for_each(|(x, ch)| {
        let pos = (x as i64, y as i64);
        match ch {
          '.' => {
            map.insert(pos, Empty);
          }
          '@' => {
            map.insert(pos, Empty);
            sp = Some(pos);
          }
          'A'..='Z' => {
            doors.insert(ch, pos);
          }
          'a'..='z' => {
            map.insert(pos, Key(ch));
            keys.insert(ch, (pos, None));
          }
          _ => {}
        }
      })
    });

    let start = sp.unwrap();

    let mut m = Self {
      map,
      start,
      doors,
      keys,
    };
    m.update_key_steps();
    m
  }

  fn update_key_steps(&mut self) {
    let mut steps = 0;
    let mut search = vec![self.start];
    let mut visited = vec![];

    while !search.is_empty() {
      steps += 1;
      let mut new_search = vec![];
      for pos in search.into_iter() {
        visited.push(pos);

        Direction::iter_all().for_each(|dir| {
          let t = dir.move_position(&pos);
          if !visited.contains(&t) {
            if let Some(tile) = self.map.get(&t) {
              new_search.push(t);

              if let Key(ch) = tile {
                let mut pos_steps = self.keys.get_mut(&ch).unwrap();
                pos_steps.1 = Some(steps);
              }
            }
          }
        })
      }

      search = new_search;
    }
  }

  fn pick_key(&self, key: char) -> Self {
    let mut new = self.clone();

    // set start
    let (key_pos, _) = new.keys.remove(&key).unwrap();
    new.start = key_pos;
    // clear key's position
    new.map.insert(key_pos, Empty);

    // remove door and add to map
    let door = key.to_ascii_uppercase();
    if let Some(door_pos) = new.doors.remove(&door) {
      new.map.insert(door_pos, Empty);
    }

    // calculate (cache) key steps
    new.update_key_steps();

    new
  }
}

fn dfs_map(mz: &Maze, steps_acc: u64) -> u64 {
  if mz.keys.is_empty() {
    return steps_acc;
  }

  mz.keys
    .iter()
    // filter reachable keys
    .filter(|(_, (_, steps))| steps.is_some())
    .map(|(k, (_, steps))| {
      // pick key
      let sub_maze = mz.pick_key(*k);

      // DFS with door removed (position added to map)
      dfs_map(&sub_maze, steps_acc + steps.unwrap())
    })
    // pick min
    .min()
    .unwrap()
}

pub fn solution() {
  let input = std::fs::read_to_string("inputs/2019/18.txt").unwrap();
  let maze = Maze::from_str(&input);

  println!("Shortest path: {} steps", dfs_map(&maze, 0));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let input = "#########
#b.A.@.a#
#########";

    let mz = Maze::from_str(input);
    assert_eq!(8, dfs_map(&mz, 0));
  }

  #[test]
  fn test2() {
    let input = "########################
  #...............b.C.D.f#
  #.######################
  #.....@.a.B.c.d.A.e.F.g#
  ########################";

    let mz = Maze::from_str(input);
    assert_eq!(132, dfs_map(&mz, 0));
  }

  #[test]
  fn test3() {
    let input = "#################
  #i.G..c...e..H.p#
  ########.########
  #j.A..b...f..D.o#
  ########@########
  #k.E..a...g..B.n#
  ########.########
  #l.F..d...h..C.m#
  #################";

    let mz = Maze::from_str(input);
    assert_eq!(136, dfs_map(&mz, 0));
  }

  #[test]
  fn test4() {
    let input = "########################
  #@..............ac.GI.b#
  ###d#e#f################
  ###A#B#C################
  ###g#h#i################
  ########################";

    let mz = Maze::from_str(input);
    assert_eq!(81, dfs_map(&mz, 0));
  }
}
