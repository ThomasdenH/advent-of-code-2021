use std::collections::HashMap;

struct Graph<'a> {
    nodes: HashMap<&'a str, Node<'a>>,
    visit_rule: VisitRule
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum VisitRule {
    Single,
    DoubleOnce
}

struct Node<'a> {
    neighbours: Vec<&'a str>,
    visited: bool,
    is_small: bool
}

impl<'a> Graph<'a> {
    fn add_connection_one_way(&mut self, name: &'a str, name_2: &'a str) {
        let is_small = name.chars().all(|c| c.is_ascii_lowercase());
        self
            .nodes
            .entry(name)
            .or_insert_with(|| Node {
                neighbours: Vec::new(),
                visited: false,
                is_small
            })
            .neighbours
            .push(name_2);
    }

    fn new(input: &'a str, visit_rule: VisitRule) -> Self {
        let mut graph = Graph {
            nodes: HashMap::new(),
            visit_rule
        };
        for line in input.lines() {
            let mut split = line.split('-');
            let node_a_name = split.next().unwrap();
            let node_b_name = split.next().unwrap().trim_end();
            graph.add_connection_one_way(node_a_name, node_b_name);
            if node_b_name != "end" && node_a_name != "start" {
                graph.add_connection_one_way(node_b_name, node_a_name);
            }
        }
        graph
    }

    fn is_big_or_unvisited(&self, node: &str) -> bool {
        let node = self.nodes.get(node).unwrap();
        !node.is_small || !node.visited
    }

    fn paths_to_end(&mut self, node_str: &'a str, paths: usize, can_still_visit_twice: &mut bool) -> usize {
        let node = self.nodes.get_mut(node_str).unwrap();
        let used_extra_visit = node.is_small && node.visited;
        if used_extra_visit {
            debug_assert!(self.visit_rule == VisitRule::DoubleOnce);
            debug_assert!(node.visited);
            debug_assert!(!node.is_small || *can_still_visit_twice == true);
            *can_still_visit_twice = false;
        } else {
            debug_assert!(!node.is_small || !node.visited);
            node.visited = true;
        }
        let paths = node
            .neighbours
            .clone()
            .iter()
            .filter_map(|other_node| {
                if *other_node == "end" {
                    Some(1)
                } else if self.is_big_or_unvisited(other_node) || (self.visit_rule == VisitRule::DoubleOnce && *can_still_visit_twice) {
                    Some(self.paths_to_end(other_node, paths, can_still_visit_twice))
                } else {
                    None
                }
            })
            .sum();
        if used_extra_visit {
            *can_still_visit_twice = true;
        } else {
            self.nodes.get_mut(node_str).unwrap().visited = false;
        }
        paths
    }

    fn paths_from_start_to_end(&mut self) -> usize {
        let mut can_still_visit_twice = if self.visit_rule == VisitRule::DoubleOnce { true } else { false };
        self.paths_to_end("start", 1, &mut can_still_visit_twice)
    }
}

pub fn part_1(input: &str) -> usize {
    Graph::new(input, VisitRule::Single).paths_from_start_to_end()
}

pub fn part_2(input: &str) -> usize {
    Graph::new(input, VisitRule::DoubleOnce).paths_from_start_to_end()
}

#[test]
fn test_part_1_example_1() {
    let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    assert_eq!(part_1(input), 10);
}

#[test]
fn test_part_1_example_2() {
    let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
    assert_eq!(part_1(input), 19);
}

#[test]
fn test_part_1_example_3() {
    let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
    assert_eq!(part_1(input), 226);
}

#[test]
fn test_part_2_example_1() {
    let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    assert_eq!(part_2(input), 36);
}



#[test]
fn test_part_2_example_2() {
    let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
    assert_eq!(part_2(input), 103);
}

#[test]
fn test_part_2_example_3() {
    let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
    assert_eq!(part_2(input), 3509);
}
