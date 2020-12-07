use petgraph::graphmap::DiGraphMap;
use petgraph::visit::{Bfs, Walker};
use std::fs;

pub fn solution() {
  let content = fs::read_to_string("inputs/2020/7.txt").unwrap();

  let mut graph = DiGraphMap::new();
  let mut graph2 = DiGraphMap::new();

  content.lines().for_each(|line| {
    let mut parts = line.trim_end_matches(".").split(" contain ");

    // container bag always end with "bags"
    let bag = parts.next().unwrap().trim_end_matches(" bags");
    graph.add_node(bag);
    graph2.add_node(bag);

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
        graph2.add_edge(bag, cnt, num);
      })
    }
  });

  {
    // part 1
    let sg_bfs = Bfs::new(&graph, "shiny gold");
    println!("Bags for shiny gold: {}", sg_bfs.iter(&graph).count() - 1);
  }

  {
    // part 2
    // sum of each child: num * (1 + calc(child))

    println!("Bags to buy: {}", buy_bags(&graph2, "shiny gold"));
  }
}

fn buy_bags(graph: &DiGraphMap<&str, u32>, bag: &str) -> u32 {
  let mut count = 0;
  for (_, inner, weight) in graph.edges(bag) {
    count += weight * (1 + buy_bags(graph, inner))
  }

  count
}
