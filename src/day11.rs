use std::fmt;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct Octopus(u8);

impl Octopus {
    fn increase_energy_level(&mut self) -> bool {
        self.0 += 1;
        self.0 == 10
    }

    fn reset_flashes(&mut self) -> bool {
        if self.0 >= 10 {
            self.0 = 0;
            true
        } else {
            false
        }
    }

    fn energy_level(&self) -> u8 {
        if self.0 >= 10 {
            0
        } else {
            self.0
        }
    }
}

struct Grid<const SIZE: usize> {
    grid: Vec<Octopus>,
    amount_of_flashes: usize,
}

impl<const SIZE: usize> fmt::Debug for Grid<SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..SIZE {
            for x in 0..SIZE {
                write!(f, "{}", self.get_at(x, y).energy_level())?;
            }
            if y < SIZE - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl<const SIZE: usize> Grid<SIZE> {
    fn new(input: &str) -> Self {
        Grid {
            grid: input.bytes().map(|b| Octopus(b & 0b1111)).collect(),
            amount_of_flashes: 0,
        }
    }

    fn get_at(&self, x: usize, y: usize) -> &Octopus {
        &self.grid[y * (SIZE + 1) + x]
    }

    fn get_mut_at(&mut self, x: usize, y: usize) -> &mut Octopus {
        &mut self.grid[y * (SIZE + 1) + x]
    }

    fn increase_energy_level_and_maybe_flash(&mut self, x: usize, y: usize) {
        let octopus = self.get_mut_at(x, y);
        let flashes = octopus.increase_energy_level();
        if flashes {
            self.amount_of_flashes += 1;
            let min_x = x.saturating_sub(1);
            let max_x = (x + 1).min(SIZE - 1);
            let min_y = y.saturating_sub(1);
            let max_y = (y + 1).min(SIZE - 1);
            for other_x in min_x..=max_x {
                for other_y in min_y..=max_y {
                    if (other_x, other_y) != (x, y) {
                        self.increase_energy_level_and_maybe_flash(other_x, other_y);
                    }
                }
            }
        }
    }

    fn update(&mut self) -> bool {
        for y in 0..SIZE {
            for x in 0..SIZE {
                self.increase_energy_level_and_maybe_flash(x, y);
            }
        }
        let mut all_flashes = true;
        for y in 0..SIZE {
            for x in 0..SIZE {
                all_flashes = self.get_mut_at(x, y).reset_flashes() && all_flashes;
            }
        }
        all_flashes
    }
}

pub fn part_1(input: &str) -> usize {
    let mut grid = Grid::<10>::new(input);
    for _ in 0..100 {
        grid.update();
    }
    grid.amount_of_flashes
}

pub fn part_2(input: &str) -> usize {
    let mut grid = Grid::<10>::new(input);
    (0..).find(|_| grid.update()).unwrap() + 1
}

#[test]
fn test_part_1_example() {
    let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    assert_eq!(part_1(input), 1656);
}

#[test]
fn test_part_2_example() {
    let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    assert_eq!(part_2(input), 195);
}

#[test]
fn test_part_1_small_example() {
    let input = "11111
19991
19191
19991
11111";
    let mut grid = Grid::<5>::new(input);
    assert_eq!(format!("{:?}", grid), input);
    grid.update();
    assert_eq!(
        format!("{:?}", grid),
        "34543
40004
50005
40004
34543"
    );
    grid.update();
    assert_eq!(
        format!("{:?}", grid),
        "45654
51115
61116
51115
45654"
    );
}
