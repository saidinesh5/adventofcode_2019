pub fn process_a(text: &str) -> usize {
    //Convert the text input to a vector of integers
    let mut memory = to_vector_of_usize(text);

    if memory.len() < 2 {
        println!("Empty memory cannot be processed!");
        return 0;
    }

    memory[1] = 12;
    memory[2] = 2;
    process(&mut memory)
}

pub fn process_b(text: &str) -> usize {
    let memory = to_vector_of_usize(text);

    if memory.len() < 2 {
        println!("Empty memory cannot be processed!");
        return 0;
    }

    for i in 0..100 {
        for j in 0..100 {
            let mut new_memory = memory.clone();
            new_memory[1] = i;
            new_memory[2] = j;

            if process(&mut new_memory) == 19690720 {
                return 100*i + j;
            }
        }
    }

    println!("Unable to find a valid pair of inputs that produce: 19690720");
    return 0;
}

fn to_vector_of_usize(text: &str) -> Vec<usize> {
    text.split(',')
        .filter(|item| item.len() > 0)
        .map(|item| item.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn process(memory: &mut Vec<usize>) -> usize {
    let mut p = 0;

    while p + 3 < memory.len() {
        let operation = memory[p];
        let result_location = memory[p+3];

        match operation {
            1 => memory[result_location] = memory[memory[p+1]] + memory[memory[p+2]],
            2 => memory[result_location] = memory[memory[p+1]] * memory[memory[p+2]],
            _ => {
                if operation != 99 {
                    println!("Invalid operation!: {} ", operation);
                }
                break
            }
        };

        p += 4;
    }

    memory[0]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        // 1,0,0,0,99 becomes 2,0,0,0,99 (1 + 1 = 2).
        assert_eq!(process(&mut(vec![1,0,0,0,99])), 2);
        // 2,3,0,3,99 becomes 2,3,0,6,99 (3 * 2 = 6).
        assert_eq!(process(&mut(vec![2,3,0,3,99])), 2);
        // 2,4,4,5,99,0 becomes 2,4,4,5,99,9801 (99 * 99 = 9801).
        assert_eq!(process(&mut(vec![2,4,4,5,99,0])), 2);
        // 1,1,1,4,99,5,6,0,99 becomes 30,1,1,4,2,5,6,0,99.
        assert_eq!(process(&mut(vec![1,1,1,4,99,5,6,0,99])), 30);
    }
}