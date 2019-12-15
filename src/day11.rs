use super::intcode_computer::*;
use std::collections::HashMap;

type Pair = (isize, isize);
type Color = isize;

const BLACK: isize = 0;
const WHITE: isize = 1;

pub fn process_a(text: &str) -> usize {
    run_robot(text, BLACK).len()
}

pub fn process_b(text: &str) -> String {
    let panels = run_robot(text, WHITE);
    let min_x = panels.keys().min_by_key(|&item| item.0).unwrap().0;
    let min_y = panels.keys().min_by_key(|&item| item.1).unwrap().1;
    let max_x = panels.keys().max_by_key(|&item| item.0).unwrap().0;
    let max_y = panels.keys().max_by_key(|&item| item.1).unwrap().1;
    let width = (max_x - min_x) as usize  + 1;
    let height = (max_y - min_y) as usize + 1;
    let mut canvas = (0..height).map(|_| vec![' '; width]).collect::<Vec<Vec<char>>>();

    for (pos, color) in panels.iter() {
        let column = (pos.0 - min_x) as usize;
        let row = (pos.1 - min_y) as usize;
        let c = if *color == WHITE { '#' } else { ' ' };
        canvas[row][column] = c;
    }

    // Reversing the string for better printing
    (0..height).rev().map(|row| String::from("\n") + &canvas[row].iter().collect::<String>())
               .collect::<String>()
}

// Returns all the panels that are painted by the robot
fn run_robot(instructions: &str, initial_input: isize) -> HashMap<Pair, Color> {
    let mut computer = IntCodeComputer::from(instructions);
    let mut panels: HashMap<Pair, isize> = HashMap::new();
    let mut current_location: Pair = (0,0);
    let mut current_angle: f64 = 90.0;

    // Paint the initial panel as per the requirement
    panels.entry(current_location).or_insert(initial_input);

    while !computer.is_halted() {
        let current_color = panels.entry(current_location).or_insert(BLACK);
        // Tell the computer what the current panel is colored as
        computer.push_input(*current_color);

        computer.process(ReturnEvent::OutputReadyEvent);

        // Paint the current panel with the color the computer tells you to
        if computer.has_output() {
            panels.insert(current_location, computer.pop_output());
        }

        computer.process(ReturnEvent::OutputReadyEvent);

        if computer.has_output() {
            match computer.pop_output() {
                0 => current_angle += 90.0,
                1 => current_angle -= 90.0,
                _ => {
                    println!("Invalid turn direction received!");
                    break;
                }
            }

            let direction = (current_angle.to_radians().cos() as isize, current_angle.to_radians().sin() as isize);
            current_location = (current_location.0 + direction.0 , current_location.1 + direction.1);
        }
    }

    panels
}

// No good unit tests today :(