mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn testdata(path: &str) -> String {
    use std::fs::File;
    use std::io::*;
    let mut file = File::open(path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    contents.trim().to_string()
}

fn main() {
    let days = [day01, day02, day03, day04, day05, day06];

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

