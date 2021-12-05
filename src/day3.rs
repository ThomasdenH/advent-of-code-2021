#[rustfmt::skip]
#[allow(clippy::all)]
mod code_golf_part_1;

pub use code_golf_part_1::z as part_1_code_golf;

const fn ones_mask(size: usize) -> u32 {
    let mut mask = 0;
    let mut pos = 0;
    loop {
        mask |= 1 << pos;
        pos += 1;
        if pos >= size {
            break;
        }
    }
    mask
}

pub fn part_1_number_len<const NUMBER_SIZE: usize>(input: &str) -> u32 {
    let mut outputs = [0u16; NUMBER_SIZE];

    for chunk in input.as_bytes().chunks(NUMBER_SIZE + 1) {
        for (output, num) in outputs.iter_mut().zip(chunk) {
            *output += u16::from(0b1 & num);
        }
    }

    let total = (input.len() + 1) / (NUMBER_SIZE + 1);
    let gamma = outputs[..NUMBER_SIZE].iter().fold(0, |acc, b| {
        (acc << 1) + if *b > total as u16 / 2 { 1 } else { 0 }
    });
    let epsilon = ones_mask(NUMBER_SIZE) ^ gamma;
    epsilon * gamma
}

pub fn part_1(input: &str) -> u32 {
    part_1_number_len::<12>(input)
}

pub fn part_2(input: &str) -> u32 {
    part_2_number_len::<12>(input)
}

#[derive(PartialEq, Eq)]
enum RatingType {
    OxygenGenerator,
    Co2Scrubber,
}

fn part_2_number_len<const NUMBER_LEN: usize>(input: &str) -> u32 {
    let numbers: Vec<u32> = read_numbers(input).collect();

    let perform_iter = |rating_type: RatingType| -> u32 {
        let mut current = 0;
        let mut current_mask = 0;

        for i in (0..NUMBER_LEN).rev() {
            let position_mask = 1 << i;
            // Count how many match the current number
            let (zeros, ones, last) = numbers
                .iter()
                // Filter out those that do not fit
                .filter(|num| (*num & current_mask) == current)
                .fold((0, 0, 0), |(zeros, ones, _last), num| {
                    if num & position_mask == 0 {
                        (zeros + 1, ones, *num)
                    } else {
                        (zeros, ones + 1, *num)
                    }
                });

            if ones + zeros == 1 {
                return last;
            }
            if (rating_type == RatingType::OxygenGenerator && ones >= zeros)
                || (rating_type == RatingType::Co2Scrubber && ones < zeros)
            {
                current |= position_mask;
            }
            current_mask |= position_mask;
        }
        current
    };

    let oxygen_generator_rating = perform_iter(RatingType::OxygenGenerator);
    let co2_scrubber_rating = perform_iter(RatingType::Co2Scrubber);

    oxygen_generator_rating * co2_scrubber_rating
}

fn read_numbers(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.lines().map(|line| {
        line.as_bytes()
            .iter()
            .fold(0, |acc, b| (acc << 1) + if *b == b'1' { 1 } else { 0 })
    })
}

#[test]
fn test_part_1_input() {
    let input = include_str!("../input/2021/day3.txt");
    assert_eq!(part_1(input), 3320834);
}

#[test]
fn test_part_1_code_golf_input() {
    let input = include_str!("../input/2021/day3.txt");
    assert_eq!(part_1_code_golf(input), 3320834);
}

#[test]
fn test_part_1_example() {
    let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    assert_eq!(part_1_number_len::<5>(input), 198);
}

#[test]
fn test_part_2_example() {
    let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    assert_eq!(part_2_number_len::<5>(input), 230);
}

#[test]
fn test_part_2_input() {
    let input = include_str!("../input/2021/day3.txt");
    assert_eq!(part_2(input), 4481199);
}
