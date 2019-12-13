use std::convert::From;
use std::cell::RefCell;
use std::collections::HashSet;
use num_integer::lcm;

pub fn process_a(text: &str, steps: usize) -> usize {
    // TODO: Is there a better way to do this than to use a RefCell?
    // Had to use a RefCell so as to be able to do:
    // moons[i].borrow_mut().apply_gravity(&moons[j].borrow());
    let moons = text.lines()
                    .filter(|line| line.len() > 0)
                    .map(|line| RefCell::new(Moon::from(line.trim())))
                    .collect::<Vec<RefCell<Moon>>>();

    for _ in 0..steps {

        for i in 0..moons.len() {
            // First apply all the gravity
            for j in 0..moons.len() {
                if i != j {
                    moons[i].borrow_mut().apply_gravity(&moons[j].borrow());
                }
            }
        }

        // Then apply all the velocity, so as to not double add things
        for moon in moons.iter() {
            moon.borrow_mut().apply_velocity();
        }

    }

    moons.iter()
         .map(|moon| moon.borrow().total_energy())
         .sum::<usize>()
}

pub fn process_b(text: &str) -> usize {
    let moons = text.lines()
                    .filter(|line| line.len() > 0)
                    .map(|line| Moon::from(line.trim()))
                    .collect::<Vec<Moon>>();

    // TODO: Can this function be done better/faster using some ninja math?
    fn cycle_time(positions: Vec<isize>) -> usize {
        let mut state = positions.iter()
                                 .cloned()
                                 .chain(vec![0; positions.len()].into_iter())
                                 .collect::<Vec<isize>>();
        let mut steps = 0;
        let mut bag = HashSet::new();

        bag.insert(state.clone());

        let position = |i| i;
        let velocity = |i| i + positions.len();

        loop {
            // Apply gravity
            for i in 0..positions.len() {
                for j in 0..positions.len() {
                    if i != j {
                        state[velocity(i)] += signum(state[position(j)] - state[position(i)]);
                    }
                }
            }

            // Apply velocity
            for i in 0..positions.len() {
                state[position(i)] += state[velocity(i)];
            }

            steps += 1;

            if bag.contains(&state) {
                break;
            } else {
                bag.insert(state.clone());
            }
        }

        steps
    }

    // Cycle times for each of the axes
    let cycle_times = (0..3).map(|i| {
                                        let mut result = Vec::new();
                                        for moon in moons.iter() {
                                            result.push(moon.position[i]);
                                        }
                                        cycle_time(result)
                                    })
                            .collect::<Vec<usize>>();

    cycle_times.iter()
               .fold(cycle_times[0], |current_lcm, &value| lcm(current_lcm, value))
}

type Vector3 = [isize; 3];

fn signum(value: isize) -> isize {
    if value > 0 { 1 }
    else if value == 0 { 0 }
    else { -1 }
}

#[derive(Debug)]
struct Moon {
    position: Vector3,
    velocity: Vector3
}

impl Moon {
    fn apply_gravity(&mut self, other: &Self) {
        for i in 0..self.velocity.len() {
            self.velocity[i] += signum(other.position[i] - self.position[i]);
        }
    }

    fn apply_velocity(&mut self) {
        for i in 0..self.position.len() {
            self.position[i] += self.velocity[i];
        }
    }

    fn total_energy(&self) -> usize {
        let potential_energy = self.position.iter().map(|i| i.abs() as usize).sum::<usize>();
        let kinetic_energy = self.velocity.iter().map(|i| i.abs() as usize).sum::<usize>();
        potential_energy * kinetic_energy
    }
}

impl From<&str> for Moon {
    /// Constructs a moon from a line like: "<x=-1, y=0, z=2>"
    fn from(text: &str) -> Self {
        assert!(text.len() > 2);
        let t = text[1..text.len() - 1].split(',')
                                       .filter(|s| s.len() > 0)
                                       .map(|item| item.split('=')
                                                       .nth(1)
                                                       .unwrap()
                                                       .trim()
                                                       .parse::<isize>().unwrap())
                                       .collect::<Vec<isize>>();
        assert!(t.len() == 3);
        Moon {
            position: [t[0], t[1], t[2]],
            velocity: [0, 0, 0]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_a() {

        // Example 1
        assert_eq!(process_a("<x=-1, y=0, z=2>\n\
        <x=2, y=-10, z=-7>\n\
        <x=4, y=-8, z=8>\n\
        <x=3, y=5, z=-1>", 10), 179);


        assert_eq!(process_a("<x=-8, y=-10, z=0>\n\
        <x=5, y=5, z=10>\n\
        <x=2, y=-7, z=3>\n\
        <x=9, y=-8, z=-3>", 100), 1940);
    }

    #[test]
    fn test_b() {

        assert_eq!(process_b("<x=-1, y=0, z=2>\n\
        <x=2, y=-10, z=-7>\n\
        <x=4, y=-8, z=8>\n\
        <x=3, y=5, z=-1>"), 2772);


        assert_eq!(process_b("<x=-8, y=-10, z=0>\n\
        <x=5, y=5, z=10>\n\
        <x=2, y=-7, z=3>\n\
        <x=9, y=-8, z=-3>"), 4686774924);
    }
}