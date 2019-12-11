use num_integer::gcd;
use std::collections::HashMap;
use std::collections::HashSet;

// We are going to use this type to represent both slopes and points
type Pair = (isize, isize);

pub fn process_a(text: &str) -> usize {
    let points = asteroid_coordinates(text);

    // For each point, compute the number of unique slopes with other points
    let slopes_per_point = points.iter()
                                 .map(|p| {
                                     slopes_with_point(*p, &points).iter()
                                                                   .map(|item| item.1)
                                                                   .collect::<HashSet<Pair>>()
                                                                   .len()
                                    })
                                 .collect::<Vec<usize>>();

    let point_with_max_slopes = slopes_per_point.iter().enumerate().max_by_key(|&(_, value)| value).unwrap();

    // println!("Asteroid at {:?} has the maximum visibility of {} asteroids",
    //          asteroid_coordinates[point_with_max_slopes.0], *point_with_max_slopes.1);
    *point_with_max_slopes.1
}

pub fn process_b(text: &str, start: (isize, isize), target: usize) -> isize {
    let points = asteroid_coordinates(text);
    let index_x = points.iter().position(|&p| p == start).unwrap();
    let x = points[index_x];

    let mut slopes_of_points = slopes_with_point(x, &points);

    // Sort by distance from x
    slopes_of_points.sort_by_key(|&(index_p, _)| {
        let p = points[index_p];
        (p.0 - x.0)*(p.0 - x.0) + (p.1 - x.1)*(p.1 - x.1)
    });

    // Map of (slope) -> [index of point with slope 1, index of point with slope 2, ...]
    let mut points_per_slope: HashMap<&Pair, Vec<usize>> = HashMap::new();

    for p in slopes_of_points.iter() {
        let index = p.0;
        if points_per_slope.contains_key(&(p.1)) {
            points_per_slope.get_mut(&(p.1)).unwrap().push(index);
        } else {
            points_per_slope.insert(&(p.1), vec![index]);
        }
    }

    let mut slopes = points_per_slope.keys()
                                     .map(|&p| *p)
                                     .collect::<Vec<Pair>>();

    // Sort the slopes by their clockwise angle with up vector, starting at x
    slopes.sort_by_key(|s| {
        // Up vector = (0, x.1)
        // Slope vector = (s.0, s.1)
        let dot = (x.1*s.1) as f64;
        let det = (-x.1*s.0) as f64;
        let mut angle = dot.atan2(det);
        angle = if angle < 0.0 { 2.0*std::f64::consts::PI + angle } else { angle };

        // Since rust doesn't let us sort floats :/
        (angle*10000000000000.0) as i64
    });

    let mut vaporized_points: Vec<usize> = Vec::new();

    // We vaporize all the points one, by one.. except for x itself
    while vaporized_points.len() < points.len() - 1 {
        for slope in &slopes {
            let vec = points_per_slope.get_mut(slope).unwrap();
            if !vec.is_empty() {
                vaporized_points.push(vec.remove(0));
            }
        }
    }

    assert!(vaporized_points.len() > target);
    let target_point = points[vaporized_points[target]];
    100*target_point.0 + target_point.1
}

// Take the grid of .'s and #'s and return the coordinates(column, row) of points where #'s exist
fn asteroid_coordinates(text: &str) -> Vec<Pair> {
    text.lines()
        .filter(|line| line.len() > 0)
        .enumerate()
        .flat_map(|(row, line)| {
            line.trim()
                .bytes()
                .enumerate()
                .map(|(column, c)| {
                    if c as char == '#' { Some((column as isize, row as isize)) }
                    else { None }
                })
                .filter_map(|item| item)
                .collect::<Vec<Pair>>()
            })
        .collect::<Vec<Pair>>()
}

// Returns a vector[(slope of a point, index of a point)]
fn slopes_with_point(x: Pair, points: &Vec<Pair>) -> Vec<(usize, Pair)> {
    points.iter()
          .enumerate()
          .filter_map(|(index_p, &p)| {
              if p != x {
                  // We are computing slope as pair
                  // to avoid floating point/division by zero mess
                  let num = p.1 - x.1;
                  let den = p.0 - x.0;
                  let g = gcd(num, den);
                  let slope = (num/g, den/g);
                  Some((index_p, slope))
                }
              else { None }
            })
          .collect::<Vec<(usize, Pair)>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        // Best is 3,4 because it can detect 8 asteroids
        assert_eq!(process_a("\n\
        .#..#\n\
        .....\n\
        #####\n\
        ....#\n\
        ...##"), 8);

        // Best is 5,8 with 33 other asteroids detected
        assert_eq!(process_a("\n\
        ......#.#.\n\
        #..#.#....\n\
        ..#######.\n\
        .#.#.###..\n\
        .#..#.....\n\
        ..#....#.#\n\
        #..#....#.\n\
        .##.#..###\n\
        ##...#..#.\n\
        .#....####"), 33);

        // Best is 6,3 with 41 other asteroids detected
        assert_eq!(process_a("\n\
        .#..#..###\n\
        ####.###.#\n\
        ....###.#.\n\
        ..###.##.#\n\
        ##.##.#.#.\n\
        ....###..#\n\
        ..#.#..#.#\n\
        #..#.#.###\n\
        .##...##.#\n\
        .....#.#.."), 41);

        // Best is 11,13 with 210 other asteroids detected
        assert_eq!(process_a("\n\
        .#..##.###...#######\n\
        ##.############..##.\n\
        .#.######.########.#\n\
        .###.#######.####.#.\n\
        #####.##.#.##.###.##\n\
        ..#####..#.#########\n\
        ####################\n\
        #.####....###.#.#.##\n\
        ##.#################\n\
        #####.##.###..####..\n\
        ..######..##.#######\n\
        ####.##.####...##..#\n\
        .#####..#.######.###\n\
        ##...#.##########...\n\
        #.##########.#######\n\
        .####.#.###.###.#.##\n\
        ....##.##.###..#####\n\
        .#.#.###########.###\n\
        #.#.#.#####.####.###\n\
        ###.##.####.##.#..##"), 210);
    }

    #[test]
    fn test_b() {
        assert_eq!(process_b("\n\
        .#....#####...#..\n\
        ##...##.#####..##\n\
        ##...#...#.#####.\n\
        ..#.....#...###..\n\
        ..#.#.....#....##", (8, 3), 1), 900);

        // Best is 11,13 with 210 other asteroids detected
        // The 200th asteroid to be vaporized is at 8,2
        assert_eq!(process_b("\n\
        .#..##.###...#######\n\
        ##.############..##.\n\
        .#.######.########.#\n\
        .###.#######.####.#.\n\
        #####.##.#.##.###.##\n\
        ..#####..#.#########\n\
        ####################\n\
        #.####....###.#.#.##\n\
        ##.#################\n\
        #####.##.###..####..\n\
        ..######..##.#######\n\
        ####.##.####...##..#\n\
        .#####..#.######.###\n\
        ##...#.##########...\n\
        #.##########.#######\n\
        .####.#.###.###.#.##\n\
        ....##.##.###..#####\n\
        .#.#.###########.###\n\
        #.#.#.#####.####.###\n\
        ###.##.####.##.#..##", (11, 13), 199), 802);
    }

}