use petgraph::graphmap::DiGraphMap;
use petgraph::visit::Bfs;
use std::collections::HashSet;
use std::fs;

pub fn solution() {
  let content = fs::read_to_string("inputs/2020/7.txt").unwrap();

  let mut graph = DiGraphMap::new();

  content.lines().for_each(|line| {
    let mut parts = line.trim_end_matches(".").split(" contain ");

    // container bag always end with "bags"
    let bag = parts.next().unwrap().trim_end_matches(" bags");
    graph.add_node(bag);

    // parse contents
    let contents = parts.next().unwrap();
    if contents != "no other bags" {
      contents.split(", ").for_each(|content| {
        let idx = content.find(" ").unwrap();
        let (num, cnt) = content.split_at(idx);
        let num = num.parse::<u32>().unwrap();
        // content bag can end with "bag" or "bags"
        let cnt = cnt.trim().trim_end_matches(" bag").trim_end_matches(" bags");
        graph.add_edge(cnt, bag, num);
      })
    }
  });

  {
    let mut sg_bags = HashSet::new();
    let mut sg_bfs = Bfs::new(&graph, "shiny gold");
    while let Some(bag) = sg_bfs.next(&graph) {
      sg_bags.insert(bag);
    }
    println!("Bags for shiny gold: {}", sg_bags.len() - 1);
  }
}
