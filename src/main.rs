mod solutions;

fn main() {
  println!(
    r#"
     _             _
    //\dvent  of  [|)ust
                    `
"#
  );

  let day = std::env::args().skip(1).next().unwrap().parse::<u8>().unwrap();

  solutions::run(day);
}
