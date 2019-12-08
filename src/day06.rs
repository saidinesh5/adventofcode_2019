use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn process_a(text: &str) -> usize {
    let graph = to_adjacency_list(text, GraphType::DirectedGraph);

    // Something to speed up the computation
    let mut orbit_count_cache: HashMap<String, usize> = HashMap::new();

    graph.keys()
         .fold(0, |sum, key| sum + orbit_count(key, &graph, &mut orbit_count_cache))
}

pub fn process_b(text: &str) -> usize {
    let graph = to_adjacency_list(text, GraphType::UndirectedGraph);

    // A simple breadth first search
    let mut keys_to_visit = VecDeque::new();
    let mut visited_keys = HashSet::new();

    keys_to_visit.push_back(("YOU", 0));

    while keys_to_visit.len() > 0 {
        let (key, steps) =  keys_to_visit.pop_front().unwrap();

        visited_keys.insert(key);

        if graph.contains_key(key) {
            for item in graph.get(key).unwrap().iter() {
                if item == "SAN" {
                    // -1 because we want to start from the orbit connected to YOU and we started from YOU
                    return steps - 1;
                }
                else {
                    if !visited_keys.contains(&item.as_ref()) {
                        keys_to_visit.push_back((item, 1 + steps))
                    }
                }
            }
        }
    }

    return 0;
}

fn orbit_count(key: &str, graph: &HashMap<String, Vec<String>>, cache: &mut HashMap<String, usize>) -> usize {
    if cache.contains_key(key) {
        return *cache.get(key).unwrap();
    }
    else if !graph.contains_key(key) {
        cache.insert(String::from(key), 0);
        return 0;
    } else {
        let result = 1 + graph.get(key)
                              .unwrap().iter()
                                       .fold(0, |sum, item| sum + orbit_count(item, &graph, cache));
        cache.insert(String::from(key), result);
        result
    }
}

#[derive(PartialEq)]
enum GraphType {
    DirectedGraph,
    UndirectedGraph
}

fn to_adjacency_list(text: &str, graph_type: GraphType) -> HashMap<String, Vec<String>> {
    // Link is the vector of edges in the graph
    // A)B => [(B, A)] => B Orbits around A
    let links = text.lines()
                    .filter(|&line| line.len() > 0)
                    .map(|line| {
                        let components = line.split(')')
                                             .collect::<Vec<&str>>();
                        (String::from(components[1]), String::from(components[0]))
                    })
                    .collect::<Vec<(String, String)>>();

    // graph is a simple adjacency list of key -> connected keys
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for (key, value) in links.iter() {
        if graph.contains_key(key) {
            graph.get_mut(key).unwrap().push(value.to_string());
        }
        else {
            graph.insert(key.to_string(), vec![value.to_string()]);
        }

        if graph_type == GraphType::UndirectedGraph {
            if graph.contains_key(value) {
                graph.get_mut(value).unwrap().push(key.to_string());
            }
            else {
                graph.insert(value.to_string(), vec![key.to_string()]);
            }
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_a() {
        // For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
        assert_eq!(process_a("\
        COM)B\n\
        B)C\n\
        C)D\n\
        D)E\n\
        E)F\n\
        B)G\n\
        G)H\n\
        D)I\n\
        E)J\n\
        J)K\n\
        K)L"), 42);
    }

    #[test]
    fn test_b() {
        assert_eq!(process_b("\
        COM)B\n\
        B)C\n\
        C)D\n\
        D)E\n\
        E)F\n\
        B)G\n\
        G)H\n\
        D)I\n\
        E)J\n\
        J)K\n\
        K)L\n\
        K)YOU\n\
        I)SAN"), 4);
    }
}