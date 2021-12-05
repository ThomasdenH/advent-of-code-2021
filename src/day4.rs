use nom::IResult;

pub fn part_1(input: &str) -> u32 {
    let (_, (numbers, mut cards)) = parse::parse(input.as_bytes())
        .map_err(|err| panic!("{}", err))
        .unwrap();
    for &number in numbers.iter() {
        for card in cards.iter_mut() {
            card.mark(number);
            if card.has_won() {
                return u32::from(number) * card.unmarked_number_sum();
            }
        }
    }
    panic!("No solution!")
}

pub fn part_2(input: &str) -> u32 {
    let (_, (numbers, mut cards)) = parse::parse(input.as_bytes())
        .map_err(|err| panic!("{}", err))
        .unwrap();
    for &number in numbers.iter() {
        if let [final_card] = cards.as_mut_slice() {
            final_card.mark(number);
            if final_card.has_won() {
                return u32::from(number) * final_card.unmarked_number_sum();
            }
        } else {
            let mut i = 0;
            // If there are multiple cards, remove all winning cards
            while i < cards.len() {
                cards[i].mark(number);
                if cards[i].has_won() {
                    cards.remove(i);
                } else {
                    i += 1;
                }
            }
        }
    }
    panic!("No solution!")
}

const MARKED_MASK: u8 = 0b1000_0000;

#[derive(PartialEq, Eq, Debug)]
struct BingoCard {
    // Most significant bit is used to mark seen numbers.
    numbers: [u8; 25],
}

impl BingoCard {
    fn parse(input: &[u8]) -> IResult<&[u8], BingoCard> {
        let mut numbers = [0; 25];
        let (bingo_card_bytes, remainder) = input.split_at(25 * 3 - 1);
        for (bytes, n) in bingo_card_bytes.chunks(3).zip(numbers.iter_mut()) {
            *n = 10 * (bytes[0] & 0b1111) + (bytes[1] & 0b1111);
        }
        Ok((remainder, BingoCard { numbers }))
    }

    fn mark(&mut self, num: u8) {
        for number in self.numbers.iter_mut() {
            if *number == num {
                *number |= MARKED_MASK;
                break;
            }
        }
    }

    fn is_marked(&self, column: usize, row: usize) -> bool {
        self.numbers[column + row * 5] & MARKED_MASK != 0
    }

    fn has_column(&self) -> bool {
        'column_loop: for column in 0..5 {
            for row in 0..5 {
                if !self.is_marked(column, row) {
                    continue 'column_loop;
                }
            }
            return true;
        }
        false
    }

    fn has_row(&self) -> bool {
        'row_loop: for row in 0..5 {
            for column in 0..5 {
                if !self.is_marked(column, row) {
                    continue 'row_loop;
                }
            }
            return true;
        }
        false
    }

    fn has_won(&self) -> bool {
        self.has_column() || self.has_row()
    }

    fn unmarked_number_sum(&self) -> u32 {
        self.numbers
            .iter()
            .copied()
            .filter(|num| num & MARKED_MASK == 0)
            .map(u32::from)
            .sum()
    }
}

mod parse {
    use nom::{
        bytes::complete::tag,
        character::complete::digit1,
        combinator::map,
        multi::separated_list1,
        sequence::{terminated, tuple},
        IResult,
    };

    use super::BingoCard;

    fn number(input: &[u8]) -> IResult<&[u8], u8> {
        map(digit1, |s: &[u8]| {
            s.iter().map(|d| d - b'0').fold(0, |acc, x| 10 * acc + x)
        })(input)
    }

    pub(super) fn parse(input: &[u8]) -> IResult<&[u8], (Vec<u8>, Vec<BingoCard>)> {
        tuple((
            terminated(separated_list1(tag(","), number), tag("\n\n")),
            separated_list1(tag("\n\n"), BingoCard::parse),
        ))(input)
    }
}

#[test]
fn test_parse_bingo_card() {
    let card = b"22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19";
    assert_eq!(
        BingoCard::parse(card),
        Ok((
            [].as_ref(),
            BingoCard {
                numbers: [
                    22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12,
                    20, 15, 19
                ]
            }
        ))
    );
}

#[test]
fn test_parsing() {
    parse::parse(
        b"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7",
    )
    .unwrap();
}

#[test]
fn test_part_1_example() {
    assert_eq!(
        part_1(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
        ),
        4512
    );
}

#[test]
fn test_part_2_example() {
    assert_eq!(
        part_2(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
        ),
        1924
    );
}

#[test]
fn test_part_1_input() {
    let input = include_str!("../input/2021/day4.txt");
    assert_eq!(part_1(input), 69579);
}

#[test]
fn test_part_2_input() {
    let input = include_str!("../input/2021/day4.txt");
    assert_eq!(part_2(input), 14877);
}
