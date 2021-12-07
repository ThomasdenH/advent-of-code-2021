use std::cmp::Ordering;

use itertools::Itertools;

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
                .fold(usize::from(x - b'0'), |acc, digit| {
                    10 * acc + usize::from(digit - b'0')
                })
        })
    })
}

/// Find the k'th item if `input` would be sorted, even if it isn't.
fn k_th(input: &mut [usize], k: usize) -> usize {
    debug_assert!(!input.is_empty());
    let pivot = input[0];
    let remaining_input = &mut input[1..];
    let order_of_pivot = remaining_input
        .iter_mut()
        .partition_in_place(|item| *item < pivot);
    match k.cmp(&order_of_pivot) {
        Ordering::Greater => k_th(
            &mut remaining_input[order_of_pivot..],
            k - 1 - order_of_pivot,
        ),
        Ordering::Equal => pivot,
        Ordering::Less => k_th(&mut remaining_input[..order_of_pivot], k),
    }
}

pub fn part_1(input: &str) -> usize {
    let mut numbers: Vec<_> = parse_numbers(input).collect();
    let mid_point = numbers.len() / 2;
    let mid_point_value = k_th(&mut numbers, mid_point);
    // As a result of computing the kth point, the array is actually partitioned in < k, > k.
    // However, the naive way is faster
    numbers
        .into_iter()
        .map(|x| {
            if x > mid_point_value {
                x - mid_point_value
            } else {
                mid_point_value - x
            }
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    part_2_sized::<2000>(input)
}

fn part_2_sized<const MAX_VALUE: usize>(input: &str) -> usize {
    let mut frequency_table = vec![0; MAX_VALUE];
    let mut count = 0;
    let mut sum = 0;
    let mut sum_of_squares = 0;
    for number in parse_numbers(input) {
        debug_assert!(number < MAX_VALUE, "number exceeds maximum value - 1!");
        frequency_table[number] += 1;
        count += 1;
        sum += number;
        sum_of_squares += number * number;
    }
    // For mu = 0
    let mut numbers_less_than_eq_mu = 0;
    let mut fuel_at_mu = (sum + sum_of_squares) / 2;
    let mut mu_times_count = 0;
    for numbers_eq_to_mu in frequency_table {
        // mu is equal to the iteration index but not actually used itself
        // (0..)

        // Compute for mu
        numbers_less_than_eq_mu += numbers_eq_to_mu;

        let fuel_increase = mu_times_count + numbers_less_than_eq_mu;
        let fuel_decrease = sum;

        if fuel_increase > fuel_decrease {
            break;
        }

        // For mu + 1
        mu_times_count += count;

        fuel_at_mu += fuel_increase;
        fuel_at_mu -= fuel_decrease;
    }
    fuel_at_mu
}

#[test]
fn test_part_1_example_input() {
    let input = "16,1,2,0,4,2,7,1,2,14";
    assert_eq!(part_1(input), 37);
}

#[test]
fn test_part_1_duplicate_midpoint() {
    let input = "0,1,1,1,1,1,1,1,2,3";
    assert_eq!(part_1(input), 4);
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
    assert_eq!(part_2_sized::<20>(input), 168);
}

#[test]
fn test_kth() {
    assert_eq!(k_th(&mut vec![2, 0, 1], 0), 0);
    assert_eq!(k_th(&mut vec![2, 0, 1], 1), 1);
    assert_eq!(k_th(&mut vec![2, 0, 1], 2), 2);
    assert_eq!(k_th(&mut vec![0, 5, 1, 2, 3, 6, 4], 3), 3)
}

#[test]
fn test_part_1_input() {
    let input = include_str!("../input/2021/day7.txt");
    assert_eq!(part_1(input), 339321);
}

#[test]
fn test_part_2_input() {
    let input = include_str!("../input/2021/day7.txt");
    assert_eq!(part_2(input), 95476244);
}
