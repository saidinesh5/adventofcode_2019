use std::collections::HashMap;
use std::collections::VecDeque;

use super::intcode_computer::*;

type Pair = (isize, isize);

const COMMAND_MOVE_NORTH: usize = 1;
const COMMAND_MOVE_SOUTH: usize = 2;
const COMMAND_MOVE_WEST: usize = 3;
const COMMAND_MOVE_EAST: usize = 4;

const MOVE_BLOCKED: isize = 0;
const MOVE_SUCCEEDED: isize = 1;
const MOVE_FOUND_OXYGEN: isize = 2;

pub fn process_a(instructions: &str) -> isize {
    let computer = IntCodeComputer::from(instructions);
    let mut visited_locations: HashMap<Pair, isize> = HashMap::new();
    let mut locations_to_visit: VecDeque<(Pair, IntCodeComputer, isize)> = VecDeque::new();

    locations_to_visit.push_back(((0, 0), computer.clone(), 0));
    visited_locations.insert((0, 0), MOVE_SUCCEEDED);

    // Simple breadth first search to find Oxygen System
    while !locations_to_visit.is_empty() {
        let (p, test_computer, commands_taken) = locations_to_visit.pop_front().unwrap();

        // command index, For each of the direction...
        for direction in 1..5 {
            let next_position = move_to(&p, direction);

            if !visited_locations.contains_key(&next_position) {

                let mut next_computer = test_computer.clone();
                next_computer.push_input(direction as isize);
                next_computer.process(ReturnEvent::InputNeededEvent);

                if next_computer.has_output() {
                    let output = next_computer.pop_output();
                    visited_locations.insert(next_position, output);
                    match output {
                        MOVE_FOUND_OXYGEN => return commands_taken as isize + 1,
                        MOVE_SUCCEEDED => locations_to_visit.push_back((next_position,
                                                                        next_computer,
                                                                        commands_taken + 1)),
                        MOVE_BLOCKED | _ => ()
                    }
                }
            }
        }
    }

    // Return -1 if Oxygen system is not found
    -1
}

pub fn process_b(instructions: &str) -> usize {
    let room_graph = get_room_graph(instructions);

    let oxygen = *room_graph.iter().find(|&(_, v)| *v == MOVE_FOUND_OXYGEN).unwrap().0;
    let mut visited_locations: HashMap<Pair, usize> = HashMap::new();
    let mut locations_to_visit: VecDeque<(Pair, usize)> = VecDeque::new();

    locations_to_visit.push_back(((oxygen.0, oxygen.1), 0));

    // Just a breadth first search to expand Oxygen to all unblocked places
    // While keeping track of time
    while !locations_to_visit.is_empty() {
        let (p, current_time) = locations_to_visit.pop_front().unwrap();

        for direction in 1..5 {
            let next_position = move_to(&p, direction);

            if *room_graph.get(&next_position).unwrap_or(&MOVE_BLOCKED) != MOVE_BLOCKED
                   && !visited_locations.contains_key(&next_position) {
                visited_locations.insert(next_position, current_time + 1);
                locations_to_visit.push_back((next_position, current_time + 1));
            }
        }
    }

    // Total time taken is just the maximum time reached during the search
    *visited_locations.values()
                      .max()
                      .unwrap()
}

fn move_to(p: &Pair, direction: usize) -> Pair{
    match direction {
        COMMAND_MOVE_NORTH => (p.0, p.1 + 1),
        COMMAND_MOVE_SOUTH => (p.0, p.1 - 1),
        COMMAND_MOVE_EAST =>  (p.0 + 1, p.1),
        COMMAND_MOVE_WEST =>  (p.0 - 1, p.1),
        _ => (p.0, p.1)
    }
}

fn get_room_graph(instructions: &str) -> HashMap<Pair, isize> {
    let computer = IntCodeComputer::from(instructions);
    let mut visited_locations: HashMap<Pair, isize> = HashMap::new();
    let mut locations_to_visit: VecDeque<(Pair, IntCodeComputer)> = VecDeque::new();

    locations_to_visit.push_back(((0, 0), computer.clone()));
    visited_locations.insert((0, 0), MOVE_SUCCEEDED);

    // Simple breadth first search to populate the graph
    while !locations_to_visit.is_empty() {
        let (p, test_computer) = locations_to_visit.pop_front().unwrap();

        // command index, For each of the direction...
        for direction in 1..5 {
            let next_position = move_to(&p, direction);

            if !visited_locations.contains_key(&next_position) {

                let mut next_computer = test_computer.clone();
                next_computer.push_input(direction as isize);
                next_computer.process(ReturnEvent::InputNeededEvent);

                if next_computer.has_output() {
                    let output = next_computer.pop_output();

                    visited_locations.insert(next_position, output);

                    match output {
                        MOVE_FOUND_OXYGEN | MOVE_SUCCEEDED => locations_to_visit.push_back((next_position,
                                                                                            next_computer)),
                        MOVE_BLOCKED | _ => ()
                    }
                }
            }
        }
    }

    visited_locations
}

// No unit tests available today either :(