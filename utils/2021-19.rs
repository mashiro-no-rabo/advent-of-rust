fn main() {
  let mut ams = Vec::new();
  let mm = ["X", "Y", "Z", "NegX", "NegY", "NegZ"];
  for x in mm {
    for y in mm {
      for z in mm {
        ams.push((x, y, z));
      }
    }
  }

  ams
    .into_iter()
    .filter(|&(x, y, z)| {
      ([x, y, z].contains(&"NegX") || [x, y, z].contains(&"X"))
        && ([x, y, z].contains(&"NegY") || [x, y, z].contains(&"Y"))
        && ([x, y, z].contains(&"NegZ") || [x, y, z].contains(&"Z"))
        && (!([x, y, z].contains(&"NegX") && [x, y, z].contains(&"X")))
        && (!([x, y, z].contains(&"NegY") && [x, y, z].contains(&"Y")))
        && (!([x, y, z].contains(&"NegZ") && [x, y, z].contains(&"Z")))
    })
    .for_each(|(x, y, z)| println!("(AxisMap::{},AxisMap::{},AxisMap::{}),", x, y, z));
}
