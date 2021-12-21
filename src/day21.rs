fn parse(input: &[u8]) -> (u8, u8) {
    let skip_1 = "Player 1 starting position: ".len();
    let skip_2 = "Player 1 starting position: 7\nPlayer 2 starting position: ".len();
    (input[skip_1] - b'0', input[skip_2] - b'0')
}

struct Dice(u8, usize);

impl Dice {
    const fn new() -> Self {
        Dice(6, 0)
    }

    fn next_sum_mod_10(&mut self) -> u8 {
        let curr = self.0;
        // We do this mod 10 anyway, so do so immediately
        self.0 += 3 * 3;
        self.1 += 3;
        if self.0 >= 10 {
            self.0 -= 10;
        }
        curr
    }

    const fn times_trown(&self) -> usize {
        self.1
    }
}

lazy_static::lazy_static!{
    static ref FINAL_SCORE: [[usize; 10]; 10] = {
        let mut final_score = [[0; 10]; 10];
        for x in 0..10 {
            for y in 0..10 {
                let dice = &mut Dice::new();
                let mut p_1 = Player::new(x - 1);
                let mut p_2 = Player::new(y - 1);
                final_score[usize::from(x)][usize::from(y)] = loop {
                    p_1.next_turn(dice);
                    if p_1.has_won() {
                        break p_2.score() * dice.times_trown();
                    }
            
                    p_2.next_turn(dice);
                    if p_2.has_won() {
                        break p_1.score() * dice.times_trown();
                    }
                };
            }
        }
        final_score
    };
}

#[derive(Eq, PartialEq, Debug)]
struct Player {
    current_pos: u8,
    score: usize
}

impl Player {
    const fn new(current_pos: u8) -> Self {
        Player {
            current_pos,
            score: 0
        }
    }

    fn next_turn(&mut self, dice: &mut Dice) {
        self.current_pos += dice.next_sum_mod_10();
        if self.current_pos >= 10 {
            self.current_pos -= 10;
        }
        self.score += usize::from(self.current_pos) + 1;
    }

    fn has_won(&self) -> bool {
        self.score >= 1000
    }

    fn score(&self) -> usize {
        self.score
    }
}

pub fn part_1(input: &str) -> usize {
    let (p_1, p_2) = parse(input.as_bytes());
    FINAL_SCORE[usize::from(p_1 - 1)][usize::from(p_2 - 1)]
}

#[test]
fn test_part_1_example() {
    assert_eq!(part_1("Player 1 starting position: 4
Player 2 starting position: 8"), 739785);
}

pub fn part_2(input: &str) -> usize {
    const WINNING_SCORE: usize = 21;
    const SCORE_OFFSET: usize = 3;
    let (starting_score_1, starting_score_2) = parse(input.as_bytes());
    let mut a = [[0usize; WINNING_SCORE + SCORE_OFFSET]; WINNING_SCORE + SCORE_OFFSET];
    let mut b = [[0usize; WINNING_SCORE + SCORE_OFFSET]; WINNING_SCORE + SCORE_OFFSET];
    let score_1 = usize::from(starting_score_1);
    let score_2 = usize::from(starting_score_2);
    b[score_1 + SCORE_OFFSET][score_2 + SCORE_OFFSET] = 1;
    dbg!(score_1, score_2);
    let mut possibilities_for_score = 0;
    for sum in (score_1 + score_2 + 1)..(2 * WINNING_SCORE - 1) {
        let min_score_1 = if sum > WINNING_SCORE - 1 {
            sum - (WINNING_SCORE - 1)
        } else {
            usize::from(starting_score_1)
        };
        for score_1 in min_score_1..sum.min(WINNING_SCORE) {
            assert!(score_1 < WINNING_SCORE && score_2 < WINNING_SCORE);
            let score_2 = sum - score_1;
            dbg!(score_1, score_2);

            print!("A\n\t");
            for x in 1..WINNING_SCORE {
                print!("|{}|\t", x);
            }
            println!();
            for y in 1..WINNING_SCORE {
                print!("{} |\t", y);
                for x in 1..WINNING_SCORE {
                    print!("{}\t", a[x + SCORE_OFFSET][y + SCORE_OFFSET]);
                }
                println!();
            }

            print!("B\n\t");
            for x in 1..WINNING_SCORE {
                print!("|{}|\t", x);
            }
            println!();
            for y in 1..WINNING_SCORE {
                print!("{} |\t", y);
                for x in 1..WINNING_SCORE {
                    print!("{}\t", b[x + SCORE_OFFSET][y + SCORE_OFFSET]);
                }
                println!();
            }

            a[score_1 + SCORE_OFFSET][score_2 + SCORE_OFFSET] = b[score_1 + SCORE_OFFSET - 1][score_2 + SCORE_OFFSET]
                + b[score_1 + SCORE_OFFSET - 2][score_2 + SCORE_OFFSET]
                + b[score_1 + SCORE_OFFSET - 3][score_2 + SCORE_OFFSET]
                + a[score_1 + SCORE_OFFSET - 1][score_2 + SCORE_OFFSET]
                + a[score_1 + SCORE_OFFSET - 2][score_2 + SCORE_OFFSET]
                + a[score_1 + SCORE_OFFSET - 3][score_2 + SCORE_OFFSET]
                + a[score_1 + SCORE_OFFSET - 1][score_2 + SCORE_OFFSET]
                + a[score_1 + SCORE_OFFSET - 2][score_2 + SCORE_OFFSET]
                + a[score_1 + SCORE_OFFSET - 3][score_2 + SCORE_OFFSET];
            b[score_1 + SCORE_OFFSET][score_2 + SCORE_OFFSET] = a[score_1 + SCORE_OFFSET][score_2 + SCORE_OFFSET - 1]
                + a[score_1 + SCORE_OFFSET][score_2 + SCORE_OFFSET - 2]
                + a[score_1 + SCORE_OFFSET][score_2 + SCORE_OFFSET - 3]
                + b[score_1 + SCORE_OFFSET][score_2 + SCORE_OFFSET - 1]
                + b[score_1 + SCORE_OFFSET][score_2 + SCORE_OFFSET - 2]
                + b[score_1 + SCORE_OFFSET][score_2 + SCORE_OFFSET - 3]
                + b[score_1 + SCORE_OFFSET][score_2 + SCORE_OFFSET - 1]
                + b[score_1 + SCORE_OFFSET][score_2 + SCORE_OFFSET - 2]
                + b[score_1 + SCORE_OFFSET][score_2 + SCORE_OFFSET - 3];
        }
    }
    // p_1 always wins by throwing the dice last
    let p_1_wins: usize = (1..=3).flat_map(|dice| {
        (0..WINNING_SCORE).map(move |p_2_score| {
            b[WINNING_SCORE - dice + SCORE_OFFSET][p_2_score + SCORE_OFFSET] * (4 - dice)
        })
    }).sum();
    let p_2_wins: usize = (1..=3).flat_map(|dice| {
        (0..WINNING_SCORE).map(move |p_1_score| {
            a[p_1_score + SCORE_OFFSET][WINNING_SCORE - dice + SCORE_OFFSET] * (4 - dice)
        })
    }).sum();
    p_1_wins.max(p_2_wins)
}

#[test]
fn test_die() {
    let mut dice_struct = Dice::new();
    let mut dice = 1_usize;
    for round in 0..1000 {
        let sum = dice + dice + 1 + dice + 2;
        dice += 3;
        dice %= 1000;
        assert_eq!(usize::from(dice_struct.next_sum_mod_10()) % 10, sum % 10, "failure in round {}", round);
    }
}

#[test]
fn test_part_2_example() {
    assert_eq!(part_2("Player 1 starting position: 4
Player 2 starting position: 8"), 444356092776315);
}
