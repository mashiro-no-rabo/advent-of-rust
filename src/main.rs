mod solutions;

fn main() {
  println!(
    r#"
     _             _
    //\dvent  of  [|)ust
                    `
"#
  );

  let day = std::env::args()
    .skip(1)
    .next()
    .expect("need day")
    .parse::<u8>()
    .expect("day is not number");

  solutions::run(day);
}
