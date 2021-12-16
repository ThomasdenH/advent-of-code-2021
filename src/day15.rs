use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

struct Grid<'a, const SIZE: usize, const DUPLICATED: usize>(&'a [u8]);

impl<'a, const SIZE: usize, const DUPLICATED: usize> Grid<'a, SIZE, DUPLICATED> {
    fn new(input: &'a str) -> Self {
        debug_assert_eq!(input.as_bytes()[SIZE], b'\n');
        debug_assert_eq!(input.len(), (SIZE + 1) * SIZE - 1);
        Grid(input.as_bytes())
    }

    fn cost_at(&self, index: Index<SIZE, DUPLICATED>) -> usize {
        (usize::from(self.0[index.array_index()] & 0b1111) + index.block_bonus() - 1) % 9 + 1
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
struct Index<const SIZE: usize, const DUPLICATED: usize> {
    x: usize,
    y: usize,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
struct BinaryHeapItem<const SIZE: usize, const DUPLICATED: usize> {
    index: Index<SIZE, DUPLICATED>,
    cost_estimate: usize,
}

impl<const SIZE: usize, const DUPLICATED: usize> Index<SIZE, DUPLICATED> {
    fn from_x_y(x: usize, y: usize) -> Self {
        Index { x, y }
    }

    /// Guess the cost from this point to the goal
    fn cost_guess(&self) -> usize {
        (SIZE * DUPLICATED - 1 - self.x) + (SIZE * DUPLICATED - 1 - self.y)
    }

    fn array_index(&self) -> usize {
        (self.x % SIZE) + (SIZE + 1) * (self.y % SIZE)
    }

    fn block_bonus(&self) -> usize {
        (self.x / SIZE) + (self.y / SIZE)
    }

    fn neighbour_array_indices(&self) -> impl Iterator<Item = Index<SIZE, DUPLICATED>> {
        // Left
        (if self.x > 0 {
            Some(Index {
                x: self.x - 1,
                y: self.y,
            })
        } else {
            None
        })
        .into_iter()
        // Right
        .chain(
            if self.x + 1 < SIZE * DUPLICATED {
                Some(Index {
                    x: self.x + 1,
                    y: self.y,
                })
            } else {
                None
            }
            .into_iter(),
        )
        // Up
        .chain(
            if self.y > 0 {
                Some(Index {
                    x: self.x,
                    y: self.y - 1,
                })
            } else {
                None
            }
            .into_iter(),
        )
        // Down
        .chain(
            if self.y + 1 < SIZE * DUPLICATED {
                Some(Index {
                    x: self.x,
                    y: self.y + 1,
                })
            } else {
                None
            }
            .into_iter(),
        )
    }
}

impl<const SIZE: usize, const DUPLICATED: usize> PartialOrd for BinaryHeapItem<SIZE, DUPLICATED> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const SIZE: usize, const DUPLICATED: usize> Ord for BinaryHeapItem<SIZE, DUPLICATED> {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost_estimate).cmp(&self.cost_estimate)
    }
}

pub fn part_1(input: &str) -> usize {
    parts_generic::<100, 1>(input)
}

pub fn part_2(input: &str) -> usize {
    parts_generic::<100, 5>(input)
}

pub fn parts_generic<const SIZE: usize, const DUPLICATED: usize>(input: &str) -> usize {
    let grid = Grid::<SIZE, DUPLICATED>::new(input);
    let mut open_set = BinaryHeap::new();
    let mut g_score = HashMap::new();
    let start: Index<SIZE, DUPLICATED> = Index::from_x_y(0, 0);
    let end_index = Index::from_x_y(DUPLICATED * SIZE - 1, DUPLICATED * SIZE - 1);
    open_set.push(BinaryHeapItem {
        index: start,
        cost_estimate: start.cost_guess(),
    });
    g_score.insert(start, 0);
    while let Some(BinaryHeapItem { index, .. }) = open_set.pop() {
        let current_score = *g_score.get(&index).unwrap();
        if index == end_index {
            return current_score;
        }
        for neighbour in index.neighbour_array_indices() {
            let tentative_score = current_score + grid.cost_at(neighbour);
            if tentative_score < *g_score.get(&neighbour).unwrap_or(&usize::MAX) {
                g_score.insert(neighbour, tentative_score);
                let cost_estimate = tentative_score + neighbour.cost_guess();
                open_set.push(BinaryHeapItem {
                    index: neighbour,
                    cost_estimate,
                });
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
    assert_eq!(parts_generic::<10, 1>(input), 40);
}

#[test]
fn test_part_2_example() {
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
    assert_eq!(parts_generic::<10, 5>(input), 315);
}

#[test]
fn test_part_1_input() {
    let input = include_str!("../input/2021/day15.txt");
    assert_eq!(part_1(input), 423);
}

#[test]
fn test_part_2_input() {
    let input = include_str!("../input/2021/day15.txt");
    assert_eq!(part_2(input), 2778);
}
