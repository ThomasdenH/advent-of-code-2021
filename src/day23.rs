use std::{collections::{BinaryHeap, HashMap}, num::NonZeroU8};
use std::{
    cmp::Ordering
};

use arrayvec::ArrayVec;
use nom::{character::complete::{line_ending, one_of, not_line_ending}, IResult, sequence::terminated, bytes::complete::{tag, take}, combinator::map};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Amphipod(NonZeroU8);

impl Default for Amphipod {
    fn default() -> Self {
        Amphipod(b'0'.try_into().unwrap())
    }
}

impl Amphipod {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(one_of("ABCD"), |c| Amphipod(NonZeroU8::try_from(c as u8).unwrap()))(input)
    }

    const fn energy_per_step(&self) -> usize {
        if self == Amphipod(NonZeroU8::try_from(b'A').unwrap()) {
            1
        } else if self == Amphipod(NonZeroU8::try_from(b'B').unwrap()) {
            10
        } else if self == Amphipod(NonZeroU8::try_from(b'C').unwrap()) {
            100
        } else {
            1000
        }
    }
}

const ROOM_COUNT: usize = 4;
const HALL_SPOTS: usize = ROOM_COUNT + 1;

#[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
struct Diagram<const ROOM_DEPTH: usize> {
    rooms: [ArrayVec<Amphipod, ROOM_DEPTH>; ROOM_COUNT],
    hall_positions: [Option<Amphipod>; HALL_SPOTS]
}

impl<const ROOM_DEPTH: usize> Diagram<ROOM_DEPTH> {
    fn parse(input: &str) -> IResult<&str, Self> {
        let mut diagram = Diagram::default();
        for room in diagram.rooms.iter_mut() {
            for _ in 0..ROOM_DEPTH {
                room.push(Amphipod::default());
            }
        }

        // #############
        let (input, _) = terminated(not_line_ending, line_ending)(input)?;
        // #...........#
        let (mut input, _) = terminated(not_line_ending, line_ending)(input)?; 
        for depth in 0..ROOM_DEPTH {
            let (new_input, _) = take(3_usize)(input)?;
            input = new_input;
            for room in diagram.rooms.iter_mut() {
                let (new_input, amphipod) = terminated(Amphipod::parse, tag("#"))(input)?;
                input = new_input;
                room[ROOM_DEPTH - 1 - depth] = amphipod;
            }
            let (new_input, _) = terminated(not_line_ending, line_ending)(input)?;
            input = new_input
        }

        let (mut input, _) = tag("  ")(input)?;
        for _ in 0..(2 * ROOM_COUNT + 1) {
            input = tag("#")(input)?.0;
        }

        Ok((input, diagram))
    }

    fn estimate_remaining_cost(&self) -> usize {
        unimplemented!()
    }

    fn possible_moves(&self) -> impl Iterator<Item = Move> {
        unimplemented!()
    }

    fn is_solution(&self) -> bool {
        unimplemented!()
    }

    fn cost_of_move(&self, r#move: Move) -> usize {
        unimplemented!()
    }

    fn after_move(&self, r#move: Move) -> Self {
        unimplemented!()
    }
}

enum MoveType {
    RoomToHall,
    HallToRoom
}

struct Move {
    r#type: MoveType,
    room: u8,
    hall: u8,
    amphipod: Amphipod
}

#[derive(Eq, PartialEq)]
struct BinaryHeapItem<const ROOM_DEPTH: usize> {
    item: Diagram<ROOM_DEPTH>,
    cost_estimate: usize
}

impl<const ROOM_DEPTH: usize> Ord for BinaryHeapItem<ROOM_DEPTH> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost_estimate.cmp(&self.cost_estimate)
    }
}

impl<const ROOM_DEPTH: usize> PartialOrd for BinaryHeapItem<ROOM_DEPTH> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_1(input: &str) -> usize {
    let (_, diagram) = Diagram::parse(input).unwrap();
    let mut open_set = BinaryHeap::new();
    let mut g_score = HashMap::new();
    g_score.insert(diagram, 0);
    open_set.push(BinaryHeapItem {
        item: diagram,
        cost_estimate: diagram.estimate_remaining_cost()
    });

    while let Some(BinaryHeapItem { item , .. }) = open_set.pop() {
        let current_score = *g_score.get(&item).unwrap();
        if item.is_solution() {
            return current_score;
        }
        for possible_move in item.possible_moves() {
            let tentative_score = current_score + item.cost_of_move(possible_move);
            let after_move = diagram.after_move(possible_move);
            if tentative_score < *g_score.get(&after_move).unwrap_or(&usize::MAX) {
                g_score.insert(after_move, tentative_score);
                let cost_estimate = tentative_score + after_move.estimate_remaining_cost();
                open_set.push(BinaryHeapItem {
                    item: after_move,
                    cost_estimate,
                });
            }
        }
    }
    unreachable!("No solution")
}


#[test]
fn test_parse() {
    let input = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

  let pod = |a: u8| Amphipod(a.try_into().unwrap());

  assert_eq!(Diagram::parse(input), Ok(("", Diagram {
      rooms: [
          ArrayVec::from([pod(b'A'), pod(b'B')]),
          ArrayVec::from([pod(b'D'), pod(b'C')]),
          ArrayVec::from([pod(b'C'), pod(b'B')]),
          ArrayVec::from([pod(b'A'), pod(b'D')])
      ],
      hall_positions: [None; HALL_SPOTS]
  })));
}