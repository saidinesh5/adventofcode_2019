pub fn process_a(text: &str, phases: usize) -> String {
    cleanup_signal(text.trim(), phases)[0..8].to_string()
}

pub fn process_b(text: &str, phases: usize) -> String {
    let offset = text[..7].parse::<usize>().unwrap();
    cleanup_signal(&text.trim().repeat(10000), phases)[offset..(offset+8)].to_string()
}


fn cleanup_signal(text: &str, phases: usize) -> String {
    let digits = text.chars()
                     .map(|i| i.to_string()
                               .parse::<isize>().unwrap())
                     .collect::<Vec<isize>>();

    // let base_pattern = [0, 1, 0, -1];
    let mut current_input = digits;

    for _ in  0..phases {
        // The annoying math that makes this work
        //           0  1  2  3   4   5
        //      A = [1, 2, 3, 4,  5]
        //      S = [0, 1, 3, 6, 10, 15]
        //      start_position = 3
        //      length = 2
        //      prefix_sums[start_position + length] - prefix_sums[start_position]

        let mut prefix_sums = vec![0; current_input.len() + 1];

        for i in 1..(prefix_sums.len()) {
            prefix_sums[i] = prefix_sums[i-1] + current_input[i-1];
        }

        let pattern_sum = |mut start_position, length| {
            let skip = 4*length;
            let mut result = 0;
            while start_position < prefix_sums.len() {
                result += prefix_sums[std::cmp::min(start_position + length, prefix_sums.len() - 1)] - prefix_sums[start_position];
                start_position += skip;
            }
            result
        };

        current_input = (1..(current_input.len() + 1)).map(|output_position| {
                                                            // Because the pattern looks like 0,1,0,-1 we just subtract some sums from some sums
                                                            // -1 because we have to skip first element
                                                            let to_add = pattern_sum(output_position - 1, output_position);
                                                            let to_subtract = pattern_sum(3*output_position - 1, output_position);
                                                            (to_add - to_subtract).abs()%10
                                                      })
                                                      .collect::<Vec<isize>>();

    }

    current_input.iter()
                 .map(|&i| i.to_string())
                 .collect::<String>()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_a() {

        assert_eq!(process_a("12345678", 0), "12345678");

        assert_eq!(process_a("12345678", 1), "48226158");

        assert_eq!(process_a("12345678", 2), "34040438");

        assert_eq!(process_a("12345678", 3), "03415518");

        assert_eq!(process_a("12345678", 4), "01029498");

        // Here are the first eight digits of the final output list after 100 phases for some larger inputs
        assert_eq!(process_a("80871224585914546619083218645595", 100), "24176176");

        assert_eq!(process_a("19617804207202209144916044189917", 100), "73745418");

        assert_eq!(process_a("69317163492948606335995924319873", 100), "52432133");
    }

    #[test]
    fn test_b() {
        assert_eq!(process_b("03036732577212944063491565474664", 100), "84462026");

        assert_eq!(process_b("02935109699940807407585447034323", 100), "78725270");

        assert_eq!(process_b("03081770884921959731165446850517", 100), "53553731");
    }
}