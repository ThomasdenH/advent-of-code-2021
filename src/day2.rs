use std::{str};

enum Direction {
    Forward,
    Up,
    Down,
}

pub fn part_1(input: &str) -> u64 {
    let (horizontal, depth) = parse(input).fold(
        (0, 0),
        |(horizontal, depth), (direction, magnitude)| match direction {
            Direction::Down => (horizontal, depth + magnitude),
            Direction::Forward => (horizontal + magnitude, depth),
            Direction::Up => (horizontal, depth - magnitude),
        },
    );
    horizontal * depth
}

pub fn part_2(input: &str) -> u64 {
    let (horizontal, depth, _aim) = parse(input).fold(
        (0, 0, 0),
        |(horizontal, depth, aim), (direction, magnitude)| match direction {
            Direction::Down => (horizontal, depth, aim + magnitude),
            Direction::Up => (horizontal, depth, aim - magnitude),
            Direction::Forward => (horizontal + magnitude, depth + aim * magnitude, aim),
        },
    );
    horizontal * depth
}

fn parse_direction(input: &mut &[u8]) -> Option<Direction> {
    input.first()
        .copied()
        .map(|b| match b {
            b'f' => {
                *input = &input[8..];
                Direction::Forward
            }
            b'd' => {
                *input = &input[5..];
                Direction::Down
            }
            b'u' => {
                *input = &input[3..];
                Direction::Up
            }
            _ => unreachable!("Invalid input, expected direction")
        })
}

fn parse_number(input: &mut &[u8]) -> u64 {
    const NUMBER_MASK: u8 = 0b0100000;
    std::iter::from_fn(|| {
        // Take the first digit while 
        input.split_first()
            .map(|(digit, remainder)| {
                *input = remainder;
                digit
            })
    })
    .take_while(|d| *d & NUMBER_MASK != 0)
    .inspect(|d| debug_assert!(d.is_ascii_digit()))
    .map(|d| u64::from(d - b'0'))
    .fold(0, |acc, d| 10 * acc + d)
}

fn parse(input: &str) -> impl Iterator<Item = (Direction, u64)> + '_ {
    let mut input = input.as_bytes();
    std::iter::from_fn(move || {
        parse_direction(&mut input)
            .map(|direction| (direction, parse_number(&mut input)))
    })
}

#[test]
fn test_part_1_example() {
    let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    assert_eq!(part_1(input), 150);
}

#[test]
fn test_part_2_example() {
    let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    assert_eq!(part_2(input), 900);
}

#[test]
fn test_part_1_input() {
    let input = include_str!("../input/2021/day2.txt");
    assert_eq!(part_1(input), 1813801);
}

#[test]
fn test_part_2_input() {
    let input = include_str!("../input/2021/day2.txt");
    assert_eq!(part_2(input), 1960569556);
}
