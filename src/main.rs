mod solutions;

const THIS_YEAR: u32 = 2019;

fn main() {
  println!(
    r#"
     _             _
    //\dvent  of  [|)ust
                    `
"#
  );

  let mut args = std::env::args().skip(1);

  let arg1 = if let Some(op) = args.next().map(|x| x.parse::<u32>().ok()) {
    op
  } else {
    None
  };
  let arg2 = if let Some(op) = args.next().map(|x| x.parse::<u8>().ok()) {
    op
  } else {
    None
  };

  if arg1.is_none() {
    panic!("Please specify which day to run");
  }

  let (year, day) = if let Some(day) = arg2 {
    // both year and day
    (arg1.unwrap(), day)
  } else {
    check_day(arg1.unwrap());
    (THIS_YEAR, arg1.unwrap() as u8)
  };

  if year > THIS_YEAR {
    panic!("{} is not a valid year (yet)!", year);
  }

  check_day(day as u32);

  solutions::run(year, day);
}

fn check_day(day: u32) {
  if day > 25 {
    panic!("{} is not in advent calendar!", day);
  }
}
