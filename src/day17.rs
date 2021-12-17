use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res, opt},
    sequence::{preceded, separated_pair, pair},
};
use std::ops::RangeInclusive;
use std::str::FromStr;

struct Simulation {
    dy: i32,
    dx: u16,
    x: u16,
    y: i32,
}

impl Simulation {
    fn new_from_velocity(dx: u16, dy: i32) -> Simulation {
        Simulation { dx, dy, x: 0, y: 0 }
    }

    fn into_iter(mut self) -> impl Iterator<Item = (u16, i32)> {
        std::iter::from_fn(move || {
            self.x += self.dx;
            self.y += self.dy;
            self.dx = self.dx.saturating_sub(1);
            self.dy -= 1;
            Some((self.x, self.y))
        })
    }
}

fn parse(input: &str) -> (RangeInclusive<u16>, RangeInclusive<i32>) {
    preceded::<_, _, _, nom::error::Error<_>, _, _>(
        tag("target area: x="),
        separated_pair(
            map(
                separated_pair(
                    map_res(digit1, u16::from_str),
                    tag(".."),
                    map_res(digit1, u16::from_str),
                ),
                |(a, b): (u16, u16)| a..=b,
            ),
            tag(", y="),
            map(
                separated_pair(
                    map(pair(opt(tag("-")), digit1), |(minus, num): (Option<&str>, &str)| i32::from_str(num).unwrap() * if minus.is_some() { -1 } else { 1 }),
                    tag(".."),
                    map(pair(opt(tag("-")), digit1), |(minus, num): (Option<&str>, &str)| i32::from_str(num).unwrap() * if minus.is_some() { -1 } else { 1 }),
                ),
                |(a, b): (i32, i32)| a..=b,
            ),
        ),
    )(input)
    .unwrap()
    .1
}

pub fn part_1(input: &str) -> i32 {
    let (x_range, y_range) = &parse(input);
    // dx should be positive and it can't overshoot in the first step.
    let mut max_y = i32::MIN;
    for dx in 0..=*x_range.end() {
        for dy in *y_range.start()..=-*y_range.start() {
            let mut current_max_y = i32::MIN;
            let mut reached_target = false;
            for (x, y) in Simulation::new_from_velocity(dx, dy).into_iter() {
                if y < *y_range.start() || x > *x_range.end() {
                    break;
                } else if x_range.contains(&x) && y_range.contains(&y) {
                    reached_target = true;
                }
                current_max_y = current_max_y.max(y);
                
            }
            if reached_target {
                max_y = max_y.max(current_max_y);
            }
        }
    }
    max_y
}


pub fn part_2(input: &str) -> usize {
    let mut count = 0;
    let (x_range, y_range) = &parse(input);
    // dx should be positive and it can't overshoot in the first step.
    for dx in 0..=*x_range.end() {
        for dy in *y_range.start()..=-*y_range.start() {
            let mut reached_target = false;
            for (x, y) in Simulation::new_from_velocity(dx, dy).into_iter() {
                if y < *y_range.start() || x > *x_range.end() {
                    break;
                } else if x_range.contains(&x) && y_range.contains(&y) {
                    reached_target = true;
                }
                
            }
            if reached_target {
                count += 1;
            }
        }
    }
    count
}

#[test]
fn test_part_1_example() {
    assert_eq!(part_1("target area: x=20..30, y=-10..-5"), 45);
}

#[test]
fn test_part_2_example() {
    assert_eq!(part_2("target area: x=20..30, y=-10..-5"), 112);
}

#[test]
fn test_part_1_input() {
    let input = include_str!("../input/2021/day17.txt");
    assert_eq!(part_1(input), 10296);
}

#[test]
fn test_part_2_input() {
    let input = include_str!("../input/2021/day17.txt");
    assert_eq!(part_2(input), 2371);
}
