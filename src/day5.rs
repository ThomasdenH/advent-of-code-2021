use std::{ops::{Mul, Sub, DivAssign, Add}, collections::HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coordinate {
    x: i32,
    y: i32
}

impl From<(i32, i32)> for Coordinate {
    fn from((x, y): (i32, i32)) -> Self {
        Coordinate {x,y}
    }
}

impl Add<Vector> for Coordinate {
    type Output = Coordinate;
    fn add(self, Vector(dx, dy): Vector) -> Self::Output {
        Coordinate {
            x: self.x + dx,
            y: self.y + dy
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Vector(i32, i32);

impl Vector {
    fn gcd(&self) -> i32 {
        if self.0 < 0 {
            Vector(-self.0, self.1).gcd()
        } else if self.1 < 0 {
            Vector(self.0, -self.1).gcd()
        } else if self.0 < self.1 {
            Vector(self.1, self.0).gcd()
        } else if self.1 == 0 {
            self.0
        } else if self.0 == 0 {
            self.1
        } else {
            Vector(self.1, self.0 % self.1).gcd()
        }
    }
}

impl From<Coordinate> for Vector {
    fn from(Coordinate { x, y }: Coordinate) -> Self {
        Vector(x, y)
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Vector) -> Self {
        Vector(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl DivAssign<i32> for Vector {
    fn div_assign(&mut self, rhs: i32) {
        *self = Vector(self.0 / rhs, self.1 / rhs)
    }
}

impl Mul<i32> for Vector {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Vector(self.0 * rhs, self.1 * rhs)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct LineSegment {
    from: Coordinate,
    to: Coordinate
}

impl From<(Coordinate, Coordinate)> for LineSegment {
    fn from((from, to): (Coordinate, Coordinate)) -> Self {
        LineSegment { from, to}
    }
}

enum Direction {
    Horizontal,
    Vertical,
    Other
}

impl LineSegment {
    fn difference_vector(&self) -> Vector {
        Vector::from(self.to) - Vector::from(self.from)
    }

    fn direction(&self) -> Direction {
        if self.from.x == self.to.x {
            Direction::Vertical
        } else if self.from.y == self.to.y {
            Direction::Horizontal
        } else {
            Direction::Other
        }
    }

    fn is_horizontal_or_vertical(&self) -> bool {
        matches!(self.direction(), Direction::Horizontal | Direction::Vertical)
    }

    fn points(&self) -> impl Iterator<Item = Coordinate> + '_ {
        let mut diff = self.difference_vector();
        let gcd = diff.gcd();
        diff /= diff.gcd();
        (0..=gcd).map(move |i| self.from.add(diff * i))
    }
}

mod parse {
    use nom::{sequence::{separated_pair}, character::complete::{digit1, newline}, IResult, combinator::{map, eof}, bytes::complete::tag, branch::alt, multi::separated_list1};

    use super::{Coordinate, LineSegment};

    fn number(input: &[u8]) -> IResult<&[u8], i32> {
        map(digit1, |number_str: &[u8]| number_str.iter().map(|d| i32::from(d & 0b1111)).fold(0, |acc, d| 10 * acc + d))(input)
    }
    
    fn coordinate(input: &[u8]) -> IResult<&[u8], Coordinate> {
        map(separated_pair(number, tag(","), number), Coordinate::from)(input)
    }
    
    fn line_segment(input: &[u8]) -> IResult<&[u8], LineSegment> {
        map(separated_pair(coordinate, tag(" -> "), coordinate), LineSegment::from)(input)
    }

    pub(super) fn entire_input(input: &[u8]) -> IResult<&[u8], Vec<LineSegment>> {
        separated_list1(newline, line_segment)(input)
    }
}


pub fn part_1(input: &str) -> usize {
    let (_, mut segments) = parse::entire_input(input.as_bytes()).unwrap();
    segments.retain(LineSegment::is_horizontal_or_vertical);

    let mut points_of_overlap = HashSet::new();

    for (i, segment) in segments.iter().enumerate() {
        for other_segment in segments.iter().skip(i + 1) {
            for point_1 in segment.points() {
                for point_2 in other_segment.points() {
                    if point_1 == point_2 {
                        points_of_overlap.insert(point_1);
                    }
                }
            }
        }
    }
    points_of_overlap.len()
}


pub fn part_2(input: &str) -> usize {
    let (_, mut segments) = parse::entire_input(input.as_bytes()).unwrap();
    
    let mut points_of_overlap = HashSet::new();

    for (i, segment) in segments.iter().enumerate() {
        for other_segment in segments.iter().skip(i + 1) {
            for point_1 in segment.points() {
                for point_2 in other_segment.points() {
                    if point_1 == point_2 {
                        points_of_overlap.insert(point_1);
                    }
                }
            }
        }
    }
    points_of_overlap.len()
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
    assert_eq!(part_1(input), 69579);
}

#[test]
fn test_part_2_input() {
    let input = include_str!("../input/2021/day5.txt");
    assert_eq!(part_2(input), 14877);
}
