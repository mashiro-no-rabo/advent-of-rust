use itertools::Itertools;
use std::fs;

type Pair = Vec<PairPart>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PairPart {
  PairStart,
  RegularNumber(u32),
  PairEnd,
}
use PairPart::*;

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/18.txt").unwrap();

  let final_pair = content.lines().map(parse).reduce(pair_add).unwrap();

  println!("magnitude: {}", pair_mag(&final_pair));

  let max_mag = content
    .lines()
    .map(parse)
    .permutations(2)
    .map(|mut p| pair_mag(&pair_add(p.pop().unwrap(), p.pop().unwrap())))
    .max()
    .unwrap();

  println!("max mag: {}", max_mag);
}

fn parse(l: &str) -> Pair {
  l.chars().fold(Pair::new(), |mut acc, c| {
    // no input is > 9
    match c {
      '[' => acc.push(PairStart),
      ']' => acc.push(PairEnd),
      ',' => {}
      _ => acc.push(RegularNumber(c.to_digit(10).unwrap())),
    }
    acc
  })
}

fn pair_add(p1: Pair, p2: Pair) -> Pair {
  let mut pair_added = Pair::new();
  pair_added.push(PairStart);
  pair_added.extend(p1.into_iter());
  pair_added.extend(p2.into_iter());
  pair_added.push(PairEnd);

  pair_reduce(pair_added)
}

fn pair_reduce(mut pair: Pair) -> Pair {
  loop {
    let mut nested = 0;

    // always explode first
    if pair
      .iter()
      .enumerate()
      .find_map(|(idx, pp)| {
        match pp {
          PairStart => nested += 1,
          PairEnd => nested -= 1,
          RegularNumber(_) => {
            if nested > 4 {
              return Some(idx);
            }
          }
        }

        None
      })
      .and_then(|idx| {
        // explode
        let left = match pair.remove(idx) {
          RegularNumber(x) => x,
          _ => unimplemented!("must be a regular number"),
        };
        let right = match pair.remove(idx) {
          RegularNumber(x) => x,
          _ => unimplemented!("must be a regular number"),
        };

        let mut l = idx;
        while l >= 1 {
          l -= 1;
          if let Some(RegularNumber(x)) = pair.get_mut(l) {
            *x += left;
            break;
          }
        }

        for r in (idx + 1)..(pair.len() - 1) {
          if let Some(RegularNumber(x)) = pair.get_mut(r) {
            *x += right;
            break;
          }
        }

        pair.remove(idx); // end
        pair.insert(idx, RegularNumber(0));
        pair.remove(idx - 1); // start

        Some(())
      })
      .or_else(|| {
        // no explode, find split
        pair
          .iter()
          .enumerate()
          .find_map(|(idx, pp)| {
            match pp {
              RegularNumber(x) => {
                if *x > 9 {
                  return Some(idx);
                }
              }
              _ => {}
            }

            None
          })
          .and_then(|idx| {
            // split
            let x = match pair.remove(idx) {
              RegularNumber(x) => x,
              _ => unimplemented!("must be a regular number"),
            };
            pair.insert(idx, PairEnd);
            pair.insert(idx, RegularNumber(x / 2 + x % 2));
            pair.insert(idx, RegularNumber(x / 2));
            pair.insert(idx, PairStart);
            Some(())
          })
      })
      .is_none()
    {
      // no action
      break;
    }
  }

  pair
}

fn pair_mag(pair: &Pair) -> u32 {
  pair
    .iter()
    .fold(Vec::new(), |mut acc, pp| {
      match pp {
        RegularNumber(x) => acc.push(*x),
        PairEnd => {
          let right = acc.pop().unwrap();
          let left = acc.pop().unwrap();
          acc.push(left * 3 + right * 2);
        }
        PairStart => {}
      }

      acc
    })
    .pop()
    .unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_examples() {
    assert_eq!(pair_reduce(parse("[[[[[9,8],1],2],3],4]")), parse("[[[[0,9],2],3],4]"));
    assert_eq!(pair_reduce(parse("[7,[6,[5,[4,[3,2]]]]]")), parse("[7,[6,[5,[7,0]]]]"));
    assert_eq!(pair_reduce(parse("[[6,[5,[4,[3,2]]]],1]")), parse("[[6,[5,[7,0]]],3]"));

    assert_eq!(
      pair_add(parse("[[[[4,3],4],4],[7,[[8,4],9]]]"), parse("[1,1]")),
      parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    );

    assert_eq!(pair_mag(&parse("[[1,2],[[3,4],5]]")), 143);
  }

  #[test]
  fn test_steps() {
    assert_eq!(
      pair_add(
        parse("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"),
        parse("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]")
      ),
      parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")
    );
  }
}
