use std::fmt;

#[derive(Eq, PartialEq)]
enum ParseResult<'a> {
    Valid,
    Incomplete { stack: &'a mut Vec<u8> },
    /// Chunk closes with the wrong character
    Corrupted {
        expected: u8,
        found: u8
    }
}

impl ParseResult<'_> {
    fn to_incomplete_string(&self) -> Option<String> {
        if let ParseResult::Incomplete { stack} = self {
            let mut s = String::new();
            for item in stack.iter().rev() {
                s.push(match item {
                    b'(' => ')',
                    b'[' => ']',
                    b'<' => '>',
                    b'{' => '}',
                    _ => unreachable!("invalid input!")
                });
            }
            Some(s)
        } else {
            None
        }
    }
}

impl fmt::Debug for ParseResult<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseResult::Valid => writeln!(f, "Parse result: valid"),
            ParseResult::Incomplete { .. } => {
                write!(f, "Parse result: incomplete. expected '{}'", self.to_incomplete_string().unwrap())
            },
            ParseResult::Corrupted { expected, found }=> writeln!(f, "Parse result: corrupted (expected closer for {} but found {})", char:: from(*expected), char::from(*found))
        }
    }
}

fn is_opening(b: u8) -> bool {
    // 0b00101000 ( -> 0b00101001 -> 0
    // 0b00111100 < -> 0b00111101 -> 0
    // 0b01011011 [ -> 0b01011100 -> 0
    // 0b01111011 { -> 0b01111100 -> 0
    // 0b00101001 ) -> 0b00101010 -> 1
    // 0b00111110 > -> 0b00111111 -> 1
    // 0b01011101 ] -> 0b01011110 -> 1
    // 0b01111101 } -> 0b01111110 -> 1
    // 0b00001010 \n-> 0b00001011 -> 1 (Not an opening either)
    //                         ^
    (b + 1) & 0b10 == 0
}

fn matches(a: u8, b: u8) -> bool {
    debug_assert!(is_opening(a) && !is_opening(b));
    a ^ b < 0b111
}

fn parse<'a>(stack: &'a mut Vec<u8>, input: &mut impl Iterator<Item = u8>) -> ParseResult<'a> {
    debug_assert!(stack.is_empty());
    loop {
        match input.next().unwrap_or(b'\n') {
            b'\n' if stack.is_empty() => return ParseResult::Valid,
            b'\n' => return ParseResult::Incomplete { stack },
            c if is_opening(c) => stack.push(c),
            found => {
                if let Some(expected) = stack.pop() {
                    if !matches(expected, found) {
                        // Flush remainder of line
                        for n in input {
                            if n == b'\n' {
                                break;
                            }
                        }
                        return ParseResult::Corrupted { expected, found };
                    }
                }
            }
        }
    }
}

pub fn part_1(input: &str) -> usize {
    let mut iter = input.bytes();
    let mut stack = Vec::new();
    let mut acc = 0;
    while iter.len() > 0 {
        if let ParseResult::Corrupted { found, .. } = parse(&mut stack, &mut iter) {
            acc += match found {
                b')' => 3,
                b']' => 57,
                b'}' => 1197,
                other => {
                    debug_assert_eq!(other, b'>');
                    25137
                }
            };
        }
        stack.clear();
    }
    acc
}


pub fn part_2(input: &str) -> usize {
    let mut iter = input.bytes();
    let mut stack = Vec::new();
    let mut acc = Vec::new();
    while iter.len() > 0 {
        if let ParseResult::Incomplete { stack } = parse(&mut stack, &mut iter) {
            acc.push(stack.drain(..)
                .map(|b| match b {
                    b'(' => 1,
                    b'[' => 2,
                    b'{' => 3,
                    _ => {
                        debug_assert_eq!(b, b'<');
                        4
                    }
                }).rev().fold(0, |acc, x| 5 * acc + x)
            );
        }
        stack.clear();
    }
    let l = acc.len();
    *acc.select_nth_unstable(l / 2).1
}

#[test]
fn test_part_1_example() {
    let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
    assert_eq!(part_1(input), 26397);
}

#[test]
fn test_part_2_example() {
    let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
    assert_eq!(part_2(input), 288957);
}

#[test]
fn test_part_1_input() {
    let input = include_str!("../input/2021/day10.txt");
    assert_eq!(part_1(input), 323613);
}

#[test]
fn test_part_2_input() {
    let input = include_str!("../input/2021/day10.txt");
    assert_eq!(part_2(input), 3103006161);
}

#[test]
fn test_is_opening() {
    assert!(is_opening(b'('));
    assert!(is_opening(b'{'));
    assert!(is_opening(b'['));
    assert!(is_opening(b'<'));
    assert!(!is_opening(b'\n'));
    assert!(!is_opening(b')'));
    assert!(!is_opening(b'}'));
    assert!(!is_opening(b'>'));
    assert!(!is_opening(b']'));
}
