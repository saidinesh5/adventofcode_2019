mod day01;
mod day02;

fn testdata(path: &str) -> String {
    use std::fs::File;
    use std::io::*;
    let mut file = File::open(path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    contents.trim().to_string()
}

fn main() {
    let days = [day01, day02];

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

