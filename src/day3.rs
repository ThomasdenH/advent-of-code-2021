pub fn part_1(input: &str) -> u32 {
    const NUMBER_LEN: usize = 12;
    let mut outputs = [0u32; NUMBER_LEN + 1];
    for (byte, out_index) in input.as_bytes().iter().zip((0..(NUMBER_LEN + 1)).cycle()) {
        outputs[out_index] += u32::from(0b1 & *byte);
    }

    let total = (input.len() + 1) / (NUMBER_LEN + 1);
    let gamma = outputs[..NUMBER_LEN].iter().fold(0, |acc, b| {
        (acc << 1) + if *b > total as u32 / 2 { 1 } else { 0 }
    });
    let epsilon = 0b111111111111 ^ gamma;
    dbg!(gamma, epsilon);
    epsilon * gamma
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
    assert_eq!(part_2_number_len::<12>(input), 230);
}

#[test]
fn test_part_2_input() {
    let input = include_str!("../input/2021/day3.txt");
    assert_eq!(part_2(input), 4481199);
}
