use std::fs;

#[derive(Debug, Default)]
struct Passport {
  birth_year: Option<String>,
  issue_year: Option<String>,
  exp_year: Option<String>,
  height: Option<String>,
  hair_color: Option<String>,
  eye_color: Option<String>,
  passport_id: Option<String>,
  country_id: Option<String>,
}

impl Passport {
  fn valid(&self) -> bool {
    self.birth_year.is_some()
      & self.issue_year.is_some()
      & self.exp_year.is_some()
      & self.height.is_some()
      & self.hair_color.is_some()
      & self.eye_color.is_some()
      & self.passport_id.is_some()
  }

  fn parse_line(&mut self, line: &str) {
    line.trim().split_ascii_whitespace().for_each(|kv| {
      let mut iter = kv.split(":");
      let key = iter.next().unwrap();
      let val = iter.next().unwrap().to_owned();
      match key {
        "byr" => self.birth_year = Some(val),
        "iyr" => self.issue_year = Some(val),
        "eyr" => self.exp_year = Some(val),
        "hgt" => self.height = Some(val),
        "hcl" => self.hair_color = Some(val),
        "ecl" => self.eye_color = Some(val),
        "pid" => self.passport_id = Some(val),
        "cid" => self.country_id = Some(val),
        _ => {}
      }
    })
  }
}

pub fn solution() {
  let content = fs::read_to_string("inputs/2020/4.txt").unwrap();

  let (valid_passports, _) =
    content
      .lines()
      .chain(vec![""].into_iter())
      .fold((0, Passport::default()), |(count, mut p), line| {
        if line.is_empty() {
          let new_count = if p.valid() { count + 1 } else { count };
          (new_count, Passport::default())
        } else {
          p.parse_line(line);
          (count, p)
        }
      });

  println!("Valid passports: {}", valid_passports);
}
