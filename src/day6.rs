fn parse(input: &[u8]) -> impl Iterator<Item = u8> + '_ {
    input.iter()
        .step_by(2)
        .map(|fish| *fish & 0b1111)
}

const BUFFER_LENGTH: usize = 9;

#[derive(Default, Debug)]
struct FishBuffer {
    fish_of_ages: [usize; BUFFER_LENGTH],
    offset: usize
}

impl FromIterator<u8> for FishBuffer {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut buffer = FishBuffer::default();
        for fish_age in iter.into_iter() {
            buffer.add_fish_at_age(usize::from(fish_age), 1);
        }
        buffer
    }
}

impl FishBuffer {
    fn advance(&mut self) {
        let fish_at_zero = self.fish_at_age(0);
        self.offset += 1;
        self.offset %= BUFFER_LENGTH;
        self.add_fish_at_age(6, fish_at_zero);
    }

    fn add_fish_at_age(&mut self, age: usize, amount: usize) {
        self.fish_of_ages[(age + self.offset) % BUFFER_LENGTH] += amount;
    }

    fn fish_at_age(&mut self, age: usize) -> usize {
        self.fish_of_ages[(age + self.offset) % BUFFER_LENGTH]
    }

    fn count_fish(&self) -> usize {
        self.fish_of_ages.iter().sum()
    }
}

pub fn fish_at_generation(input: &str, gen: usize) -> usize {
    let mut buffer: FishBuffer = parse(input.as_bytes()).collect();
    for _ in 0..gen {
        buffer.advance();
    }
    buffer.count_fish()
}

pub fn part_1(input: &str) -> usize {
    fish_at_generation(input, 80)
}

pub fn part_2(input: &str) -> usize {
    fish_at_generation(input, 256)
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
