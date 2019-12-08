pub fn process_a(text: &str) -> u64 {
    text.lines()
        .filter(|line| line.len() > 0)
        .fold(0, |sum, line| sum + (line.parse::<u64>().expect("Expected an integer"))/3 - 2)
}

pub fn process_b(text: &str) -> i64 {
    fn fuel_required(weight: i64, total: i64) -> i64 {
        let current_fuel_required = weight/3 - 2;
        return if current_fuel_required <= 0 { total } else { fuel_required(current_fuel_required, total + current_fuel_required) }
    }

    text.lines()
        .filter(|line| line.len() > 0)
        .fold(0, |sum, line| sum + fuel_required(line.parse::<i64>().expect("Expected an integer"), 0))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_a() {
        // For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
        assert_eq!(process_a("12"), 2);

        // For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
        assert_eq!(process_a("14"), 2);

        // For a mass of 1969, the fuel required is 654.
        assert_eq!(process_a("1969"), 654);

        // For a mass of 100756, the fuel required is 33583.
        assert_eq!(process_a("100756"), 33583);
    }

    #[test]
    fn test_b() {
        // A module of mass 14 requires 2 fuel. This fuel requires no further fuel
        // (2 divided by 3 and rounded down is 0, which would call for a negative fuel),
        // so the total fuel required is still just 2.
        assert_eq!(process_b("14"), 2);

        // At first, a module of mass 1969 requires 654 fuel. Then, this fuel requires 216 more fuel (654 / 3 - 2).
        // 216 then requires 70 more fuel, which requires 21 fuel, which requires 5 fuel, which requires no further fuel.
        // So, the total fuel required for a module of mass 1969 is 654 + 216 + 70 + 21 + 5 = 966.
        assert_eq!(process_b("1969"), 966);

        //The fuel required by a module of mass 100756 and its fuel is: 33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346.
        assert_eq!(process_b("100756"), 50346);
    }
}