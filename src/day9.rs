use std::collections::HashMap;

pub fn part_1_generic<const LINE_SIZE: usize>(input: &str) -> usize {
    debug_assert_eq!(input.find('\n'), Some(LINE_SIZE - 1));
    let input = input.as_bytes();
    let mut sum = 0;
    let height = input.len() / LINE_SIZE + 1;
    for y in 0..height {
        for x in 0..(LINE_SIZE - 1) {
            let index = y * LINE_SIZE + x;
            let digit = input[index];
            if (x == 0 || digit < input[index - 1])
                && (x == LINE_SIZE - 2 || digit < input[index + 1])
                && (y == 0 || digit < input[index - LINE_SIZE])
                && (y == height - 1 || digit < input[index + LINE_SIZE]) {
                    sum += usize::from(digit - b'0' + 1);
            }
        }
    }
    sum
}

pub fn part_1(input: &str) -> usize {
    const LINE_SIZE: usize = 101;
    part_1_generic::<LINE_SIZE>(input)
}

fn part_2_generic<const LINE_SIZE: usize>(input: &str) -> usize {
    // A map to basin number/size
    // Really need to comput size of areas != 9.
    let mut basin_size = HashMap::new();
    let mut next_basin = 0;
    let mut basin_above = [0; LINE_SIZE];
    let mut current_basin = [0; LINE_SIZE];
    let mut currently_in_basin = None;
    for line in input.as_bytes().chunks(LINE_SIZE) {
        for (x, b) in line.into_iter().enumerate() {
            match b {
                b'9' | b'\n' => {
                    // No basin
                    currently_in_basin = None;
                },
                _ => {
                    // There are a couple of situations:
                    // - Either we are already working in a basin from the left
                    //   and there is a 9 above. Then just keep working in the same basin.
                    // - Or, we are not yet working in a basin from the left but there is a basin above.
                    // - Or, we are working in a basin from the left and there is a basin above. These may
                    //   be the same but it might be that they should be merged.
                    /* for i in basin_above.iter() {
                        print!("{}", i);
                    }
                    println!();

                    for i in current_basin.iter() {
                        print!("{}", i);
                    }
                    println!();

                    print!("{}", std::str::from_utf8(line).unwrap());

                    for i in (0..LINE_SIZE) {
                        if i == x {
                            print!("^");
                        } else {
                            print!(" ");
                        }
                    } */
                    // println!();

                    let basin_above = basin_above[x];
                    let basin = if let Some(basin_to_the_left) = currently_in_basin {
                        if basin_above == 0 {
                            // println!(" -> To the left!");
                            // 99 
                            // 1X -> 1
                            basin_to_the_left
                        } else if basin_above == basin_to_the_left {
                            // println!(" -> Identical!");
                            // Not a problem
                            // 92
                            // 2X -> 2
                            basin_above
                        } else {
                            // println!(" -> Merge!");
                            // Trouble!
                            // 91
                            // 2X -> Merge!

                            // Take the one above and replace all to the left
                            for previous in (0..x).rev() {
                                // dbg!(x, previous, current_basin[previous], basin_to_the_left);
                                if current_basin[previous] != basin_to_the_left {
                                    break;
                                }
                                current_basin[previous] = basin_above
                            }

                            // Add the size of the basin to the left to that on top
                            *basin_size.entry(basin_above).or_default() += *basin_size.entry(basin_to_the_left).or_default();

                            basin_above
                        }
                    } else if basin_above == 0 {
                        // println!(" -> New basin!");
                        // Create new basin
                        // 99
                        // 9X -> new index
                        next_basin += 1;
                        next_basin
                    } else {
                        // println!(" -> Above!");
                        // 91
                        // 9X -> 1
                        basin_above
                    };

                    current_basin[x] = basin;
                    *basin_size.entry(basin).or_default() += 1;
                    currently_in_basin = Some(basin);

                    // println!("(New basin size: {})", basin_size.get(&basin).unwrap());
                    // println!();
                }
            }
        }
        basin_above = current_basin;
        current_basin = [0; LINE_SIZE];
    }
    let size = basin_size.len();
    basin_size.values()
        .copied()
        .collect::<Vec<usize>>()
        .select_nth_unstable(size.saturating_sub(4))
        .2.iter().copied()
        .product()

}

pub fn part_2(input: &str) -> usize {
    const LINE_SIZE: usize = 101;
    part_2_generic::<LINE_SIZE>(input)
}

#[test]
fn test_part_1_example() {
    let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
    assert_eq!(part_1_generic::<11>(input), 15);
}

#[test]
fn test_part_1_input() {
    let input = include_str!("../input/2021/day9.txt");
    assert_eq!(part_1(input), 530);
}

#[test]
fn test_part_2_example() {
    let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
    assert_eq!(part_2_generic::<11>(input), 1134);
}
