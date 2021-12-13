use std::fmt;

struct Points(Vec<(u16, u16)>);

impl Points {
    fn fold(&mut self, fold: (u8, u16)) {
        let mut max = self.0.len();
        let mut i = 0;
        while i < max {
            let p = self.0[i];
            let folded = fold_point(p, fold);
            if folded != p {
                if self.0.contains(&folded) {
                    // Just remove the current point
                    self.0.remove(i);
                    max -= 1;
                } else {
                    // Else replace the current point
                    self.0[i] = folded;
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
    }
}

impl fmt::Display for Points {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_x = self.0.iter().map(|p| p.0).max().unwrap_or_default();
        let max_y = self.0.iter().map(|p| p.1).max().unwrap_or_default();
        for y in 0..=max_y {
            for x in 0..=max_x {
                if self.0.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn fold_point((x, y): (u16, u16), (axis, along): (u8, u16)) -> (u16, u16) {
    if axis == b'x' {
        (along - along.abs_diff(x), y)
    } else {
        (x, along - along.abs_diff(y))
    }
}

fn parse(input: &str) -> (Points, impl Iterator<Item = (u8, u16)> + '_) {
    let mut points: Vec<(u16, u16)> = Vec::new();
    let mut parts = input.split("\n\n");
    let point = parts.next().unwrap();
    let instructions = parts.next().unwrap();

    for line in point.lines() {
        let mut coords = line.split(',');
        let x = coords.next().unwrap().parse().unwrap();
        let y = coords.next().unwrap().parse().unwrap();
        points.push((x, y));
    }

    let folds = instructions.lines().map(|line| {
        let coord = &line["fold along ".len()..];
        let axis = coord.as_bytes()[0];
        let pos: u16 = coord[2..].parse().unwrap();
        (axis, pos)
    });

    (Points(points), folds)
}

pub fn part_1(input: &str) -> usize {
    let (mut points, mut folds) = parse(input);
    points.fold(folds.next().unwrap());
    points.0.len()
}

pub fn part_2(input: &str) -> String {
    let (mut points, folds) = parse(input);
    for fold in folds {
        points.fold(fold);
    }
    format!("{}", points)
}

#[test]
fn test_part_1_example() {
    let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
    assert_eq!(part_1(input), 17);
}

#[test]
fn test_part_2_example() {
    let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
    assert_eq!(
        part_2(input),
        "\
#####
#...#
#...#
#...#
#####
"
    );
}

#[test]
fn test_part_1_input() {
    let input = include_str!("../input/2021/day13.txt");
    assert_eq!(part_1(input), 631);
}

#[test]
fn test_part_2_input() {
    let input = include_str!("../input/2021/day13.txt");
    assert_eq!(
        part_2(input),
        "\
####.####.#....####...##..##..###..####
#....#....#....#.......#.#..#.#..#.#...
###..###..#....###.....#.#....#..#.###.
#....#....#....#.......#.#.##.###..#...
#....#....#....#....#..#.#..#.#.#..#...
####.#....####.#.....##...###.#..#.#...
"
    );
}
