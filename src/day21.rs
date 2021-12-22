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

lazy_static::lazy_static! {
    static ref FINAL_SCORE: [[usize; 10]; 10] = {
        let mut final_score = [[0; 10]; 10];
        for x in 0..10 {
            for y in 0..10 {
                let dice = &mut Dice::new();
                let mut p_1 = Player::new(x);
                let mut p_2 = Player::new(y);
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
    score: usize,
}

impl Player {
    const fn new(current_pos: u8) -> Self {
        Player {
            current_pos,
            score: 0,
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
    assert_eq!(
        part_1(
            "Player 1 starting position: 4
Player 2 starting position: 8"
        ),
        739785
    );
}

pub fn part_2(input: &str) -> usize {
    const SCORE_OFFSET: usize = 3;
    let (starting_score_1, starting_score_2) = parse(input.as_bytes());
    let mut a = [[0usize; WINNING_SCORE + SCORE_OFFSET]; WINNING_SCORE + SCORE_OFFSET];
    let mut b = [[0usize; WINNING_SCORE + SCORE_OFFSET]; WINNING_SCORE + SCORE_OFFSET];
    let score_1 = usize::from(starting_score_1);
    let score_2 = usize::from(starting_score_2);
    b[score_1 + SCORE_OFFSET][score_2 + SCORE_OFFSET] = 1;
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
            /*
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
            }*/

            a[score_1 + SCORE_OFFSET][score_2 + SCORE_OFFSET] = b[score_1 + SCORE_OFFSET - 1]
                [score_2 + SCORE_OFFSET]
                + b[score_1 + SCORE_OFFSET - 2][score_2 + SCORE_OFFSET]
                + b[score_1 + SCORE_OFFSET - 3][score_2 + SCORE_OFFSET]
                + a[score_1 + SCORE_OFFSET - 1][score_2 + SCORE_OFFSET]
                + a[score_1 + SCORE_OFFSET - 2][score_2 + SCORE_OFFSET]
                + a[score_1 + SCORE_OFFSET - 3][score_2 + SCORE_OFFSET]
                + a[score_1 + SCORE_OFFSET - 1][score_2 + SCORE_OFFSET]
                + a[score_1 + SCORE_OFFSET - 2][score_2 + SCORE_OFFSET]
                + a[score_1 + SCORE_OFFSET - 3][score_2 + SCORE_OFFSET];
            b[score_1 + SCORE_OFFSET][score_2 + SCORE_OFFSET] = a[score_1 + SCORE_OFFSET]
                [score_2 + SCORE_OFFSET - 1]
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
    let p_1_wins: usize = (1..=3)
        .flat_map(|dice| {
            (0..WINNING_SCORE).map(move |p_2_score| {
                b[WINNING_SCORE - dice + SCORE_OFFSET][p_2_score + SCORE_OFFSET] * (4 - dice)
            })
        })
        .sum();
    let p_2_wins: usize = (1..=3)
        .flat_map(|dice| {
            (0..WINNING_SCORE).map(move |p_1_score| {
                a[p_1_score + SCORE_OFFSET][WINNING_SCORE - dice + SCORE_OFFSET] * (4 - dice)
            })
        })
        .sum();
    p_1_wins.max(p_2_wins)
}

/// An array representing the number of ways to throw that outcome.
/// ```rust
/// assert_eq!(THREE_DICE_OUTCOMES[3], 1); // Throw 1, 1, 1
/// assert_eq!(THREE_DICE_OUTCOMES[9], 1); // Throw 3, 3, 3
/// ```
const THREE_DICE_OUTCOMES: [usize; 10] = {
    let mut outcomes = [0; 10];
    let mut a = 1;
    while a <= 3 {
        let mut b = 1;
        while b <= 3 {
            let mut c = 1;
            while c <= 3 {
                outcomes[a + b + c] += 1;
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }
    outcomes
};

const WINNING_SCORE: usize = 21;
const SCORE_OFFSET: usize = 10;

const fn score_to_index(score: usize) -> usize {
    SCORE_OFFSET + score
}

const fn score_to_index_sub(score: usize, sub: usize) -> usize {
    SCORE_OFFSET + score - sub
}

struct WaysToGetScore {
    poss: [[Option<usize>; WINNING_SCORE + SCORE_OFFSET]; WINNING_SCORE + SCORE_OFFSET],
}

impl WaysToGetScore {
    const fn new() -> Self {
        WaysToGetScore {
            poss: [[None; WINNING_SCORE + SCORE_OFFSET]; WINNING_SCORE + SCORE_OFFSET],
        }
    }

    /// Compute the number of ways to get a score. For that, we should know the possibilities at the start of the turn, up to own_score, other_score.
    fn compute_turn(
        mut self,
        own_score: usize,
        other_score: usize,
        other_score_possibilities: &WaysToGetScore,
    ) -> WaysToGetScore {
        // We want to find the number of ways that we can reach the score `own_score`, `other_score`.
        // We can arrive there from `own_score - dice_trow`, `other_score` for each dice throw.

        // It shouldn't have been computed yet.
        // assert_eq!(self.poss[score_to_index(own_score)][score_to_index(other_score)], 0);

        self.poss[score_to_index(own_score)][score_to_index(other_score)] = Some(0);

        let mut dice_throw = 3;
        while dice_throw <= 9 {
            let possibilities_to_throw_dice = THREE_DICE_OUTCOMES[dice_throw];
            println!(
                "Looking up {} {}",
                other_score,
                own_score as isize - dice_throw as isize
            );
            *self.poss[score_to_index(own_score)][score_to_index(other_score)]
                .as_mut()
                .unwrap() += possibilities_to_throw_dice
                * other_score_possibilities.poss[score_to_index(other_score)]
                    [score_to_index_sub(own_score, dice_throw)]
                .unwrap();
            dice_throw += 1;
        }

        self
    }

    fn print(&self, start_1: usize, start_2: usize, highlight_x: usize, highlight_y: usize) {
        print!("\t");
        for x in start_1..WINNING_SCORE {
            print!("|{}|\t", x);
        }
        println!();
        for y in start_2..WINNING_SCORE {
            print!("{} |\t", y);
            for x in start_1..WINNING_SCORE {
                if x == highlight_x && y == highlight_y {
                    print!("[");
                } else {
                    print!(" ");
                }
                if let Some(u) = self.poss[x + SCORE_OFFSET][y + SCORE_OFFSET] {
                    print!("{}", u);
                } else {
                    print!("_");
                }
                if x == highlight_x && y == highlight_y {
                    print!("]\t");
                } else {
                    print!(" \t");
                }
            }
            println!();
        }
    }

    fn compute_possibilities_to_win(&self) -> usize {
        let mut possibilities = 0;
        let mut dice_throw = 3;
        while dice_throw <= 9 {
            let mut winning_score = WINNING_SCORE;
            // We always win by crossing WINNING_SCORE. But it can be more than that. In fact, WINNING_SCORE + dice roll - 1 can be reached.
            while winning_score < WINNING_SCORE + dice_throw {
                let possibilities_to_throw_dice = THREE_DICE_OUTCOMES[dice_throw];
                let mut other_score = SCORE_OFFSET;
                while other_score < WINNING_SCORE {
                    possibilities += possibilities_to_throw_dice
                        * self.poss[score_to_index_sub(winning_score, dice_throw)]
                            [score_to_index(other_score)]
                        .unwrap();
                    other_score += 1;
                }
                winning_score += 1;
            }
            dice_throw += 1;
        }
        possibilities
    }

    fn compute_ways_to_get_score(starting_score_p1: usize, starting_score_p2: usize) -> usize {
        let mut after_p1_turn = WaysToGetScore::new();
        let mut after_p2_turn = WaysToGetScore::new();

        let mut x = 0;
        while x < WINNING_SCORE + SCORE_OFFSET {
            let mut y = 0;
            while y < WINNING_SCORE + SCORE_OFFSET {
                if x < starting_score_p1 + SCORE_OFFSET || y < starting_score_p2 + SCORE_OFFSET {
                    after_p1_turn.poss[x][y] = Some(0);
                    after_p2_turn.poss[y][x] = Some(0);
                }
                y += 1;
            }
            x += 1;
        }
        after_p2_turn.poss[score_to_index(starting_score_p2)][score_to_index(starting_score_p1)] =
            Some(1);
        after_p1_turn.poss[score_to_index(starting_score_p1)][score_to_index(starting_score_p2)] =
            Some(0);

        // We want to go through all scores in incrementing fashion.
        let mut score_sum = starting_score_p1 + starting_score_p2 + 1;
        while score_sum <= 2 * WINNING_SCORE {
            // The p_1 score should be more than the starting score and never so low that the p_2 score is winning.
            // `max(starting_score_p1, score_sum - (WINNING_SCORE - 1))`
            let min_score_p1 = if starting_score_p1 + (WINNING_SCORE - 1) > score_sum {
                starting_score_p1
            } else {
                score_sum - (WINNING_SCORE - 1)
            };

            // Similartly, the p_2 score should never be a winning score and never so high that the p_2 score is lower
            // than their starting score.
            // `min(WINNING_SCORE - 1, score_sum - starting_score_p2)`
            let max_score_p1 = if WINNING_SCORE - 1 < score_sum - starting_score_p2 {
                WINNING_SCORE - 1
            } else {
                score_sum - starting_score_p2
            };

            let mut score_1 = min_score_p1;
            while score_1 <= max_score_p1 {
                let score_2 = score_sum - score_1;
                println!("{} {}", score_1, score_2);
                println!("After p_1 turn:");
                after_p1_turn.print(starting_score_p1, starting_score_p2, score_1, score_2);
                println!("After p_2 turn:");
                after_p2_turn.print(starting_score_p2, starting_score_p1, score_2, score_1);
                // First p_1 should throw the dice.
                after_p1_turn = after_p1_turn.compute_turn(score_1, score_2, &after_p2_turn);
                // Then p_2
                after_p2_turn = after_p2_turn.compute_turn(score_2, score_1, &after_p1_turn);
                score_1 += 1;
            }
            score_sum += 1;
        }

        // We have now computed the board up to WINNING_SCORE - 1. Finally, sum all outcomes for the final dice roll.
        after_p1_turn
            .compute_possibilities_to_win()
            .max(after_p2_turn.compute_possibilities_to_win())
    }
}

/// The solutions for part 2 stored in the following manner:
/// The solution for a=b'4', b=b'5' is stored at
/// (a ^ (b << 4)) mod 256
const PART_2_SOLUTIONS: [usize; 256] = {
    let mut solutions = [0; 256];
    let mut a = b'0';
    while a <= b'9' {
        let mut b = b'0';
        while b <= b'9' {
            solutions[(a ^ (b << 4)) as usize] = WaysToGetScore::compute_ways_to_get_score((a - b'0') as usize, (b - b'0') as usize);
            b += 1;
        }
        a += 1;
    }
    solutions
};

pub fn part_2_precomputed(input: &str) -> usize {
    let input = input.as_bytes();
    let skip_1 = "Player 1 starting position: ".len();
    let skip_2 = "Player 1 starting position: 7\nPlayer 2 starting position: ".len();
    PART_2_SOLUTIONS[usize::from(input[skip_1] ^ (input[skip_2] << 4))]
}

#[test]
fn test_die() {
    let mut dice_struct = Dice::new();
    let mut dice = 1_usize;
    for round in 0..1000 {
        let sum = dice + dice + 1 + dice + 2;
        dice += 3;
        dice %= 1000;
        assert_eq!(
            usize::from(dice_struct.next_sum_mod_10()) % 10,
            sum % 10,
            "failure in round {}",
            round
        );
    }
}

#[test]
fn test_part_2_example() {
    assert_eq!(
        WaysToGetScore::compute_ways_to_get_score(4, 8),
        444356092776315
    );
}
