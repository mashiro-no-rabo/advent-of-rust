fn number_to_vec(n: i64) -> Vec<i64> {
  let mut digits = Vec::new();
  let mut n = n;
  while n > 9 {
    digits.push(n % 10);
    n /= 10;
  }
  digits.push(n);
  digits.reverse();
  digits
}

fn valid_password(num: i64) -> bool {
  let nv = number_to_vec(num);
  let mut good_repeat = false;
  let mut repeated_count = 1;
  let mut cur_repeated = nv[0];
  for idx in 1..6 {
    if nv[idx] < nv[idx - 1] {
      return false;
    }
    if nv[idx] == cur_repeated {
      repeated_count += 1;
    } else {
      cur_repeated = nv[idx];
      if repeated_count == 2 {
        good_repeat = true
      }
      repeated_count = 1;
    }
  }
  (repeated_count == 2) || good_repeat
}

pub fn solution() {
  let mut count = 0;

  for num in 109_165..=576_723 {
    if valid_password(num) {
      count += 1;
    }
  }

  println!("Answer 1: {}", count)
}
