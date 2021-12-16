use std::fs;

struct Packet {
  version: u32,
  typ: u8,
  data: PacketData,
}

enum PacketData {
  Literal(u64),
  SubPackets(Vec<Packet>),
}

impl Packet {
  fn from_hexdecimal(hd: &str) -> Self {
    let mut binary = String::new();
    for i in 0..hd.trim().len() {
      binary.push_str(&format!("{:04b}", u8::from_str_radix(&hd[i..=i], 16).unwrap()));
    }
    Packet::consume_str(&binary).1
  }

  fn consume_str(s: &str) -> (usize, Self) {
    let mut consumed = 0;

    let (vs, rest) = s.split_at(3);
    consumed += 3;
    let version = u32::from_str_radix(vs, 2).unwrap();

    let (ts, mut rest) = rest.split_at(3);
    consumed += 3;
    let typ = u8::from_str_radix(ts, 2).unwrap();

    let data = match typ {
      4 => {
        let mut s = String::new();
        loop {
          consumed += 5;
          let (bs, r) = rest.split_at(5);
          s.push_str(&bs[1..5]);
          rest = r;
          if bs.starts_with('0') {
            break;
          }
        }

        PacketData::Literal(u64::from_str_radix(&s, 2).unwrap())
      }
      _ => {
        let (len_type, rest) = rest.split_at(1);
        consumed += 1;
        match len_type {
          "0" => {
            let (ls, mut rest) = rest.split_at(15);
            consumed += 15;
            let mut len_left = usize::from_str_radix(ls, 2).unwrap();

            let mut sub_packets = Vec::new();
            while len_left > 0 {
              let (c, packet) = Packet::consume_str(rest);
              let (_, r) = rest.split_at(c);
              consumed += c;
              sub_packets.push(packet);
              len_left -= c;
              rest = r;
            }

            PacketData::SubPackets(sub_packets)
          }
          "1" => {
            let (cs, mut rest) = rest.split_at(11);
            consumed += 11;
            let count = usize::from_str_radix(cs, 2).unwrap();
            let mut sub_packets = Vec::new();
            for _ in 0..count {
              let (c, packet) = Packet::consume_str(rest);
              let (_, r) = rest.split_at(c);
              consumed += c;
              sub_packets.push(packet);
              rest = r;
            }

            PacketData::SubPackets(sub_packets)
          }
          _ => unimplemented!(),
        }
      }
    };

    (consumed, Self { version, typ, data })
  }

  fn version_sum(&self) -> u32 {
    match self.data {
      PacketData::Literal(_) => self.version,
      PacketData::SubPackets(ref v) => v.iter().map(|p| p.version_sum()).sum::<u32>() + self.version,
    }
  }

  fn calculate(&self) -> u64 {
    match self.data {
      PacketData::Literal(d) => d,
      PacketData::SubPackets(ref sps) => match self.typ {
        0 => sps.iter().map(|p| p.calculate()).sum::<u64>(),
        1 => sps.iter().map(|p| p.calculate()).product::<u64>(),
        2 => sps.iter().map(|p| p.calculate()).min().unwrap(),
        3 => sps.iter().map(|p| p.calculate()).max().unwrap(),
        5 => {
          if sps[0].calculate() > sps[1].calculate() {
            1
          } else {
            0
          }
        }
        6 => {
          if sps[0].calculate() < sps[1].calculate() {
            1
          } else {
            0
          }
        }
        7 => {
          if sps[0].calculate() == sps[1].calculate() {
            1
          } else {
            0
          }
        }
        _ => unimplemented!(),
      },
    }
  }
}

pub fn solution() {
  let content = fs::read_to_string("inputs/2021/16.txt").unwrap();

  let packet = Packet::from_hexdecimal(&content);
  println!("version sum: {}", packet.version_sum());
  println!("calculate: {}", packet.calculate());
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_version_sum() {
    let packet = Packet::from_hexdecimal("8A004A801A8002F478");
    assert_eq!(packet.version_sum(), 16);
    let packet = Packet::from_hexdecimal("620080001611562C8802118E34");
    assert_eq!(packet.version_sum(), 12);
    let packet = Packet::from_hexdecimal("C0015000016115A2E0802F182340");
    assert_eq!(packet.version_sum(), 23);
    let packet = Packet::from_hexdecimal("A0016C880162017C3686B18A3D4780");
    assert_eq!(packet.version_sum(), 31);
  }

  #[test]
  fn test_calculate() {
    let packet = Packet::from_hexdecimal("C200B40A82");
    assert_eq!(packet.calculate(), 3);
    let packet = Packet::from_hexdecimal("04005AC33890");
    assert_eq!(packet.calculate(), 54);
    let packet = Packet::from_hexdecimal("880086C3E88112");
    assert_eq!(packet.calculate(), 7);
    let packet = Packet::from_hexdecimal("CE00C43D881120");
    assert_eq!(packet.calculate(), 9);
    let packet = Packet::from_hexdecimal("D8005AC2A8F0");
    assert_eq!(packet.calculate(), 1);
    let packet = Packet::from_hexdecimal("F600BC2D8F");
    assert_eq!(packet.calculate(), 0);
    let packet = Packet::from_hexdecimal("9C005AC2F8F0");
    assert_eq!(packet.calculate(), 0);
    let packet = Packet::from_hexdecimal("9C0141080250320F1802104A08");
    assert_eq!(packet.calculate(), 1);
  }
}
