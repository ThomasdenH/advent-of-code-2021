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
        // Compare the value that is removed with the one that is added
        .filter(|(removed, _, _, added)| added > removed)
        .count()
}

fn parse_numbers(input: &str) -> impl Iterator<Item = u64> + '_ {
    let mut iter = input
        .as_bytes()
        .iter()
        .copied()
        .inspect(|&a| debug_assert!(a.is_ascii_digit() || a == b'\n'));
    std::iter::from_fn(move || {
        iter.next().map(|x| {
            (&mut iter)
                .take_while(|b| *b != b'\n')
                // Map to u64, preserve order
                .fold(u64::from(x), |acc, digit| (acc << 8) | u64::from(digit))
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
    assert_eq!(part_2(input), 1822);
}
