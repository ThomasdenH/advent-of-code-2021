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

const MIN_THREE_DICE_RESULT: usize = 3;
const MAX_THREE_DICE_RESULT: usize = 9;

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
const POSITIONS: usize = 10;

#[derive(Default)]
struct Table([[[[usize; POSITIONS]; WINNING_SCORE]; POSITIONS]; WINNING_SCORE]);

impl Table {
    const fn empty() -> Self {
        Table([[[[0; POSITIONS]; WINNING_SCORE]; POSITIONS]; WINNING_SCORE])
    }

    const fn from_starting_position(own: usize, other: usize) -> Self {
        let mut t = [[[[0; POSITIONS]; WINNING_SCORE]; POSITIONS]; WINNING_SCORE];
        t[0][own][0][other] = 1;
        Table(t)
    }

    fn do_turn(&mut self, own_score: usize, other_score: usize, own_position: usize, other_position: usize, other_player: &Table) -> usize {
        assert!(own_score < WINNING_SCORE);
        assert!(other_score < WINNING_SCORE);
        assert!(own_position < POSITIONS);
        assert!(other_position < POSITIONS);
        let mut possibilities = 0;
        let mut dice = MIN_THREE_DICE_RESULT;
        while dice <= MAX_THREE_DICE_RESULT {
            // The current player position can be derived from their score.
            let own_position_new = (own_position + dice) % POSITIONS;
            let own_score_new = own_score + own_position_new + 1;
            let frequency = THREE_DICE_OUTCOMES[dice];
            if own_score_new >= WINNING_SCORE {
                possibilities += other_player.0[other_score][other_position][own_score][own_position] * frequency;
            } else {
                assert!(own_score_new < WINNING_SCORE);
                assert!(own_position_new < POSITIONS);
                self.0[own_score_new][own_position_new][other_score][other_position] += other_player.0[other_score][other_position][own_score][own_position] * frequency;
            }
            dice += 1;
        }
        possibilities
    }
}

fn compute_possibilities_to_reach_score(start_p1: usize, start_p2: usize) -> usize {
    let mut after_turn_p2 = Table::from_starting_position(start_p2 - 1, start_p1 - 1);
    let mut after_turn_p1 = Table::empty();
    let mut possibilities_p1 = 0;
    let mut possibilities_p2 = 0;
    let mut p1_score = 0;
    while p1_score < WINNING_SCORE {
        let mut p2_score = 0;
        while p2_score < WINNING_SCORE {
            let mut p1_position = 0;
            while p1_position < POSITIONS {
                let mut p2_position = 0;
                while p2_position < POSITIONS {
                    possibilities_p1 += after_turn_p1.do_turn(p1_score, p2_score, p1_position, p2_position, &after_turn_p2);
                    possibilities_p2 += after_turn_p2.do_turn(p2_score, p1_score, p2_position, p1_position, &after_turn_p1);
                    p2_position += 1;
                }
                p1_position += 1;
            }
            p2_score += 1;
        }
        p1_score += 1;
    }
    possibilities_p1.max(possibilities_p2)
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
            // solutions[(a ^ (b << 4)) as usize] = compute_possibilities_to_reach_score((a - b'0') as usize, (b - b'0') as usize);
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
        compute_possibilities_to_reach_score(4, 8),
        444356092776315
    );
}
