use std::{collections::{BinaryHeap, HashMap}, cmp::Ordering};

struct Grid<'a, const LINE_SIZE: usize>(&'a [u8]);

impl<'a, const LINE_SIZE: usize> Grid<'a, LINE_SIZE> {
    fn new(input: &'a str) -> Self {
        debug_assert!(input.as_bytes()[LINE_SIZE] == b'\n');
        Grid(input.as_bytes())
    }

    fn cost(&self, index: usize) -> usize {
        usize::from(self.0[index] & 0b1111)
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct Index<const LINE_SIZE: usize> {
    x: usize,
    y: usize
}

impl<const LINE_SIZE: usize> Index<LINE_SIZE> {
    fn cost_guess(&self) -> usize {
        self.x + self.y
    }

    fn array_index(&self) -> usize {
        self.x + LINE_SIZE * self.y
    }

    fn neighbour_array_indices(&self, len: usize) -> impl Iterator<Item = usize> {
        let index = self.array_index();
        // Left
        (if self.x > 0 {
            Some(index - 1)
        } else {
            None
        }).into_iter()
        // Right
        .chain(if self.x + 1 < LINE_SIZE - 1 {
            Some(index + 1)
        } else {
            None
        }.into_iter())
        // Up
        .chain(if index > LINE_SIZE {
            Some(index - LINE_SIZE)
        } else {
            None
        }.into_iter())
        // Down
        .chain(if index + LINE_SIZE < len {
            Some(index + LINE_SIZE)
        } else {
            None
        }.into_iter())
    }
}

impl<const LINE_SIZE: usize> From<usize> for Index<LINE_SIZE> {
    fn from(u: usize) -> Index<LINE_SIZE> {
        Index {
            x: u % LINE_SIZE,
            y: u / LINE_SIZE
        }
    }
}

impl<const LINE_SIZE: usize> PartialOrd for Index<LINE_SIZE> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const LINE_SIZE: usize> Ord for Index<LINE_SIZE> {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost_guess()).cmp(&self.cost_guess())
    }
}

pub fn part_1(input: &str) -> usize {
    part_1_generic::<101>(input)
}

pub fn part_1_generic<const LINE_SIZE: usize>(input: &str) -> usize {
    let grid = Grid::<LINE_SIZE>::new(input);
    let mut open_set: BinaryHeap<Index<LINE_SIZE>> = BinaryHeap::new();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();
    let start: Index<LINE_SIZE> = Index { x: 0, y: 0 };
    let end_index = input.len() - 1;
    open_set.push(start);
    g_score.insert(start.array_index(), 0);
    f_score.insert(start.array_index(), 0);
    while let Some(index) = open_set.pop() {
        let current_score = *g_score.get(&index.array_index()).unwrap();
        println!("{},{}: {}", index.x, index.y, current_score);
        if index.array_index() == end_index {
            return current_score;
        }
        for neighbour in index.neighbour_array_indices(input.len()) {
            let tentative_score = current_score + grid.cost(neighbour);
            if tentative_score < *g_score.get(&neighbour).unwrap_or(&usize::MAX) {
                g_score.insert(neighbour, tentative_score);
                f_score.insert(neighbour, tentative_score + 1);
                open_set.push(neighbour.into());
            }
        }
    }
    unreachable!();
}

#[test]
fn test_part_1_example() {
    let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    assert_eq!(part_1_generic::<11>(input), 40);
}
