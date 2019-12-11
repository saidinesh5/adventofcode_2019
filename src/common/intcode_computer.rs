use std::collections::VecDeque;
use std::convert::From;

#[derive(Debug, Clone)]
pub struct IntCodeComputer {
    is_halted: bool,
    p: usize,
    relative_base: isize,
    memory: Vec<isize>,
    input: VecDeque<isize>,
    output: VecDeque<isize>
}

#[derive(PartialEq)]
pub enum ReturnEvent {
    OutputEvent,
    HaltEvent
}

impl IntCodeComputer {
    pub fn new() -> IntCodeComputer {
        IntCodeComputer {
            is_halted: false,
            p: 0,
            relative_base: 0,
            memory: Vec::new(),
            input: VecDeque::new(),
            output: VecDeque::new()
        }
    }

    pub fn push_input(&mut self, value: isize) {
        self.input.push_back(value);
    }

    pub fn has_output(&self) -> bool {
        self.output.len() > 0
    }

    pub fn pop_output(&mut self) -> isize {
        self.output.pop_front().unwrap()
    }

    pub fn pop_all_output(&mut self) -> Vec<isize> {
        let mut result = Vec::new();

        while self.has_output() {
            result.push(self.pop_output());
        }

        result
    }

    pub fn is_halted(&self) -> bool {
        self.is_halted
    }

    fn init_fetch(&mut self, memory_location: usize) -> isize {
        if memory_location >= self.memory.len() {
            self.memory.resize(memory_location + 1, 0);
        }

        self.memory[memory_location]
    }

    fn current_parameter_mode(&mut self, parameter_position: usize) -> isize {
        // Rightmost 2 digits = opcode
        // After that each digit represents parameter mode
        (self.init_fetch(self.p)/100/((10 as isize).pow(parameter_position as u32 - 1)))%10
    }

    fn current_parameter_value(&mut self, parameter_position: usize) -> isize {
        let base_value = self.init_fetch(self.p + parameter_position);
        // 2 => Relative mode
        // 1 => Immediate mode
        // 0 => Position mode
        match self.current_parameter_mode(parameter_position) {
            2 => { self.init_fetch((self.relative_base + base_value) as usize) },
            1 => { base_value },
            _ => { self.init_fetch(base_value as usize) }
        }
    }

    pub fn process(&mut self, return_event: ReturnEvent) -> bool {
        if self.is_halted {
            return self.is_halted;
        }

        while self.p < self.memory.len() {
            let opcode = self.memory[self.p]%100;
            let p = self.p;

            // Parameters that an instruction writes to will never be in immediate mode.
            let write_location = match opcode {
                1 | 2 | 7 | 8 => if self.current_parameter_mode(3) == 2 { self.relative_base + self.init_fetch(p+3) } else { self.init_fetch(p+3) },
                3             => if self.current_parameter_mode(1) == 2 { self.relative_base + self.init_fetch(p+1) } else { self.init_fetch(p+1) },
                _             => 0
            } as usize;

            self.init_fetch(write_location); //Make sure it exists

            match opcode {
                1 => {
                    self.memory[write_location] = self.current_parameter_value(1) + self.current_parameter_value(2);
                    self.p += 4;
                },
                2 => {
                    self.memory[write_location] = self.current_parameter_value(1) * self.current_parameter_value(2);
                    self.p +=4;
                },
                3 => {
                    self.memory[write_location] = self.input.pop_front().unwrap();
                    self.p += 2;
                },
                4 => {
                    let output_parameter = self.current_parameter_value(1);
                    self.output.push_back(output_parameter);
                    self.p += 2;

                    if return_event == ReturnEvent::OutputEvent {
                        break;
                    }
                },
                5 => {
                    if self.current_parameter_value(1) != 0 {
                        self.p = self.current_parameter_value(2) as usize;
                    }
                    else {
                        self.p += 3;
                    }
                },
                6 => {
                    if self.current_parameter_value(1) == 0 {
                        self.p = self.current_parameter_value(2) as usize;
                    }
                    else {
                        self.p += 3;
                    }
                },
                7 => {
                    self.memory[write_location] = (self.current_parameter_value(1) < self.current_parameter_value(2)) as isize;
                    self.p += 4;
                },
                8 => {
                    self.memory[write_location] = (self.current_parameter_value(1) == self.current_parameter_value(2)) as isize;
                    self.p += 4;
                },
                9 => {
                    self.relative_base = self.relative_base + self.current_parameter_value(1);
                    self.p += 2;
                }
                _ => {
                    if opcode != 99 {
                        println!("Invalid operation!: {} ", opcode);
                    }
                    self.is_halted = true;
                    break
                }
            };
        }

        return self.is_halted;
    }
}

impl From<&str> for IntCodeComputer {
    /// Assume that the text is a comma separated values of memory region
    fn from(text: &str) -> Self {
        IntCodeComputer {
            is_halted: false,
            p: 0,
            relative_base: 0,
            memory: {
                text.split(',')
                    .filter(|item| item.trim().len() > 0)
                    .map(|item| item.trim().parse::<isize>().unwrap())
                    .collect::<Vec<isize>>()
            },
            input: VecDeque::new(),
            output: VecDeque::new()
        }
    }
}

impl From<Vec<isize>> for IntCodeComputer {
    fn from(instructions: Vec<isize>) -> Self {
        IntCodeComputer {
            is_halted: false,
            p: 0,
            relative_base: 0,
            memory: instructions,
            input: VecDeque::new(),
            output: VecDeque::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fetch_output_for(memory: Vec<isize>, input: &mut Vec<isize>) -> Vec<isize> {
        let mut computer = IntCodeComputer::from(memory);

        while input.len() > 0 {
            computer.push_input(input.remove(0));
        }

        computer.process(ReturnEvent::HaltEvent);
        computer.pop_all_output()
    }

    #[test]
    fn test_position_and_immediate_mode() {
        // 101,0,0,0,99 becomes 101,0,0,0,99 (0 + 101 = 101).
        assert_eq!(fetch_output_for(vec![101,0,0,0,4,0,99], &mut vec![1]), [101]);

        // 1001,0,0,0,99 becomes 1001,0,0,0,99 (1001 + 0 = 101).
        assert_eq!(fetch_output_for(vec![1001,0,0,0,4,0,99], &mut vec![1]), [1001]);

        // 1101,1,238,0,99 becomes 239,1,238,0,99 (1 + 238 = 101).
        assert_eq!(fetch_output_for(vec![1101,1,238,0,4,0,99], &mut vec![1]), [239]);

        // 1,0,0,0,99 becomes 2,0,0,0,99 (1 + 1 = 2).
        assert_eq!(fetch_output_for(vec![1,0,0,0,4,0,99], &mut vec![1]), [2]);

        // The program 3,0,4,0,99 outputs whatever it gets as input, then halts.
        assert_eq!(fetch_output_for(vec![3,0,4,0,99], &mut vec![5]), [5]);

        // The program 1002,4,3,4,33,4,4,99 multiplies parameter 4 in position mode
        // with 3 in immediate mode and puts it in position 4, then halts.
        assert_eq!(fetch_output_for(vec![1002,4,3,4,33], &mut vec![5]), []);

        // Integers can be negative: 1101,101,-1,0 is a valid program (find 101 + -1, store the result in position 0)
        assert_eq!(fetch_output_for(vec![1101,101,-1,0,4,0,99,0], &mut vec![5]), [100]);

        // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not)
        assert_eq!(fetch_output_for(vec![3,9,8,9,10,9,4,9,99,-1,8], &mut vec![5]), [0]);
        assert_eq!(fetch_output_for(vec![3,9,8,9,10,9,4,9,99,-1,8], &mut vec![8]), [1]);

        // 3,9,7,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        assert_eq!(fetch_output_for(vec![3,9,7,9,10,9,4,9,99,-1,8], &mut vec![8]), [0]);
        assert_eq!(fetch_output_for(vec![3,9,7,9,10,9,4,9,99,-1,8], &mut vec![5]), [1]);

        // Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        assert_eq!(fetch_output_for(vec![3,3,1108,-1,8,3,4,3,99], &mut vec![5]), [0]);
        assert_eq!(fetch_output_for(vec![3,3,1108,-1,8,3,4,3,99], &mut vec![8]), [1]);

        // Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not)
        assert_eq!(fetch_output_for(vec![3,3,1107,-1,8,3,4,3,99], &mut vec![8]), [0]);
        assert_eq!(fetch_output_for(vec![3,3,1107,-1,8,3,4,3,99], &mut vec![5]), [1]);
    }

    #[test]
    fn test_relative_mode() {
        // Set the relative base to 1 and output the relative position -1
        assert_eq!(fetch_output_for(vec![109,1,204,-1, 99], &mut vec![]), [109]);

        // 109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99 takes no input and produces a copy of itself as output.
        assert_eq!(fetch_output_for(vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99], &mut vec![]),
                   [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);

        // 1102,34915192,34915192,7,4,7,99,0 should output a 16-digit number
        assert_eq!(fetch_output_for(vec![1102,34915192,34915192,7,4,7,99,0], &mut vec![])[0].to_string().bytes().len(), 16);

        // 104,1125899906842624,99 should output the large number in the middle.
        assert_eq!(fetch_output_for(vec![104,1125899906842624,99], &mut vec![]), [1125899906842624]);
    }

    #[test]
    fn test_jumps() {
        // Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:
        // (using position mode)
        assert_eq!(fetch_output_for(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &mut vec![0]), [0]);
        assert_eq!(fetch_output_for(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &mut vec![5]), [1]);

        // (using immediate mode)
        assert_eq!(fetch_output_for(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &mut vec![0]), [0]);
        assert_eq!(fetch_output_for(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &mut vec![5]), [1]);

        // The above example program uses an input instruction to ask for a single number.
        // The program will then output 999 if the input value is below 8, output 1000 if the input value is equal to 8,
        // or output 1001 if the input value is greater than 8.
        let program = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                          1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                          999,1105, 1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        assert_eq!(fetch_output_for(program.clone(), &mut vec![7]), [999]);
        assert_eq!(fetch_output_for(program.clone(), &mut vec![8]), [1000]);
        assert_eq!(fetch_output_for(program.clone(), &mut vec![9]), [1001]);
    }
}