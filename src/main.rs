use adventofcode_2019::*;

fn testdata(path: &str) -> String {
    use std::fs::File;
    use std::io::*;
    let mut file = File::open(path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    contents.trim().to_string()
}

fn main() {
    let days = [day01, day02, day03, day04, day05, day06, day07, day08, day09, day10];

    if std::env::args().count() > 2 {
        println!("Useage: {} [day number]", std::env::args().nth(0).unwrap());
        return;
    }

    let day = if std::env::args().count() == 1 { days.len() + 1 }
              else { std::env::args().nth(1).unwrap().parse::<usize>().expect("Unsigned Integer expected") };


    if day > 0 && day <= days.len() {
        days[day - 1]();
    }
    else {
        for &day in days.iter() {
            day();
        }
    }
}

fn day01() {
    println!("Day 1 result a = {}", day01::process_a(testdata("testdata/day01/a.txt").as_ref()));
    println!("Day 1 result b = {}", day01::process_b(testdata("testdata/day01/b.txt").as_ref()));
}

fn day02() {
    println!("Day 2 result a = {}", day02::process_a(testdata("testdata/day02/input.txt").as_ref()));
    println!("Day 2 result b = {}", day02::process_b(testdata("testdata/day02/input.txt").as_ref()));
}

fn day03() {
    println!("Day 3 result a = {}", day03::process_a(testdata("testdata/day03/input.txt").as_ref()));
    println!("Day 3 result b = {}", day03::process_b(testdata("testdata/day03/input.txt").as_ref()));
}

fn day04() {
    println!("Day 4 result a = {}", day04::process_a(136818,685979));
    println!("Day 4 result b = {}", day04::process_b(136818,685979));
}

fn day05() {
    println!("Day 5 result a = {}", day05::process_a(testdata("testdata/day05/input.txt").as_ref()));
    println!("Day 5 result b = {}", day05::process_b(testdata("testdata/day05/input.txt").as_ref()));
}

fn day06() {
    println!("Day 6 result a = {}", day06::process_a(testdata("testdata/day06/input.txt").as_ref()));
    println!("Day 6 result b = {}", day06::process_b(testdata("testdata/day06/input.txt").as_ref()));
}

fn day07() {
    println!("Day 7 result a = {}", day07::process_a(testdata("testdata/day07/input.txt").as_ref()));
    println!("Day 7 result b = {}", day07::process_b(testdata("testdata/day07/input.txt").as_ref()));
}

fn day08() {
    println!("Day 8 result a = {}", day08::process_a(testdata("testdata/day08/input.txt").as_ref(), 25, 6));
    println!("Day 8 result b = {}", day08::process_b(testdata("testdata/day08/input.txt").as_ref(), 25, 6));
}

fn day09() {
    println!("Day 9 result a = {}", day09::process(testdata("testdata/day09/input.txt").as_ref(), 1));
    println!("Day 9 result a = {}", day09::process(testdata("testdata/day09/input.txt").as_ref(), 2));
}

fn day10() {
    println!("Day 10 result a = {}", day10::process_a(testdata("testdata/day10/input.txt").as_ref()));
    // Asteroid at (37, 25) has the maximum visibility of 309 asteroids, as per a
    // So directly inputting those coordinates, to reduce the burden on process_b
    println!("Day 10 result a = {}", day10::process_b(testdata("testdata/day10/input.txt").as_ref(), (37, 25), 199));
}
