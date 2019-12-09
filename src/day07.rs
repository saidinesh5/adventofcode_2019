use permute;

use super::intcode_computer::*;

pub fn process_a(text: &str) -> isize {
    let amplifier_count = 5;
    let amplifiers = (0..amplifier_count).map(#[allow(unused)]
                                              |i| IntCodeComputer::new(text))
                                         .collect::<Vec<IntCodeComputer>>();

    permute::permutations_of(&[0,1,2,3,4]).map(|permutation| {
                                                    let mut amplifiers = amplifiers.clone();

                                                    for p in permutation.enumerate() {
                                                    // The program first asks for the phase setting
                                                    amplifiers[p.0].push_input(*p.1);

                                                    // And then asks for the input signal
                                                    if p.0 == 0 {
                                                        amplifiers[p.0].push_input(0);
                                                    }
                                                    else {
                                                        let output = amplifiers[p.0 - 1].pop_output();
                                                        amplifiers[p.0].push_input(output);
                                                    }

                                                    amplifiers[p.0].process(ReturnEvent::HaltEvent);
                                                }

                                                amplifiers[amplifier_count - 1].pop_output()
                                           })
                                          .max()
                                          .unwrap()
}

pub fn process_b(text: &str) -> isize {
    let amplifier_count = 5;
    let original_amplifiers = (0..amplifier_count).map(#[allow(unused)]
                                                      |i| IntCodeComputer::new(text))
                                                  .collect::<Vec<IntCodeComputer>>();

    permute::permutations_of(&[5,6,7,8,9]).map(|permutation| {
                                                    let mut amplifiers = original_amplifiers.clone();
                                                    let mut signal = 0;
                                                    let mut i = 0;
                                                    let p: Vec<isize> = permutation.map(|x| *x as isize).collect();

                                                    // Initialize the amplifiers with phase setting first
                                                    for a in 0..amplifier_count {
                                                        amplifiers[a].push_input(p[a]);
                                                    }

                                                    // Cycle through each of the amplifiers until the last one is halted
                                                    while !amplifiers[amplifier_count - 1].is_halted() {

                                                        // And then asks for the input signal
                                                        amplifiers[i].push_input(signal);
    
                                                        amplifiers[i].process(ReturnEvent::OutputEvent);

                                                        if amplifiers[i].has_output() {
                                                            signal = amplifiers[i].pop_output();
                                                        } else {
                                                            if !amplifiers[i].is_halted() {
                                                                println!("Amplifier {} had no output for this iteration!", i);
                                                            }
                                                        }

                                                        i = (i+1)%amplifier_count;
                                                    }

                                                    signal
                                                 })
                                          .max()
                                          .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        // Max thruster signal 43210 (from phase setting sequence 4,3,2,1,0)
        assert_eq!(process_a("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"), 43210);

        // Max thruster signal 54321 (from phase setting sequence 0,1,2,3,4)
        assert_eq!(process_a("3,23,3,24,1002,24,10,24,1002,23,-1,23,\
        101,5,23,23,1,24,23,23,4,23,99,0,0"), 54321);

        // Max thruster signal 65210 (from phase setting sequence 1,0,4,3,2)
        assert_eq!(process_a("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,\
        1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"), 65210);
    }

    #[test]
    fn test_b() {
        // Max thruster signal 139629729 (from phase setting sequence 9,8,7,6,5)
        assert_eq!(process_b("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,\
        27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"), 139629729);

        // Max thruster signal 18216 (from phase setting sequence 9,7,8,5,6)
        assert_eq!(process_b("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,\
        -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,\
        53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"), 18216);
    }
}