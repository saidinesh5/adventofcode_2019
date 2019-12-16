use regex::Regex;
use std::collections::HashMap;

pub fn process_a(text: &str) -> u64 {
    get_ore_required(&get_elements(text), &mut HashMap::new(), &String::from("FUEL"), 1)
}

pub fn process_b(text: &str) -> u64 {
    let elements = get_elements(text);
    let available_ore = 1000000000000 as u64;

    let mut minimum_fuel = 0 as u64;
    let mut maximum_fuel = 1000000000000 as u64;
    let mut maximum_fuel_produced = 0;

    // Given a function that tells us how much ore we need to produce a given quantity of fuel, simply binary search
    // from 0 to available ore. It should still take like less than 64 turns to get the answer we need
    while minimum_fuel < maximum_fuel {
        let fuel_to_produce = (maximum_fuel + minimum_fuel)/2;
        let ore_required = get_ore_required(&elements, &mut HashMap::new(), &String::from("FUEL"), fuel_to_produce);

        if ore_required == available_ore {
            maximum_fuel_produced = fuel_to_produce;
            break;
        } else if ore_required > available_ore {
            maximum_fuel = fuel_to_produce - 1;
        } else {
            minimum_fuel = fuel_to_produce + 1;
            maximum_fuel_produced = std::cmp::max(fuel_to_produce, maximum_fuel_produced);
        }
    }

    maximum_fuel_produced
}

fn get_elements(text: &str) -> HashMap<String, Element> {
    text.lines()
        .filter(|line| line.trim().len() > 0)
        .map(|line| {
                        let e = Element::from(line);
                        (e.name.clone(), e)
                    })
        .collect::<HashMap<String, Element>>()

}

fn get_ore_required(elements: &HashMap<String, Element>,
                    available_elements: &mut HashMap<String, u64>,
                    element_name: &String, required_quantity: u64) -> u64 {
    if element_name == "ORE" {
        return required_quantity;
    }

    let ore_required: u64;
    let available_quantity = *available_elements.get(element_name).unwrap_or(&0);

    if available_quantity >= required_quantity {
        ore_required = 0;
        available_elements.insert(element_name.clone(), available_quantity - required_quantity);
    } else {
        let element = elements.get(element_name).unwrap();
        let quantity_to_produce = required_quantity - available_quantity;
        let reactions_required = ((quantity_to_produce as f64)/(element.minimum_quanitity_produced as f64)).ceil() as u64;

        // Compute the amount of ore required for this element
        ore_required = element.elements_required
                              .iter()
                              .map(|d| get_ore_required(&elements, available_elements, &d.1, reactions_required*d.0))
                              .sum::<u64>();

        // Store the excess of Ore as available for other future requests
        available_elements.insert(element_name.clone(), reactions_required*element.minimum_quanitity_produced - quantity_to_produce);
    }

    ore_required
}

#[derive(Debug)]
struct Element {
    name: String,
    minimum_quanitity_produced: u64,
    elements_required: Vec<(u64, String)>
}

impl Element {
    fn from(line: &str) -> Self {
        let mut result = Element {
            name: String::from(""),
            minimum_quanitity_produced: 1,
            elements_required: Vec::new()
        };

        lazy_static! {
            static ref RE: Regex = Regex::new(r"((\d+) ([[:alpha:]]+))+").unwrap();
        }

        let captures = RE.captures_iter(line);

        for capture in captures {
            result.elements_required.push((capture[2].parse::<u64>().unwrap(), String::from(&capture[3])))
        }

        let e = result.elements_required.pop().unwrap();
        result.minimum_quanitity_produced = e.0;
        result.name = e.1;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_a() {

        assert_eq!(process_a("10 ORE => 10 A\n\
        1 ORE => 1 B\n\
        7 A, 1 B => 1 C\n\
        7 A, 1 C => 1 D\n\
        7 A, 1 D => 1 E\n\
        7 A, 1 E => 1 FUEL"), 31);

        assert_eq!(process_a("9 ORE => 2 A\n\
        8 ORE => 3 B\n\
        7 ORE => 5 C\n\
        3 A, 4 B => 1 AB\n\
        5 B, 7 C => 1 BC\n\
        4 C, 1 A => 1 CA\n\
        2 AB, 3 BC, 4 CA => 1 FUEL"), 165);

        assert_eq!(process_a("157 ORE => 5 NZVS\n\
        165 ORE => 6 DCFZ\n\
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n\
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n\
        179 ORE => 7 PSHF\n\
        177 ORE => 5 HKGWZ\n\
        7 DCFZ, 7 PSHF => 2 XJWVT\n\
        165 ORE => 2 GPVTF\n\
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"), 13312);

        assert_eq!(process_a("2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n\
        17 NVRVD, 3 JNWZP => 8 VPVL\n\
        53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n\
        22 VJHF, 37 MNCFX => 5 FWMGM\n\
        139 ORE => 4 NVRVD\n\
        144 ORE => 7 JNWZP\n\
        5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n\
        5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n\
        145 ORE => 6 MNCFX\n\
        1 NVRVD => 8 CXFTF\n\
        1 VJHF, 6 MNCFX => 4 RFSQX\n\
        176 ORE => 6 VJHF"), 180697);

        assert_eq!(process_a("171 ORE => 8 CNZTR\n\
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n\
        114 ORE => 4 BHXH\n\
        14 VRPVC => 6 BMBT\n\
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n\
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n\
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n\
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n\
        5 BMBT => 4 WPTQ\n\
        189 ORE => 9 KTJDG\n\
        1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n\
        12 VRPVC, 27 CNZTR => 2 XDBXC\n\
        15 KTJDG, 12 BHXH => 5 XCVML\n\
        3 BHXH, 2 VRPVC => 7 MZWV\n\
        121 ORE => 7 VRPVC\n\
        7 XCVML => 6 RJRHP\n\
        5 BHXH, 4 VRPVC => 5 LTCX"), 2210736);
    }

    #[test]
    fn test_b() {
        // The 13312 ORE-per-FUEL example could produce 82892753 FUEL
        assert_eq!(process_b("157 ORE => 5 NZVS\n\
        165 ORE => 6 DCFZ\n\
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n\
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n\
        179 ORE => 7 PSHF\n\
        177 ORE => 5 HKGWZ\n\
        7 DCFZ, 7 PSHF => 2 XJWVT\n\
        165 ORE => 2 GPVTF\n\
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"), 82892753);

        // The 180697 ORE-per-FUEL example could produce 5586022 FUEL
        assert_eq!(process_b("2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n\
        17 NVRVD, 3 JNWZP => 8 VPVL\n\
        53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n\
        22 VJHF, 37 MNCFX => 5 FWMGM\n\
        139 ORE => 4 NVRVD\n\
        144 ORE => 7 JNWZP\n\
        5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n\
        5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n\
        145 ORE => 6 MNCFX\n\
        1 NVRVD => 8 CXFTF\n\
        1 VJHF, 6 MNCFX => 4 RFSQX\n\
        176 ORE => 6 VJHF"), 5586022);

        // The 2210736 ORE-per-FUEL example could produce 460664 FUEL
        assert_eq!(process_b("171 ORE => 8 CNZTR\n\
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n\
        114 ORE => 4 BHXH\n\
        14 VRPVC => 6 BMBT\n\
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n\
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n\
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n\
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n\
        5 BMBT => 4 WPTQ\n\
        189 ORE => 9 KTJDG\n\
        1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n\
        12 VRPVC, 27 CNZTR => 2 XDBXC\n\
        15 KTJDG, 12 BHXH => 5 XCVML\n\
        3 BHXH, 2 VRPVC => 7 MZWV\n\
        121 ORE => 7 VRPVC\n\
        7 XCVML => 6 RJRHP\n\
        5 BHXH, 4 VRPVC => 5 LTCX"), 460664);
    }
}
