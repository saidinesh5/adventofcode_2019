pub fn process_a(text: &str) -> isize {
    let mut memory = to_vector_of_isize(text);

    if memory.len() < 2 {
        println!("Empty memory cannot be processed!");
        return 0;
    }

    let output =  process(&mut memory, 1);
    assert!(output.len() > 0);
    // Make sure all outputs except the last one are 0
    assert!(&output[..output.len() - 1].iter().all(|&value| value == 0));
    *output.last().unwrap()
}

pub fn process_b(text: &str) -> isize {
    let mut memory = to_vector_of_isize(text);

    if memory.len() < 2 {
        println!("Empty memory cannot be processed!");
        return 0;
    }

    let output =  process(&mut memory, 5);
    assert!(output.len() > 0);
    // Make sure all outputs except the last one are 0
    assert!(&output[..output.len() - 1].iter().all(|&value| value == 0));
    *output.last().unwrap()
}


fn to_vector_of_isize(text: &str) -> Vec<isize> {
    text.split(',')
        .filter(|item| item.len() > 0)
        .map(|item| item.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

fn process(memory: &mut Vec<isize>, input: isize) -> Vec<isize> {
    let mut output = Vec::new();
    let mut p = 0;

    while p < memory.len() {
        let parameters = memory[p]/100;
        let opcode = memory[p]%100;

        let parameter = |position| {
            let mode = (parameters/((10 as isize).pow(position as u32 - 1)))%10;
            return  if mode == 0 { memory[(memory[p + position] as usize)] } else { memory[p + position] };
        };

        // Parameters that an instruction writes to will never be in immediate mode.
        let write_location = match opcode {
            1 | 2 | 7 | 8 => memory[p+3] as usize,
            3             => memory[p+1] as usize,
            _             => 0
        };

        match opcode {
            1 => {
                memory[write_location] = parameter(1) + parameter(2);
                p += 4;
            },
            2 => {
                memory[write_location] = parameter(1) * parameter(2);
                p +=4;
            },
            3 => {
                memory[write_location] = input;
                p += 2;
            },
            4 => {
                output.push(parameter(1));
                p += 2;
            },
            5 => {
                if parameter(1) != 0 {
                    p = parameter(2) as usize;
                }
                else {
                    p += 3;
                }
            },
            6 => {
                if parameter(1) == 0 {
                    p = parameter(2) as usize;
                }
                else {
                    p += 3;
                }
            },
            7 => {
                memory[write_location] = (parameter(1) < parameter(2)) as isize;
                p += 4;
            },
            8 => {
                memory[write_location] = (parameter(1) == parameter(2)) as isize;
                p += 4;
            },
            _ => {
                if opcode != 99 {
                    println!("Invalid operation!: {} ", opcode);
                }
                break
            }
        };
    }

    output
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        // 101,0,0,0,99 becomes 101,0,0,0,99 (0 + 101 = 101).
        assert_eq!(process(&mut(vec![101,0,0,0,4,0,99]), 1), [101]);

        // 1001,0,0,0,99 becomes 1001,0,0,0,99 (1001 + 0 = 101).
        assert_eq!(process(&mut(vec![1001,0,0,0,4,0,99]), 1), [1001]);

        // 1101,1,238,0,99 becomes 239,1,238,0,99 (1 + 238 = 101).
        assert_eq!(process(&mut(vec![1101,1,238,0,4,0,99]), 1), [239]);

        // 1,0,0,0,99 becomes 2,0,0,0,99 (1 + 1 = 2).
        assert_eq!(process(&mut(vec![1,0,0,0,4,0,99]), 1), [2]);

        // The program 3,0,4,0,99 outputs whatever it gets as input, then halts.
        assert_eq!(process(&mut(vec![3,0,4,0,99]), 5), [5]);

        // The program 1002,4,3,4,33,4,4,99 multiplies parameter 4 in position mode
        // with 3 in immediate mode and puts it in position 4, then halts.
        assert_eq!(process(&mut(vec![1002,4,3,4,33]), 5), []);

        // Integers can be negative: 1101,101,-1,0 is a valid program (find 101 + -1, store the result in position 0)
        assert_eq!(process(&mut(vec![1101,101,-1,0,4,0,99,0]), 5), [100]);
    }

    #[test]
    fn test_b()  {
        // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not)
        assert_eq!(process(&mut(vec![3,9,8,9,10,9,4,9,99,-1,8]), 5), [0]);
        assert_eq!(process(&mut(vec![3,9,8,9,10,9,4,9,99,-1,8]), 8), [1]);
        

        // 3,9,7,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        assert_eq!(process(&mut(vec![3,9,7,9,10,9,4,9,99,-1,8]), 8), [0]);
        assert_eq!(process(&mut(vec![3,9,7,9,10,9,4,9,99,-1,8]), 5), [1]);

        // Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        assert_eq!(process(&mut(vec![3,3,1108,-1,8,3,4,3,99]), 5), [0]);
        assert_eq!(process(&mut(vec![3,3,1108,-1,8,3,4,3,99]), 8), [1]);

        // Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not)
        assert_eq!(process(&mut(vec![3,3,1107,-1,8,3,4,3,99]), 8), [0]);
        assert_eq!(process(&mut(vec![3,3,1107,-1,8,3,4,3,99]), 5), [1]);

        // Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:
        // (using position mode)
        assert_eq!(process(&mut(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]), 0), [0]);
        assert_eq!(process(&mut(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]), 5), [1]);

        // (using immediate mode)
        assert_eq!(process(&mut(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1]), 0), [0]);
        assert_eq!(process(&mut(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1]), 5), [1]);

        // The above example program uses an input instruction to ask for a single number.
        // The program will then output 999 if the input value is below 8, output 1000 if the input value is equal to 8,
        // or output 1001 if the input value is greater than 8.
        let mut program = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                              1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                              999,1105, 1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        assert_eq!(process(&mut program, 7), [999]);
        assert_eq!(process(&mut program, 8), [1000]);
        assert_eq!(process(&mut program, 9), [1001]);
    }
}