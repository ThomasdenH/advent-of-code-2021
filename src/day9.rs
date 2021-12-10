use std::{
    cmp::Ordering,
    fmt, iter,
    ops::{Index, IndexMut},
};

use itertools::Itertools;

pub fn part_1_generic<const LINE_SIZE: usize>(input: &str) -> usize {
    debug_assert_eq!(input.find('\n'), Some(LINE_SIZE - 1));
    iter::once(b'\n')
        .chain(input.bytes())
        .chain(iter::once(b'\n'))
        .map(|b| b ^ b'0')
        .tuple_windows()
        .map(|(a, b)| (a.cmp(&b), b))
        .tuple_windows()
        .map(|((cmp1, val1), (cmp2, _))| (cmp1, cmp2, val1))
        .enumerate()
        .filter(|(index, (cmp1, cmp2, value))| {
            *cmp1 == Ordering::Greater
                && *cmp2 == Ordering::Less
                && (*index < LINE_SIZE || *value < b'0' ^ input.as_bytes()[*index - LINE_SIZE])
                && (*index >= input.len() - LINE_SIZE
                    || *value < b'0' ^ input.as_bytes()[*index + LINE_SIZE])
        })
        .map(|(_index, (_cmp1, _cmp2, value))| usize::from(value + 1))
        .sum()
}

pub fn part_1(input: &str) -> usize {
    const LINE_SIZE: usize = 101;
    part_1_generic::<LINE_SIZE>(input)
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct MaybeBasin(usize);

impl fmt::Debug for MaybeBasin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_a_basin() {
            write!(f, "{}", self.0)
        } else {
            write!(f, "_")
        }
    }
}

impl MaybeBasin {
    fn is_no_basin(&self) -> bool {
        self.0 == usize::MAX
    }

    fn no_basin() -> Self {
        MaybeBasin(usize::MAX)
    }

    fn is_a_basin(&self) -> bool {
        !self.is_no_basin()
    }

    fn increment(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }
}

impl<T> Index<MaybeBasin> for Vec<T> {
    type Output = T;
    fn index(&self, index: MaybeBasin) -> &Self::Output {
        debug_assert!(index.is_a_basin());
        &self[index.0]
    }
}

impl<T> IndexMut<MaybeBasin> for Vec<T> {
    fn index_mut(&mut self, index: MaybeBasin) -> &mut Self::Output {
        debug_assert!(index.is_a_basin());
        &mut self[index.0]
    }
}

fn part_2_generic<const LINE_SIZE: usize>(input: &str) -> usize {
    debug_assert_eq!(input.find('\n'), Some(LINE_SIZE - 1));

    // Really need to comput size of areas != 9.

    // Stores the size of a basin by their index
    let mut basin_size = Vec::new();
    // The next basin index to use
    let mut next_basin = MaybeBasin(0);
    // Basins already associated with a tile
    let mut associated_basins = [MaybeBasin::no_basin(); LINE_SIZE];
    for (index, b) in input.bytes().enumerate() {
        let x = index % LINE_SIZE;
        match b {
            b'9' | b'\n' => {
                // No basin
                associated_basins[x] = MaybeBasin::no_basin();
            }
            _ => {
                let basin_above = associated_basins[x];
                let basin_to_the_left = if x >= 1 {
                    associated_basins[x - 1]
                } else {
                    MaybeBasin::no_basin()
                };
                if basin_to_the_left.is_a_basin() {
                    if basin_above.is_no_basin() || basin_above == basin_to_the_left {
                        associated_basins[x] = basin_to_the_left;
                        basin_size[basin_to_the_left] += 1;
                    } else {
                        for previous in (0..x).rev() {
                            if associated_basins[previous] != basin_to_the_left {
                                break;
                            }
                            associated_basins[previous] = basin_above;
                        }
                        basin_size[basin_above] += basin_size[basin_to_the_left] + 1;
                    }
                } else if basin_above.is_no_basin() {
                    assert_eq!(basin_size.len(), next_basin.0); // For some reason actually using this is not faster
                    basin_size.push(1);
                    associated_basins[x] = next_basin;
                    next_basin.increment();
                } else {
                    associated_basins[x] = basin_above;
                    basin_size[basin_above] += 1;
                };
            }
        }
    }
    let size = basin_size.len();
    basin_size
        .select_nth_unstable(size.saturating_sub(4))
        .2
        .iter()
        .copied()
        .product()
}

pub fn part_2(input: &str) -> usize {
    const LINE_SIZE: usize = 101;
    part_2_generic::<LINE_SIZE>(input)
}

#[test]
fn test_part_1_example() {
    let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
    assert_eq!(part_1_generic::<11>(input), 15);
}

#[test]
fn test_part_1_input() {
    let input = include_str!("../input/2021/day9.txt");
    assert_eq!(part_1(input), 530);
}

#[test]
fn test_part_2_example() {
    let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
    assert_eq!(part_2_generic::<11>(input), 1134);
}

#[test]
fn test_part_2_input() {
    let input = include_str!("../input/2021/day9.txt");
    assert_eq!(part_2(input), 1019494);
}
