use std::fs;

#[derive(Debug, Default)]
struct Passport {
  birth_year: Option<u32>,
  issue_year: Option<u32>,
  exp_year: Option<u32>,
  height: Option<String>,
  hair_color: Option<String>,
  eye_color: Option<String>,
  passport_id: Option<String>,
  country_id: Option<String>,
}

const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

impl Passport {
  fn valid(&self) -> bool {
    self.birth_year.is_some()
      && self.issue_year.is_some()
      && self.exp_year.is_some()
      && self.height.is_some()
      && self.hair_color.is_some()
      && self.eye_color.is_some()
      && self.passport_id.is_some()
  }

  fn valid_data(&self) -> bool {
    self.birth_year.map_or(false, |y| y >= 1920 && y <= 2002)
      && self.issue_year.map_or(false, |y| y >= 2010 && y <= 2020)
      && self.exp_year.map_or(false, |y| y >= 2020 && y <= 2030)
      && self.valid_height()
      && self.valid_hair_color()
      && self.valid_eye_color()
      && self.valid_passport_id()
  }

  fn valid_height(&self) -> bool {
    self.height.as_ref().map_or(false, |height| {
      height
        .trim_end_matches("cm")
        .parse::<u32>()
        .map_or(false, |h| h >= 150 && h <= 193)
        || height
          .trim_end_matches("in")
          .parse::<u32>()
          .map_or(false, |h| h >= 59 && h <= 76)
    })
  }

  fn valid_hair_color(&self) -> bool {
    self.hair_color.as_ref().map_or(false, |hcl| match hcl.split_at(1) {
      ("#", col) => col.chars().all(|c| c.is_ascii_hexdigit()),
      _ => false,
    })
  }

  fn valid_eye_color(&self) -> bool {
    self
      .eye_color
      .as_ref()
      .map_or(false, |e| EYE_COLORS.iter().any(|x| e.eq(x)))
  }

  fn valid_passport_id(&self) -> bool {
    self
      .passport_id
      .as_ref()
      .map_or(false, |pid| pid.len() == 9 && pid.chars().all(|c| c.is_ascii_digit()))
  }

  fn parse_line(&mut self, line: &str) {
    line.trim().split_ascii_whitespace().for_each(|kv| {
      let mut iter = kv.split(":");
      let key = iter.next().unwrap();
      let val = iter.next().unwrap();
      match key {
        "byr" => self.birth_year = Some(val.parse().map_or(0, |y| y)),
        "iyr" => self.issue_year = Some(val.parse().map_or(0, |y| y)),
        "eyr" => self.exp_year = Some(val.parse().map_or(0, |y| y)),
        "hgt" => self.height = Some(val.to_owned()),
        "hcl" => self.hair_color = Some(val.to_owned()),
        "ecl" => self.eye_color = Some(val.to_owned()),
        "pid" => self.passport_id = Some(val.to_owned()),
        "cid" => self.country_id = Some(val.to_owned()),
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

  let (valid_data, _) =
    content
      .lines()
      .chain(vec![""].into_iter())
      .fold((0, Passport::default()), |(count, mut p), line| {
        if line.is_empty() {
          let new_count = if p.valid_data() { count + 1 } else { count };
          (new_count, Passport::default())
        } else {
          p.parse_line(line);
          (count, p)
        }
      });

  println!("Valid passports: {}", valid_passports);
  println!("Valid data: {}", valid_data);
}
