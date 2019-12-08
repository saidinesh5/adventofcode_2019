pub fn process_a(text: &str, width: usize, height: usize ) -> usize {
    let layer_length = width*height;
    let layer_count = text.len()/layer_length;
    let count_of = |s: &str, c| s.bytes()
                                 .filter(|&b| b == c)
                                 .count();
    let zero_counts = (0..layer_count).map(|i| count_of(&text[i*layer_length..(i*layer_length+layer_length)], '0' as u8))
                                      .collect::<Vec<usize>>();
    let i_min_zeroes = zero_counts.iter().enumerate().min_by_key(|&(_, value)| value).unwrap().0;
    let layer_text = &text[i_min_zeroes*layer_length..(i_min_zeroes*layer_length + layer_length)];

    count_of(layer_text, '1' as u8)*count_of(layer_text, '2' as u8)
}

pub fn process_b(text: &str, width: usize, height: usize ) -> String {
    let layer_length = width*height;
    let layer_count = text.len()/layer_length;
    // The merged layers in themselves aren't the solution for this problem
    let merged_layers = (0..layer_length).map(|i| {
                                                (0..layer_count).fold('2', |current, layer_index| {
                                                    if current != '2' {
                                                        current
                                                    } else {
                                                        text.as_bytes()[layer_length*layer_index + i] as char
                                                    }
                                                })
                                            }).collect::<String>();

    // It is for the user to interpret the bitmap as readable text
    // Hence we return a printable result
    let tmp = merged_layers.bytes()
                           .map(|c| if c as char == '1' { '#' } else {  ' ' })
                          .collect::<String>();

    // We want to make sure that the printable_result starts on a new line. Hence the \n at the beginning of a line
    let printable_result = (0..height).map(|i| String::from("\n") + &tmp[width*i..width*i + width])
                                      .collect::<String>();

    printable_result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_a() {
        assert_eq!(process_a("123456789012", 3, 2), 1);
    }

    #[test]
    fn test_b() {
        // 0222112222120000 -> 0110
        assert_eq!(process_b("0222112222120000", 2, 2), String::from("\n #\n# "));
    }
}