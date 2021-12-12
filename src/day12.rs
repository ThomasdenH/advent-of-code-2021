use std::collections::HashMap;

struct Graph<'a> {
    nodes: HashMap<&'a str, Node<'a>>,
}

struct Node<'a> {
    neighbours: Vec<&'a str>,
    allowed_visits: u8,
}

impl<'a> Graph<'a> {
    fn add_connection_one_way(&mut self, name: &'a str, name_2: &'a str, max_visits: u8) {
        let is_small = name.chars().all(|c| c.is_ascii_lowercase());
        let allowed_visits = if is_small { max_visits } else { u8::MAX };
        self
            .nodes
            .entry(name)
            .or_insert_with(|| Node {
                neighbours: Vec::new(),
                allowed_visits
            })
            .neighbours
            .push(name_2);
    }

    fn new(input: &'a str, max_visits: u8) -> Self {
        let mut graph = Graph {
            nodes: HashMap::new(),
        };
        for line in input.lines() {
            let mut split = line.split('-');
            let node_a_name = split.next().unwrap();
            let node_b_name = split.next().unwrap().trim_end();
            graph.add_connection_one_way(node_a_name, node_b_name, max_visits);
            if node_b_name != "end" && node_a_name != "start" {
                graph.add_connection_one_way(node_b_name, node_a_name, max_visits);
            }
        }
        graph
    }

    fn can_visit(&self, node: &str) -> bool {
        self.nodes.get(node).unwrap().allowed_visits > 0
    }

    fn paths_to_end(&mut self, node_str: &str, paths: usize) -> usize {
        dbg!(node_str, paths);
        let node = self.nodes.get_mut(node_str).unwrap();
        node.allowed_visits -= 1;
        let paths = node
            .neighbours
            .clone()
            .iter()
            .filter_map(|node| {
                if *node == "end" {
                    Some(1)
                } else if self.can_visit(node) {
                    Some(self.paths_to_end(node, paths))
                } else {
                    None
                }
            })
            .sum();
        self.nodes.get_mut(node_str).unwrap().allowed_visits += 1;
        paths
    }

    fn paths_from_start_to_end(&mut self) -> usize {
        self.paths_to_end("start", 1)
    }
}

pub fn part_1(input: &str) -> usize {
    Graph::new(input, 1).paths_from_start_to_end()
}

pub fn part_2(input: &str) -> usize {
    Graph::new(input, 2).paths_from_start_to_end()
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