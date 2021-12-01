use itertools::*;

pub fn part_1(input: &str) -> usize {
    parse_numbers(input)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

pub fn part_2(input: &str) -> usize {
    parse_numbers(input)
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(sum_a, sum_b)| sum_b > sum_a)
        .count()
}

fn parse_numbers(input: &str) -> impl Iterator<Item = u16> + '_ {
    let mut iter = input.as_bytes().iter().copied();
    const NUMBER_MASK: u8 = 0b0100000;
    std::iter::from_fn(move || {
        iter.next().map(|x| {
            (&mut iter)
                .take_while(|digit| digit & NUMBER_MASK != 0)
                .map(|byte| u16::from(byte - b'0'))
                .fold(u16::from(x - b'0'), |acc, digit| 10 * acc + digit)
        })
    })
}

#[test]
fn test_part_1_example() {
    let input = r#"199
200
208
210
200
207
240
269
260
263"#;
    assert_eq!(part_1(input), 7);
}

#[test]
fn test_part_1_input() {
    let input = include_str!("../input/2021/day1.txt");
    assert_eq!(part_1(input), 1791);
}

#[test]
fn test_part_2_example() {
    let input = r#"199
200
208
210
200
207
240
269
260
263"#;
    assert_eq!(part_2(input), 5);
}

#[test]
fn test_part_2_input() {
    let input = include_str!("../input/2021/day1.txt");
    assert_eq!(part_2(input), 1791);
}
