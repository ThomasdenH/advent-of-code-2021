// Used for day 7
#![feature(iter_partition_in_place)]
#![feature(int_abs_diff)]
// Used for day 6
#![feature(const_eval_limit)]
#![const_eval_limit = "0"]

// Used for day 21
#![feature(const_mut_refs)]

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day2;
mod day21;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day23;

aoc_main::main! {
    year 2021;
    day1 => part_1, part_2;
    day2 => part_1, part_2;
    day3 => part_1, part_2, part_1_code_golf;
    day4 => part_1, part_2;
    day5 => part_1, part_2;
    day6 => part_1, part_2;
    day7 => part_1, part_2_incrementing_mu, part_2_mean;
    day8 => part_1, part_2, part_2_frequency_table;
    day9 => part_1, part_2;
    day10 => part_1, part_2;
    day11 => part_1, part_2;
    day12 => part_1, part_2;
    day13 => part_1, part_2;
    day14 => part_1, part_2;
    day15 => part_1, part_2;
    day16 => part_1, part_2;
    day17 => part_1, part_2;
    day21 => part_1, part_2;
    day23 => part_1;
}
