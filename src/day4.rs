use core::num;
use std::{ops::Add, iter::Sum};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct SimplifiedNumber(u8);

impl From<SimplifiedNumber> for u32 {
    fn from(SimplifiedNumber(num): SimplifiedNumber) -> u32 {
        u32::from((num & 0b1111) + 10 * ((num >> 4) & 0b1111))
    }
}

impl std::iter::Sum<SimplifiedNumber> for u32 {
    fn sum<I: Iterator<Item = SimplifiedNumber>>(iter: I) -> u32 {
        let mut a = 0;
        let mut b = 0;
        for number in iter {
            b += u32::from(number.0 & 0b1111);
            a += u32::from(number.0 & 0b11110000);
        }
        b + 10 * (a >> 4)
    }
}

pub fn part_1(input: &str) -> u32 {
    let (numbers, mut cards) = parse::parse(input.as_bytes());
    for number in numbers {
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
    let (numbers, mut cards) = parse::parse(input.as_bytes());
    for number in numbers {
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

const MARKED_VALUE: SimplifiedNumber = SimplifiedNumber(0b11111111);

#[derive(PartialEq, Eq, Debug)]
struct BingoCard {
    // Most significant bit is used to mark seen numbers.
    numbers: [SimplifiedNumber; 25],
}

impl BingoCard {
    fn parse(input: &[u8]) -> (&[u8], BingoCard) {
        let mut numbers = [SimplifiedNumber(0); 25];
        let (bingo_card_bytes, remainder) = input.split_at(25 * 3 - 1);
        for (bytes, n) in bingo_card_bytes.chunks(3).zip(numbers.iter_mut()) {
            *n = SimplifiedNumber(((bytes[0] & 0b1111) << 4) | (bytes[1] & 0b1111));
        }
        (remainder, BingoCard { numbers })
    }

    fn mark(&mut self, num: SimplifiedNumber) {
        for number in self.numbers.iter_mut() {
            if *number == num {
                *number = MARKED_VALUE;
                break;
            }
        }
    }

    fn is_marked(&self, column: usize, row: usize) -> bool {
        self.numbers[column + row * 5] == MARKED_VALUE
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
            .filter(|num| *num != MARKED_VALUE)
            .sum::<u32>()
    }
}

mod parse {
    use super::{BingoCard, SimplifiedNumber};

    pub(super) fn parse_number(input: &[u8]) -> (&[u8], SimplifiedNumber, bool) {
        let (a, input) = input.split_first().unwrap();
        let mut a = *a;
        a &= 0b1111;
        match input[0] {
            b',' => (&input[1..], SimplifiedNumber(a), false),
            b'\n' => (&input[1..], SimplifiedNumber(a), true),
            other => {
                a <<= 4;
                a |= other & 0b1111;
                let is_final = input[1] == b'\n';
                (&input[2..], SimplifiedNumber(a), is_final)
            }
        }
    }

    pub(super) struct SimplifiedNumberList<'a>(&'a [u8]);

    impl<'a> Iterator for SimplifiedNumberList<'a> {
        type Item = SimplifiedNumber;
        fn next(&mut self) -> Option<SimplifiedNumber> {
            if self.0.is_empty() {
                None
            } else {
                let (new_input, number, _) = parse_number(self.0);
                self.0 = new_input;
                Some(number)
            }
        }
    }

    pub(super) fn parse(mut input: &[u8]) -> (SimplifiedNumberList<'_>, Vec<BingoCard>) {
        let pos = input.iter().position(|b| *b == b'\n').unwrap();
        let list = SimplifiedNumberList(&input[..pos]);
        let mut input = &input[pos + 2..];
        let mut cards = Vec::new();
        loop {
            let (new_input, card) = BingoCard::parse(input);
            input = new_input;
            cards.push(card);
            if input.len() > 2 {
                input = &input[2..];
            } else {
                return (list, cards);
            }
        }
    }
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
