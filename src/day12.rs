use arrayvec::ArrayVec;

const MAX_NODES: usize = 10;
const MAX_CONNECTIONS_PER_NODE: usize = 10;

struct Graph<'a> {
    nodes: ArrayVec<(&'a [u8], Node<'a>), MAX_NODES>,
    visit_rule: VisitRule,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum VisitRule {
    Single,
    DoubleOnce,
}

#[derive(Debug)]
struct Node<'a> {
    neighbours: ArrayVec<&'a [u8], MAX_CONNECTIONS_PER_NODE>,
    visited: bool,
    is_small: bool,
}

impl<'a> Graph<'a> {
    fn add_connection_one_way(&mut self, name: &'a [u8], name_2: &'a [u8]) {
        if name != b"end" && name_2 != b"start" {
            let is_small = name.iter().all(|c| c.is_ascii_lowercase());
            if let Some(existing_node) = self.get_node_mut(name) {
                existing_node.neighbours.push(name_2);
            } else {
                let mut neighbours = ArrayVec::new();
                neighbours.push(name_2);
                self.nodes.push((
                    name,
                    Node {
                        neighbours,
                        visited: false,
                        is_small,
                    },
                ));
            }
        }
    }

    fn handle_line(&mut self, line: &'a [u8]) {
        let dash_pos = memchr::memchr(b'-', line).unwrap();
        let node_a_name = &line[..dash_pos];
        let node_b_name = &line[(dash_pos + 1)..];
        self.add_connection_one_way(node_a_name, node_b_name);
        self.add_connection_one_way(node_b_name, node_a_name);
    }

    fn new(input: &'a str, visit_rule: VisitRule) -> Self {
        let mut graph = Graph {
            nodes: ArrayVec::new(),
            visit_rule,
        };
        let mut input = input.as_bytes();
        while let Some(line_end_pos) = memchr::memchr(b'\n', input) {
            let line = &input[..line_end_pos];
            input = &input[line_end_pos + 1..];
            graph.handle_line(line);
        }
        graph.handle_line(input);
        graph
    }

    fn get_node(&self, s: &[u8]) -> Option<&Node<'a>> {
        self.nodes.iter().find(|(name, _)| *name == s).map(|o| &o.1)
    }

    fn get_node_mut(&mut self, s: &[u8]) -> Option<&mut Node<'a>> {
        self.nodes
            .iter_mut()
            .find(|(name, _)| *name == s)
            .map(|o| &mut o.1)
    }

    fn is_big_or_unvisited(&self, node: &[u8]) -> bool {
        let node = self.get_node(node).unwrap();
        !node.is_small || !node.visited
    }

    fn paths_to_end(&mut self, node_str: &'a [u8], can_still_visit_twice: &mut bool) -> usize {
        let node = self
            .nodes
            .iter_mut()
            .find(|(name, _)| *name == node_str)
            .map(|o| &mut o.1)
            .unwrap();
        let used_extra_visit = node.is_small && node.visited;
        if used_extra_visit {
            debug_assert!(self.visit_rule == VisitRule::DoubleOnce);
            debug_assert!(node.visited);
            debug_assert!(!node.is_small || *can_still_visit_twice);
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
                if *other_node == b"end" {
                    Some(1)
                } else if self.is_big_or_unvisited(other_node)
                    || (self.visit_rule == VisitRule::DoubleOnce && *can_still_visit_twice)
                {
                    Some(self.paths_to_end(other_node, can_still_visit_twice))
                } else {
                    None
                }
            })
            .sum();
        if used_extra_visit {
            *can_still_visit_twice = true;
        } else {
            self.get_node_mut(node_str).unwrap().visited = false;
        }
        paths
    }

    fn paths_from_start_to_end(&mut self) -> usize {
        let mut can_still_visit_twice = self.visit_rule == VisitRule::DoubleOnce;
        self.paths_to_end(b"start", &mut can_still_visit_twice)
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

#[test]
fn test_part_1_input() {
    let input = include_str!("../input/2021/day12.txt");
    assert_eq!(part_1(input), 5157);
}

#[test]
fn test_part_2_input() {
    let input = include_str!("../input/2021/day12.txt");
    assert_eq!(part_2(input), 144309);
}
