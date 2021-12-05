use std::{
    cmp::Ordering,
    collections::HashMap,
    hash::{Hash, Hasher, BuildHasher},
    iter::successors,
    ops::Add,
};

const A: u64 = 5306621708036795348;
const B: u64 = 2147483659;

#[derive(Default)]
struct BuildSpecialHasher {
}

impl BuildHasher for BuildSpecialHasher {
    type Hasher = SpecialHasher;
    fn build_hasher(&self) -> Self::Hasher {
        SpecialHasher::default()
    }
}

#[derive(Default)]
struct SpecialHasher {
    hash: u64,
}

impl Hasher for SpecialHasher {
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write(&mut self, bytes: &[u8]) {
        self.hash = bytes.iter().fold(self.hash, |mut h, next_byte| {
            h ^= u64::from(*next_byte);
            h *= B;
            h += A;
            h
        });
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct Coordinate {
    x: u16,
    y: u16,
}

impl From<(u16, u16)> for Coordinate {
    fn from((x, y): (u16, u16)) -> Self {
        Coordinate { x, y }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Sign {
    Neg,
    Pos,
    Nil,
}

impl From<Ordering> for Sign {
    fn from(ordering: Ordering) -> Sign {
        match ordering {
            Ordering::Greater => Sign::Pos,
            Ordering::Equal => Sign::Nil,
            Ordering::Less => Sign::Neg,
        }
    }
}

impl Add<Sign> for u16 {
    type Output = u16;
    fn add(self, rhs: Sign) -> Self::Output {
        match rhs {
            Sign::Pos => self + 1,
            Sign::Neg => self - 1,
            Sign::Nil => self,
        }
    }
}

#[derive(Clone, Copy)]
struct Vector(Sign, Sign);

impl Add<Vector> for Coordinate {
    type Output = Coordinate;
    fn add(self, rhs: Vector) -> Self::Output {
        Coordinate {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct LineSegment {
    from: Coordinate,
    to: Coordinate,
}

impl From<(Coordinate, Coordinate)> for LineSegment {
    fn from((from, to): (Coordinate, Coordinate)) -> Self {
        LineSegment { from, to }
    }
}

fn abs_diff(a: u16, b: u16) -> u16 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

impl LineSegment {
    fn is_horizontal_or_vertical(&self) -> bool {
        self.from.x == self.to.x || self.from.y == self.to.y
    }

    fn difference_vector(&self) -> Vector {
        Vector(
            self.to.x.cmp(&self.from.x).into(),
            self.to.y.cmp(&self.from.y).into(),
        )
    }

    fn len(&self) -> u16 {
        if self.from.x == self.to.x {
            abs_diff(self.from.y, self.to.y)
        } else {
            abs_diff(self.from.x, self.to.x)
        }
    }

    fn points(self) -> impl Iterator<Item = Coordinate> {
        let direction_vector = self.difference_vector();
        let length = self.len();
        successors(Some(self.from), move |point| {
            Some(*point + direction_vector)
        })
        .take(usize::from(length))
    }
}

mod parse {
    use nom::{
        bytes::complete::tag,
        character::complete::{digit1, newline},
        combinator::{iterator, map, opt, ParserIterator},
        sequence::{separated_pair, terminated},
        IResult,
    };

    use super::{Coordinate, LineSegment};

    fn number(input: &[u8]) -> IResult<&[u8], u16> {
        map(digit1, |number_str: &[u8]| {
            number_str
                .iter()
                .map(|d| u16::from(d & 0b1111))
                .fold(0, |acc, d| 10 * acc + d)
        })(input)
    }

    fn coordinate(input: &[u8]) -> IResult<&[u8], Coordinate> {
        map(separated_pair(number, tag(","), number), Coordinate::from)(input)
    }

    fn line_segment(input: &[u8]) -> IResult<&[u8], LineSegment> {
        map(
            separated_pair(coordinate, tag(" -> "), coordinate),
            LineSegment::from,
        )(input)
    }

    pub(super) fn entire_input<'a>(
        input: &'a [u8],
    ) -> ParserIterator<
        &'a [u8],
        nom::error::Error<&'a [u8]>,
        impl FnMut(&'a [u8]) -> IResult<&'a [u8], LineSegment>,
    > {
        iterator(input, terminated(line_segment, opt(newline)))
    }
}

pub fn part_1(input: &str) -> usize {
    let points: HashMap<Coordinate, bool, _> = HashMap::with_hasher(BuildSpecialHasher::default());
    let points = parse::entire_input(input.as_bytes())
        .filter(LineSegment::is_horizontal_or_vertical)
        .flat_map(LineSegment::points)
        .fold(points, |mut map, point| {
            map.entry(point)
                .and_modify(|seen| *seen = true)
                .or_insert(false);
            map
        });
    points.values().filter(|val| **val).count()
}

pub fn part_2(input: &str) -> usize {
    let points:  HashMap<Coordinate, bool, _> = HashMap::with_hasher(BuildSpecialHasher::default());
    let points = parse::entire_input(input.as_bytes())
        .flat_map(LineSegment::points)
        .fold(points, |mut map, point| {
            map.entry(point)
                .and_modify(|seen| *seen = true)
                .or_insert(false);
            map
        });
    points.values().filter(|val| **val).count()
}

#[test]
fn test_part_1_example() {
    let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    assert_eq!(part_1(input), 5);
}

#[test]
fn test_part_2_example() {
    let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    assert_eq!(part_2(input), 12);
}

#[test]
fn test_part_1_input() {
    let input = include_str!("../input/2021/day5.txt");
    assert_eq!(part_1(input), 7674);
}

#[test]
fn test_part_2_input() {
    let input = include_str!("../input/2021/day5.txt");
    assert_eq!(part_2(input), 20898);
}
