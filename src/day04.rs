pub fn process_a(lower: usize, higher:usize) -> usize {
    (lower..higher).filter(|&v| is_valid_password_a(v))
                   .collect::<Vec<usize>>()
                   .len()
}

pub fn process_b(lower: usize, higher:usize) -> usize {
    (lower..higher).filter(|&v| is_valid_password_b(v))
                   .collect::<Vec<usize>>()
                   .len()
}

fn six_digits(mut value: usize) -> [usize; 6] {
    // Wrote a custom function to see if things would be faster compared to a vec
    // Meh
    let mut i = 0;
    let mut result = [0, 0, 0, 0, 0, 0];

    while value > 0 && i < 6{
        result[5-i] = value%10;
        value = value/10;
        i += 1;
    }

    result
}

fn is_valid_password_a(value: usize) -> bool {
    let s = six_digits(value);
    let mut is_increasing = true;
    let mut has_doubles = false;

    for i in 1..s.len() {
        is_increasing = is_increasing && (s[i-1] <= s[i]);
        has_doubles = has_doubles || (s[i-1] == s[i]);
    }

    is_increasing && has_doubles
}

fn is_valid_password_b(value: usize) -> bool {
    let s = six_digits(value);
    let mut is_increasing = true;
    let mut consecutive_counts = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for i in 1..s.len() {
        is_increasing = is_increasing && (s[i-1] <= s[i]);
        if s[i] == s[i-1] {
            consecutive_counts[s[i]] += 1
        }
        else {
            consecutive_counts[s[i]] = 0
        }
    }

    is_increasing && (consecutive_counts).iter().any(|&v| v == 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        // 111111 meets these criteria (double 11, never decreases).
        assert_eq!(is_valid_password_a(111111), true);
        // 223450 does not meet these criteria (decreasing pair of digits 50).
        assert_eq!(is_valid_password_a(223450), false);
        // 123789 does not meet these criteria (no double).
        assert_eq!(is_valid_password_a(123789), false);
    }

    #[test]
    fn test_b() {
        // 112233 meets these criteria because the digits never decrease and all repeated digits are exactly two digits long.
        assert_eq!(is_valid_password_b(112233), true);
        // 123444 no longer meets the criteria (the repeated 44 is part of a larger group of 444).
        assert_eq!(is_valid_password_b(123444), false);
        // 111122 meets the criteria (even though 1 is repeated more than twice, it still contains a double 22).
        assert_eq!(is_valid_password_b(111122), true);
    }
}
