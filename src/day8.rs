fn parse_digit(input: &mut &[u8]) -> Option<u8> {
    if let Some((&first, remaining)) = input.split_first() {
        *input = remaining;
        match first {
            b'a' => Some(0b1),
            b'b' => Some(0b10),
            b'c' => Some(0b100),
            b'd' => Some(0b1000),
            b'e' => Some(0b10000),
            b'f' => Some(0b100000),
            b'g' => Some(0b10000000),
            _ => None,
        }
    } else {
        None
    }
}
fn parse_segment(input: &mut &[u8]) -> Option<u8> {
    parse_digit(input)
        .map(|first| std::iter::from_fn(|| parse_digit(input)).fold(first, |acc, d| acc | d))
}

fn parse_multiple<const COUNT: usize>(input: &mut &[u8]) -> [u8; COUNT] {
    let mut digits = [0; COUNT];
    for digit_dest in digits.iter_mut() {
        *digit_dest = parse_segment(input).unwrap();
    }
    digits
}

fn parse_line(input: &mut &[u8]) -> ([u8; 10], [u8; 4]) {
    let digits = parse_multiple::<10>(input);
    *input = &input[2..];
    let remainder = parse_multiple::<4>(input);
    (digits, remainder)
}

pub fn part_1(input: &str) -> usize {
    let mut input = input.as_bytes();
    std::iter::from_fn(|| Some(&mut input).filter(|i| !i.is_empty()).map(parse_line))
        .flat_map(|(_, display)| display.into_iter())
        .filter(|digit| match digit.count_ones() {
            2 | 4 | 3 | 7 => true,
            _ => false,
        })
        .count()
}

fn find_one_and_four(digits: [u8; 10]) -> (u8, u8) {
    let mut one = 0;
    let mut four = 0;
    for digit in digits.iter() {
        match digit.count_ones() {
            2 => one = *digit,
            4 => four = *digit,
            _ => {}
        }
    }
    (one, four)
}

struct Decoder([u8; 10]);

impl Decoder {
    fn from_digits(digits: [u8; 10]) -> Decoder {
        let (one, four) = find_one_and_four(digits);
        Decoder(Decoder::find_encodings(digits, one, four))
    }

    fn find_encodings(digits: [u8; 10], one_encoding: u8, four_encoding: u8) -> [u8; 10] {
        let mut encodings = [0; 10];
        for digit in digits {
            let ones = digit.count_ones();
            let ones_one_mask = (digit & one_encoding).count_ones();
            let ones_four_mask = (digit & four_encoding).count_ones();
            let decoded = match (ones, ones_one_mask, ones_four_mask) {
                (6, 2, 3) => 0,
                (2, _, _) => 1,
                (5, 1, 2) => 2,
                (5, 2, 3) => 3,
                (4, _, _) => 4,
                (5, 1, 3) => 5,
                (6, 1, 3) => 6,
                (3, _, _) => 7,
                (7, _, _) => 8,
                (6, 2, 4) => 9,
                other => unreachable!("unexpected combination: {:?}", other),
            };
            encodings[decoded] = digit;
        }
        encodings
    }

    fn decode_digit(&self, d: u8) -> u16 {
        self.0.iter().position(|b| *b == d).unwrap() as u16
    }

    fn decode_display(&self, d: [u8; 4]) -> u16 {
        d.into_iter()
            .map(|d| self.decode_digit(d))
            .fold(0, |acc, d| 10 * acc + d)
    }
}

pub fn part_2(input: &str) -> usize {
    let mut input = input.as_bytes();
    std::iter::from_fn(|| Some(&mut input).filter(|i| !i.is_empty()).map(parse_line))
        .map(|(digits, display)| usize::from(Decoder::from_digits(digits).decode_display(display)))
        .sum()
}

fn compute_frequency_table(input: &mut impl Iterator<Item = u8>) -> [u8; 256] {
    let mut frequency_table = [0; 256];
    for b in input {
        if b == b'|' {
            break;
        }
        frequency_table[usize::from(b)] += 1;
    }
    frequency_table
}

fn decode_digit(input: &mut impl Iterator<Item = u8>, frequency_table: [u8; 256], radix: usize) -> usize {
    let sum = input
    // End when hitting ' ' or '\n' 
    .take_while(|b| *b > (b'a' - 1))
    .map(|b| frequency_table[usize::from(b)])
    .sum();
    match sum {
        17 => 1,
        34 => 2,
        39 => 3,
        30 => 4,
        37 => 5,
        41 => 6,
        25 => 7,
        49 => 8,
        45 => 9,
        _ => { debug_assert_eq!(sum, 42); 0 }
    }
}

fn decode_number(input: &mut impl Iterator<Item = u8>, frequency_table: [u8; 256]) -> usize {
    (0..4).rev().map(|radix| decode_digit(input, frequency_table, 10_usize.pow(radix))).sum()
}

pub fn part_2_frequency_table(input: &str) -> usize {
    let mut input = input.bytes();
    let mut sum = 0;
    std::iter::from_fn(|| {
        if input.len() == 0 {
            None
        } else {
            let frequency_table = compute_frequency_table(&mut input);
            input.next();
            Some(decode_number(&mut input, frequency_table))
        }
    })
    .sum()
}

#[test]
fn test_part_1_example() {
    let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
fgae cfgab fg bagce";
    assert_eq!(part_1(input), 26);
}
