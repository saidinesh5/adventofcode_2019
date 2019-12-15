use std::collections::HashMap;

use super::intcode_computer::*;

type Pair = (isize, isize);
type Tile = isize;

const EMPTY: isize = 0;
const WALL: isize = 1;
const BLOCK: isize = 2;
const HORIZONTAL_PADDLE: isize = 3;
const BALL: isize = 4;


pub fn process_a(instructions: &str) -> usize {
    let mut computer = IntCodeComputer::from(instructions);
    computer.process(ReturnEvent::HaltEvent);

    get_coordinate_values(computer.pop_all_output()).values()
                                                    .filter(|&v| *v == BLOCK).count()
}

pub fn process_b(instructions: &str) -> isize {
    let mut computer = IntCodeComputer::from(instructions);

    computer.set_memory_value(0, 2);
    computer.process(ReturnEvent::InputNeededEvent);

    let mut canvas = get_coordinate_values(computer.pop_all_output());
    let mut block_count = canvas.values().filter(|&v| *v == BLOCK).count();
    let mut paddle_position = get_location(&canvas, HORIZONTAL_PADDLE, &(0, 0));
    let mut ball_position = get_location(&canvas, BALL, &(0, 0));
    let mut score = canvas.remove(&(-1, 0)).unwrap_or(0);

    while !computer.is_halted() && block_count > 0 {

        // Just move the joystick whichever side the ball is.. I know it can be excessive,
        // But Meh works good enough, and we are doing the loop so many times anyway
        // An optimization could have been to take into account the ball's velocity in y direction
        let joystick_position = if paddle_position.0 < ball_position.0 { 1 }
                                else if paddle_position.0 == ball_position.0 { 0 }
                                else { -1 };

        computer.push_input(joystick_position);
        computer.process(ReturnEvent::InputNeededEvent);

        // The output we get here is not for the full canvas but only what changes it seems
        let output = computer.pop_all_output();

        for i in 0..output.len()/3 {
            let (x, y, value) = (output[3*i], output[3*i +1], output[3*i + 2]);

            if (x, y) == (-1, 0) {
                score = value;
            } else {
                if *canvas.get(&(x, y)).unwrap() == BLOCK {
                    block_count -= 1;
                }

                match value {
                    BALL => ball_position = (x, y),
                    HORIZONTAL_PADDLE => paddle_position = (x, y),
                    _ => ()
                }

                canvas.insert((x, y), value);
            }
        }

        // print_canvas(&canvas);
    }

    score
}

fn get_coordinate_values(output: Vec<isize>) -> HashMap<Pair, Tile> {
    (0..(output.len()/3)).map(|i| ((output[3*i], output[3*i+1]), output[3*i+2]))
                         .collect::<HashMap<Pair, Tile>>()
}

fn get_location(coordinate_values: &HashMap<Pair, Tile>, tile: isize, default_position: &Pair) -> Pair {
    let position_ref = coordinate_values.iter()
                                        .find(|&(_, v)| *v == tile);
    match position_ref {
        Some(_) => *(position_ref.unwrap().0),
        None => (default_position.0, default_position.1)
    }
}

#[allow(dead_code)]
fn print_canvas(panels: &HashMap<Pair, Tile>) {
    let width = panels.keys().max_by_key(|v| v.0).unwrap().0 as usize + 1;
    let height = panels.keys().max_by_key(|v| v.1).unwrap().1 as usize + 1;

    // This probably means we have received a negative number
    if width > 1000 || height > 1000 {
        return
    }

    let mut canvas = (0..height).map(|_| vec![' '; width]).collect::<Vec<Vec<char>>>();

    for (pos, color) in panels.iter() {
        let column = (pos.0) as usize;
        let row = (pos.1) as usize;
        canvas[row][column] = match *color {
            EMPTY => ' ',
            WALL => '#',
            BLOCK => '*',
            HORIZONTAL_PADDLE => '=',
            BALL => '@',
            _ => ' '
        };
    }

    // Reversing the string for better printing

    println!("{}", (0..height).map(|row| String::from("\n") + &canvas[row].iter()
                                                                          .collect::<String>())
                              .collect::<String>());
}

// No unit tests today either :(