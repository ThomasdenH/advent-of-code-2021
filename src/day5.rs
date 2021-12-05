use std::{ops::{Mul, Sub, DivAssign, Add}, collections::{HashSet, HashMap}, str::Lines, cmp::Ordering};

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
    Diagonal
}

enum Overlap {
    None,
    Multi(Coordinate, Vector, i32)
}

impl Overlap {
    fn single(coord: Coordinate) -> Overlap {
        Overlap::Multi(coord, Vector(0, 0), 0)
    }
}

impl Iterator for Overlap {
    type Item = Coordinate;
    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            Overlap::None => None,
            Overlap::Multi(mut coord, vec, mut len) => {
                len -= 1;
                coord = coord + vec;
                if len == 0 {
                    *self = Overlap::None;
                } else {
                    *self = Overlap::Multi(coord, vec, len);
                }
                Some(coord)
            }
        }
    }
}

fn sort_tuple(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

impl LineSegment {
    /// Sort a line segment in the following way:
    /// If x stays constant, make y increasing.
    /// Otherwise, make x increasing.
    fn sorted(self) -> LineSegment {
        let inverted = LineSegment {
            from: self.to,
            to: self.from
        };
        match self.from.x.cmp(&self.to.x) {
            Ordering::Greater => inverted,
            Ordering::Equal => {
                match self.from.y.cmp(&self.to.y) {
                    Ordering::Greater => inverted,
                    Ordering::Equal | Ordering::Less => self
                }
            },
            Ordering::Less => self
        }
    }

    fn points_of_overlap(&self, other: LineSegment) -> Overlap {
        // Sort so all lines have increasing x, or y.
        let first = self.sorted();
        let second = other.sorted();
        first.points_of_overlap_sorted(second)
    }

    fn points_of_overlap_sorted(&self, other: LineSegment) -> Overlap {
        match (self.direction(), other.direction()) {
            (Direction::Horizontal, Direction::Horizontal) => {
                if self.from.y == other.from.y {
                    let x_start = self.from.x.max(other.from.x);
                    let x_end = self.to.x.min(other.to.x);
                    let dx = x_end - x_start;
                    if dx >= 0 {
                        Overlap::Multi(Coordinate { x: x_start, y: self.from.y }, Vector(1, 0), dx)
                    } else {
                        Overlap::None
                    }
                } else {
                    // Parallel lines
                    Overlap::None
                }
            },
            (Direction::Vertical, Direction::Vertical) => {
                if self.from.x == other.from.x {
                    let y_start = self.from.y.max(other.from.y);
                    let y_end = self.to.y.min(other.to.y);
                    let dy = y_end - y_start;
                    if dy >= 0 {
                        Overlap::Multi(Coordinate { x: self.from.x, y: y_start }, Vector(0, 1), dy)
                    } else {
                        Overlap::None
                    }
                } else {
                    // Parallel lines
                    Overlap::None
                }
            },
            (Direction::Horizontal, Direction::Vertical) => {
                other.points_of_overlap_sorted(*self)
            },
            (Direction::Vertical, Direction::Horizontal) => {
                if (self.from.y..=self.to.y).contains(&other.from.y) && (other.from.x..=other.to.x).contains(&self.from.x) {
                    Overlap::single(Coordinate { x: self.from.x, y: other.from.x })
                } else {
                    Overlap::None
                }
            },
            (Direction::Horizontal, Direction::Diagonal) => other.points_of_overlap_sorted(*self),
            (Direction::Vertical, Direction::Diagonal) => other.points_of_overlap_sorted(*self),
            (Direction::Diagonal, Direction::Diagonal) => {
                // Both line segments have possible directions: right & up and right & down.
                let self_dy = self.to.y - self.from.y;
                let other_dy = other.to.y - other.from.y;
                match ((self_dy, *self), (other_dy, other)) {
                    ((i32::MIN..=0, decreasing_y), (0.., increasing_y)) | ((0.., increasing_y), (i32::MIN..=0, decreasing_y)) => {
                        // The two lines potentially cross
                        // The formula for the increasing y is y = a + bx, b = 1.
                        if decreasing_y.from.y >= increasing_y.from.y + (decreasing_y.from.x - increasing_y.from.x)
                            && decreasing_y.to.y <= increasing_y.from.y + (decreasing_y.to.x - increasing_y.from.x)
                            && increasing_y.from.y <= decreasing_y.from.y - (increasing_y.from.x - decreasing_y.from.x)
                            && increasing_y.to.y >= decreasing_y.from.y - (increasing_y.to.x - decreasing_y.from.x) {
                                let x=  ((decreasing_y.from.y - decreasing_y.from.x) - (increasing_y.from.y - increasing_y.from.x)) / 2;
                                let y = (decreasing_y.from.y - decreasing_y.from.x) - x;
                                Overlap::single(Coordinate {x, y})
                            } else {
                                Overlap::None
                            }
                    },
                    _ => {
                        // Both are increasing or both are decreasing
                        let slope = Vector(1, self_dy.signum());
                        let a = self.from.y - self.from.x;
                        if other.from.y - other.from.x != a {
                            // Same constant, so crossing
                            let from_coordinate = if self.from.x > other.from.x {
                                self.from
                            } else {
                                other.from
                            };
                            let len = self.to.x.min(other.to.x) - from_coordinate.x;
                            if len >= 0 {
                                Overlap::Multi(from_coordinate, slope, len)
                            } else {
                                Overlap::None
                            }
                        } else {
                            // Parallel lines
                            Overlap::None
                        }
                    }
                }
            },
            (Direction::Diagonal, Direction::Horizontal) => {
                let a = self.from.y - self.from.x;
                let b = (self.to.y - self.from.y).signum();
                // self follows y = a + bx
                let other_y = other.from.y;
                if other.from.x <= self.to.x && other.to.x >= self.from.x // x should overlap
                && (a + b * other.from.x - other_y) != (a + b * other.to.x - other_y) // The diagonal should be on different sides of the horizontal line for its starting and end point
                {
                    let y = other_y;
                    let x = (y - a) * b; // (y - a) * b is the same as (y - a) / b for b in [1, -1]
                    Overlap::single(Coordinate {
                        x, y
                    })
                } else {
                    Overlap::None
                }
            }
            (Direction::Diagonal, Direction::Vertical) => {
                let a = self.from.y - self.from.x;
                let b = (self.to.y - self.from.y).signum();
                // self follows y = a + bx
                let other_x = other.from.x;
                if other_x >= self.from.x && other_x <= self.to.x // x should overlap
                && (a + b * other_x - other.from.y) != (a + b * other_x - other.to.y) // The diagonal should be on different sides of the horizontal line for its starting and end point
                {
                    let x = other_x;
                    let y = a + b * x;
                    Overlap::single(Coordinate {
                        x, y
                    })
                } else {
                    Overlap::None
                }
            }
        }
    }

    fn difference_vector(&self) -> Vector {
        Vector::from(self.to) - Vector::from(self.from)
    }

    fn direction(&self) -> Direction {
        if self.from.x == self.to.x {
            Direction::Vertical
        } else if self.from.y == self.to.y {
            Direction::Horizontal
        } else {
            Direction::Diagonal
        }
    }

    fn is_horizontal_or_vertical(&self) -> bool {
        matches!(self.direction(), Direction::Horizontal | Direction::Vertical)
    }

    fn points(self) -> impl Iterator<Item = Coordinate> {
        let mut diff = self.difference_vector();
        let gcd = diff.gcd();
        diff /= diff.gcd();
        (0..=gcd).map(move |i| self.from.add(diff * i))
    }
}

mod parse {
    use aoc_main::utils::Line;
    use nom::{sequence::{separated_pair, terminated}, character::complete::{digit1, newline}, IResult, combinator::{map, eof, iterator, opt, ParserIterator}, bytes::complete::tag, branch::alt, multi::separated_list1};

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

    pub(super) fn entire_input<'a>(input: &'a [u8]) -> ParserIterator<&'a [u8], nom::error::Error<&'a [u8]>, impl FnMut(&'a [u8]) -> IResult<&'a [u8], LineSegment>> {
        iterator(input, terminated(line_segment, opt(newline)))
    }
}


pub fn part_1(input: &str) -> usize {
    let points: HashMap<Coordinate, usize> = parse::entire_input(input.as_bytes())
        .filter(LineSegment::is_horizontal_or_vertical)
        .flat_map(LineSegment::points)
        .fold(HashMap::new(), |mut map, point| {
            map.entry(point).and_modify(|count| *count += 1).or_insert(1);
            map
        });
    points.values().filter(|val| **val > 1).count()
}

pub fn part_2(input: &str) -> usize {
    let points: HashMap<Coordinate, bool> = parse::entire_input(input.as_bytes())
        .flat_map(LineSegment::points)
        .fold(HashMap::new(), |mut map, point| {
            map.entry(point).and_modify(|seen| *seen = true).or_insert(false);
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
