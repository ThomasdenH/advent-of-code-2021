#[cfg(test)]
use std::collections::HashSet;

use itertools::Itertools;

fn parse(input: &str) -> (&[u8], impl Iterator<Item = (&[u8], u8)>) {
    let mut split = input.split("\n\n");
    let polymer = split.next().unwrap().as_bytes();
    let iter = split.next().unwrap().lines().map(|l| {
        let l = l.as_bytes();
        let i = &l[..2];
        let o = l[6];
        (i, o)
    });
    (polymer, iter)
}

const MAX_INDEX: usize = 26 * 26;

fn pair_to_index(pair: &[u8]) -> usize {
    (usize::from(pair[0]) * 26 + usize::from(pair[1]))
        - (usize::from(b'A') * 26 + usize::from(b'A'))
}

fn solve(input: &str, rounds: usize) -> usize {
    let mut frequencies = [0usize; MAX_INDEX];
    let (polymer, replacings) = parse(input);
    let first_polymer_letter = polymer[0];
    for (pair_left, pair_right) in polymer.iter().tuple_windows() {
        frequencies[pair_to_index(&[*pair_left, *pair_right])] += 1;
    }
    let replacings: Vec<_> = replacings
        .map(|(i, o)| {
            (
                pair_to_index(i),
                pair_to_index(&[i[0], o]),
                pair_to_index(&[o, i[1]]),
            )
        })
        .collect();
    for _ in 0..rounds {
        let mut new_frequencies = [0; MAX_INDEX];
        for (from, to_1, to_2) in replacings.iter().copied() {
            new_frequencies[to_1] += frequencies[from];
            new_frequencies[to_2] += frequencies[from];
        }
        frequencies = new_frequencies;
    }

    // Count the first letter, and for the other pairs count only the second letter
    let mut actual_letter_frequencies = [0_usize; 26];
    actual_letter_frequencies[usize::from(first_polymer_letter - b'A')] = 1;
    for letter in b'A'..=b'Z' {
        for other_letter in b'A'..=b'Z' {
            actual_letter_frequencies[usize::from(other_letter - b'A')] +=
                frequencies[pair_to_index(&[letter, other_letter])];
        }
    }
    actual_letter_frequencies.iter().max().unwrap()
        - actual_letter_frequencies
            .iter()
            .filter(|l| **l != 0)
            .min()
            .unwrap()
}

pub fn part_1(input: &str) -> usize {
    solve(input, 10)
}

pub fn part_2(input: &str) -> usize {
    solve(input, 40)
}

#[test]
fn test_pair_to_index() {
    let mut set = HashSet::new();
    for letter in b'A'..=b'Z' {
        for other_letter in b'A'..=b'Z' {
            let index = pair_to_index(&[letter, other_letter]);
            // No collisions
            assert!(!set.contains(&index));
            // Shouldn't exceed array bounds
            assert!(index < MAX_INDEX);
            set.insert(index);
        }
    }
}

#[test]
fn test_part_1_example() {
    let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    assert_eq!(part_1(input), 1588);
}

#[test]
fn test_part_1_input() {
    let input = include_str!("../input/2021/day14.txt");
    assert_eq!(part_1(input), 2509);
}

#[test]
fn test_part_2_input() {
    let input = include_str!("../input/2021/day14.txt");
    assert_eq!(part_2(input), 2827627697643);
}
