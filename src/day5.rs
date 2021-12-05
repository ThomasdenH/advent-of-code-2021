#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coordinate {
    x: i16,
    y: i16,
}

impl From<(i16, i16)> for Coordinate {
    fn from((x, y): (i16, i16)) -> Self {
        Coordinate { x, y }
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

impl LineSegment {
    fn is_horizontal_or_vertical(&self) -> bool {
        self.from.x == self.to.x || self.from.y == self.to.y
    }

    fn points(self) -> impl Iterator<Item = Coordinate> {
        let len = ((self.to.x - self.from.x).abs() | (self.to.y - self.from.y).abs()) as usize;
        let dx = (self.to.x - self.from.x).signum();
        let dy = (self.to.y - self.from.y).signum();
        std::iter::successors(Some(self.from), move |Coordinate { x, y }| {
            Some(Coordinate {
                x: x + dx,
                y: y + dy,
            })
        })
        .take(len + 1)
    }
}

mod parse {
    use super::{Coordinate, LineSegment};

    fn number(input: &[u8]) -> (&[u8], i16) {
        let (first, mut input) = input.split_first().unwrap();
        debug_assert!(first.is_ascii_digit());
        let mut acc = i16::from(first & 0b1111);
        while let Some((b, new_input)) = input.split_first() {
            if !b.is_ascii_digit() {
                break;
            }
            acc *= 10;
            acc += i16::from(b & 0b1111);
            input = new_input;
        }
        (input, acc)
    }

    fn coordinate(input: &[u8]) -> (&[u8], Coordinate) {
        let (input, x) = number(input);
        let (comma, input) = input.split_first().unwrap();
        debug_assert_eq!(*comma, b',');
        let (input, y) = number(input);
        (input, Coordinate { x, y })
    }

    fn line_segment(input: &[u8]) -> (&[u8], LineSegment) {
        let (input, from) = coordinate(input);
        let (arrow, input) = input.split_at(4);
        debug_assert_eq!(arrow, b" -> ");
        let (input, to) = coordinate(input);
        (input, LineSegment::from((from, to)))
    }

    pub(super) fn entire_input(input: &[u8]) -> impl Iterator<Item = LineSegment> + '_ {
        let (input, segment) = line_segment(input);
        std::iter::successors(Some((input, segment)), |(mut input, _segment)| {
            if let Some((newline, new_input)) = input.split_first() {
                debug_assert_eq!(*newline, b'\n');
                input = new_input;
                Some(line_segment(input))
            } else {
                None
            }
        })
        .map(|(_, segment)| segment)
    }
}

pub fn part_1(input: &str) -> usize {
    let mut seen = vec![u8::MAX - 1; 1024 * 1024];
    parse::entire_input(input.as_bytes())
        .filter(LineSegment::is_horizontal_or_vertical)
        .flat_map(LineSegment::points)
        .map(|Coordinate { x, y }| (x as usize) << 10 | (y as usize))
        .filter(|&index| {
            seen[index] = seen[index].wrapping_add(1);
            seen[index] == 0
        })
        .count()
}

pub fn part_2(input: &str) -> usize {
    let mut seen = vec![u8::MAX - 1; 1024 * 1024];
    parse::entire_input(input.as_bytes())
        .flat_map(LineSegment::points)
        .map(|Coordinate { x, y }| (x as usize) << 10 | (y as usize))
        .filter(|&index| {
            seen[index] = seen[index].wrapping_add(1);
            seen[index] == 0
        })
        .count()
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
