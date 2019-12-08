type Point = (isize, isize);

pub fn process_a(text: &str) -> isize {
    let wires = wire_points(text);

    if wires.len() != 2 || wires[0].len() < 3 || wires[1].len() < 3 {
        println!("Invalid Input! {}", text);
        return 0;
    }

    let left_wire = &wires[0];
    let right_wire = &wires[1];

    // Intersections = vector((manhattan distance, point))
    let mut intersections = Vec::new();

    // Just a dumb brute force approach, given this data
    for i in 1..left_wire.len() {
        for j in 1..right_wire.len() {
            match intersection(left_wire[i-1], left_wire[i], right_wire[j-1], right_wire[j]) {
                Some(p) => {
                    intersections.push((p.0.abs() + p.1.abs(), p));
                    break;
                },
                None => continue
            }
        }
    }

    intersections.sort();
    // Blindly assume there is an intersection other than 0
    intersections[1].0
}

pub fn process_b(text: &str) -> isize {
    let wires = wire_points(text);

    if wires.len() != 2 || wires[0].len() < 3 || wires[1].len() < 3 {
        println!("Invalid Input! {}", text);
        return 0;
    }

    let left_wire = &wires[0];
    let right_wire = &wires[1];

    // Intersections = vector((total steps, point))
    let mut intersections = Vec::new();

    let mut left_steps = 0;
    for i in 1..left_wire.len() {

        let mut right_steps = 0;
        for j in 1..right_wire.len() {

            match intersection(left_wire[i-1], left_wire[i], right_wire[j-1], right_wire[j]) {
                Some(p) => {
                    let delta_p_l1 = (p.0 - left_wire[i-1].0 + p.1 - left_wire[i-1].1).abs();
                    let delta_p_r1 = (p.0 - right_wire[j-1].0 + p.1 - right_wire[j-1].1).abs();
                    intersections.push((left_steps + right_steps + delta_p_l1 + delta_p_r1, p));
                    break;
                },
                None => {
                    right_steps += (right_wire[j].0 - right_wire[j-1].0 + right_wire[j].1 - right_wire[j-1].1).abs()
                }
            }
        }

        left_steps += (left_wire[i].0 - left_wire[i-1].0 + left_wire[i].1 - left_wire[i-1].1).abs();
    }

    intersections.sort();
    // Blindly assume there is an intersection other than 0
    intersections[1].0
}

fn intersection(l1: Point, l2: Point, r1: Point, r2: Point) -> Option<Point> {
    if std::cmp::min(r1.0, r2.0) <= l1.0 && l1.0 <= std::cmp::max(r1.0, r2.0) 
           && std::cmp::min(l1.1, l2.1) <= r1.1 && r1.1 <= std::cmp::max(l1.1, l2.1) {
        Some((l1.0, r1.1))
    } else if std::cmp::min(l1.0, l2.0) <= r1.0 && r1.0 <= std::cmp::max(l1.0, l2.0) 
                  && std::cmp::min(r1.1, r2.1) <= l1.1 && l1.1 <= std::cmp::max(r1.1, r2.1) {
        Some((r1.0, l1.1))
    }  else {
        // Deliberately ignoring parallel lines
        None
    }
}

fn wire_points (text: &str) -> Vec<Vec<Point>> {
    let wire_commands = text.lines()
                            .filter(|&line| line.len() > 0)
                            .map(|line| line.split(',')
                                            .map(|item| String::from(item.trim()))
                                            .filter(|item| item.len() > 0)
                                            .map(|item| {
                                                let value = (item[1..]).parse::<isize>().unwrap();

                                                match item.chars().next().unwrap() {
                                                'R' => (value, 0),
                                                'L' => (-value, 0),
                                                'U' => (0, value),
                                                'D' => (0, -value),
                                                _   => (0, 0)
                                            }}).collect::<Vec<Point>>())
                            .collect::<Vec<Vec<Point>>>();

    wire_commands.iter().map(|commands| {
                                            let mut result = vec![(0, 0)];
                                            let mut current = (0, 0);

                                            for command in commands.iter() {
                                                let p = (current.0 + command.0, current.1 + command.1);
                                                result.push(p);
                                                current = p;
                                            }

                                            result
                                        })
                        .collect::<Vec<Vec<Point>>>()
            
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(process_a("R8,U5,L5,D3\n
        U7,R6,D4,L4"), 6);
        assert_eq!(process_a("R75,D30,R83,U83,L12,D49,R71,U7,L72\n
        U62,R66,U55,R34,D71,R55,D58,R83"), 159);
        assert_eq!(process_a("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n
        U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), 135);
    }

    #[test]
    fn test_b() {
        assert_eq!(process_b("R8,U5,L5,D3\n
        U7,R6,D4,L4"), 30);
        assert_eq!(process_b("R75,D30,R83,U83,L12,D49,R71,U7,L72\n
        U62,R66,U55,R34,D71,R55,D58,R83"), 610);
        assert_eq!(process_b("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n
        U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), 410);
    }
}