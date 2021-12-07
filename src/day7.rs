fn parse_numbers(input: &str) -> impl Iterator<Item = usize> + '_ {
    let mut iter = input
        .as_bytes()
        .iter()
        .copied()
        .inspect(|&a| debug_assert!(a.is_ascii_digit() || a == b','));
    std::iter::from_fn(move || {
        iter.next().map(|x| {
            (&mut iter)
                .take_while(|b| *b != b',')
                .fold(usize::from(x - b'0'), |acc, digit| 10 * acc + usize::from(digit - b'0'))
        })
    })
}

/// Find the k'th item if `input` would be sorted, even if it isn't.
fn k_th(input: &mut [usize], k: usize) -> usize {
    let len = input.len();
    debug_assert!(len > 0);
    let pivot = input[0];
    let pivot_around = input.iter_mut().partition_in_place(|item| *item < pivot);
    if k < pivot_around {
        k_th(&mut input[..pivot_around], k)
    } else if k > pivot_around {
        k_th(&mut input[(pivot_around + 1)..], k - pivot_around - 1)
    } else {
        pivot
    }
}

pub fn part_1(input: &str) -> usize {
    let mut numbers: Vec<_> = parse_numbers(input).collect();
    let len = numbers.len();
    let mid_point = len / 2;
    let mid_point_value = k_th(&mut numbers, mid_point);
    // As a result of computing the kth point, the array is actually partitioned in < k, > k.
    numbers[len - mid_point..].iter().copied().sum::<usize>() - numbers[..mid_point].iter().copied().sum::<usize>()
}

pub fn part_2<const MAX_VALUE: usize>(input: &str) -> usize {
    let mut numbers: Vec<_> = parse_numbers(input).collect();
    // Guess a mu, find the derivative and increase mu accordingly
    let min = 0;
    let max = MAX_VALUE;
    while min != max {
        let mu_guess = (min + max) / 2;
        unimplemented!()
    }
    min
}

#[test]
fn test_part_1_example_input() {
    let input = "16,1,2,0,4,2,7,1,2,14";
    assert_eq!(part_1(input), 37);
}

#[test]
fn test_odd_even() {
    // Odd and even should both work, so care should be taken with the middle value
    assert_eq!(part_1("0,1,2"), 2);
    assert_eq!(part_1("0,1,1,2"), 2);
}

#[test]
fn test_part_2_example_input() {
    let input = "16,1,2,0,4,2,7,1,2,14";
    assert_eq!(part_2::<20>(input), 5);
}

#[test]
fn test_kth() {
    assert_eq!(k_th(&mut vec![2, 0, 1], 0), 0);
    assert_eq!(k_th(&mut vec![2, 0, 1], 1), 1);
    assert_eq!(k_th(&mut vec![2, 0, 1], 2), 2);
    assert_eq!(k_th(&mut vec![0,5,1,2,3,6,4], 3), 3)
}
