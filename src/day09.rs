use super::intcode_computer::*;

pub fn process(text: &str, input: isize) -> isize {
    let mut computer = IntCodeComputer::from(text);
    computer.push_input(input);
    computer.process(ReturnEvent::HaltEvent);

    let output =  computer.pop_all_output();
    assert!(output.len() > 0);
    // Make sure all outputs except the last one are 0
    assert!(&output[..output.len() - 1].iter().all(|&value| value == 0));
    *output.last().unwrap()
}