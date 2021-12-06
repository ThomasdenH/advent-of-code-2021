use std::{fmt::Debug};

use nom::AsBytes;

#[derive(Copy, Clone, PartialEq, Eq)]
struct Matrix<const HEIGHT: usize, const WIDTH: usize> {
    m: [[usize; WIDTH]; HEIGHT]
}

impl<const I: usize, const K: usize> Debug for Matrix<I, K>  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..I {
            write!(f, "[");
            for k in 0..K {
                if k != 0 {
                    write!(f, "\t");
                }
                write!(f,"{}", self.m[i][k]);
            }
            writeln!(f,"]");
        }
        Ok(())
    }
}

impl<const I: usize, const K: usize> Matrix<I, K> {
    const fn mul<const J: usize>(&self, other: Matrix<K, J>) -> Matrix<I, J> {
        let mut m: Matrix<I, J> = Matrix { m: [[0; J]; I]};
        let mut i = I;
        loop {
            i -= 1;
            let mut j = J;
            loop {
                j -= 1;
                let mut k = K;
                loop {
                    k -= 1;
                    m.m[i][j] += self.m[i][k] * other.m[k][j];
                    if k == 0 {
                        break;
                    }
                }
                if j == 0 {
                    break;
                }
            }
            if i == 0 {
                break;
            }
        }
        m
    }
}

impl<const I: usize> Matrix<I, I> {
    const fn exp(&self, mut e: usize) -> Matrix<I, I> {
        let mut result = *self;
        loop {
            e -= 1;
            if e == 0 {
                return result;
            }
            result = result.mul(*self);
        }
    }
}

impl Matrix<1, 1> {
    const fn val(&self) -> usize {
        self.m[0][0]
    }
}

const MATRIX_ONE: Matrix<BUFFER_LENGTH, BUFFER_LENGTH> = Matrix { m: [
    [0, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0]
]};

const BUFFER_LENGTH: usize = 9;
const SUM_MATRIX: Matrix<1, BUFFER_LENGTH> = Matrix { m: [[1; BUFFER_LENGTH]] };
/// In order to avoid a subtraction when parsing, don't subtract b'0'.
/// Instead, use a transposition matrix, which can be compiled into the multiplication.
const TRANSPOSITION: Matrix<BUFFER_LENGTH, BUFFER_LENGTH> = Matrix{ m: [
    [0, 0, 0, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 0, 0, 0],
]};
const MATRIX_80: Matrix<1, BUFFER_LENGTH> = SUM_MATRIX.mul(MATRIX_ONE.exp(80).mul(TRANSPOSITION));
const MATRIX_256: Matrix<1, BUFFER_LENGTH> = SUM_MATRIX.mul(MATRIX_ONE.exp(256).mul(TRANSPOSITION));


fn parse(input: &[u8]) -> impl Iterator<Item = u8> + '_ {
    input.iter()
        .step_by(2)
        .copied()
}

fn matrix_from_input(input: &[u8]) -> Matrix<BUFFER_LENGTH, 1> {
    let mut m = [[0]; BUFFER_LENGTH];
    for f in parse(input.as_bytes()) {
        m[usize::from(f) % BUFFER_LENGTH][0] += 1;
    }
    Matrix { m }
}

pub fn part_1(input: &str) -> usize {
    // Instead of multiplying A * b, do b^T * A^T.
    MATRIX_80.mul(matrix_from_input(input.as_bytes())).val()
}

pub fn part_2(input: &str) -> usize {
    MATRIX_256.mul(matrix_from_input(input.as_bytes())).val()
}

#[test]
fn test_example_part_1() {
    let input = "3,4,3,1,2";
    assert_eq!(part_1(input), 5934);
}

#[test]
fn test_part_1_input() {
    let input = include_str!("../input/2021/day6.txt");
    assert_eq!(part_1(input), 372300);
}

#[test]
fn test_example_part_2() {
    let input = "3,4,3,1,2";
    assert_eq!(part_2(input), 26984457539);
}

#[test]
fn test_part_2_input() {
    let input = include_str!("../input/2021/day6.txt");
    assert_eq!(part_2(input), 1675781200288);
}

#[test]
fn tests_with_matrices() {
    let matrix = |s: &str| TRANSPOSITION.mul(matrix_from_input(s.as_bytes()));
    let start_matrix = matrix("3,4,3,1,2");
    assert_eq!(start_matrix, Matrix { m: [[0], [1], [1], [2], [1], [0], [0], [0], [0]]});
    assert_eq!(MATRIX_ONE.mul(start_matrix), matrix("2,3,2,0,1"));
    dbg!(matrix("2,3,2,0,1"));
    dbg!(MATRIX_ONE);
    dbg!(MATRIX_ONE.mul(matrix("2,3,2,0,1")));
    assert_eq!(MATRIX_ONE.exp(2).mul(start_matrix), matrix("1,2,1,6,0,8"));
    assert_eq!(MATRIX_ONE.exp(3).mul(start_matrix), matrix("0,1,0,5,6,7,8"));
    assert_eq!(MATRIX_ONE.exp(4).mul(start_matrix), matrix("6,0,6,4,5,6,7,8,8"));
}
