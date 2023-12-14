use std::{error::Error, collections::HashMap, path::Iter};
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let file_as_string = std::fs::read_to_string("./src/input.txt")?;
    let mut location = "AAA";

    let mut parts = file_as_string.split("\n\n");

    let directions: Vec<char> = String::from_iter(parts.next().unwrap().split('\n')).chars().collect();

    let node_lines = parts.next().unwrap().split('\n');

    let node_regex = Regex::new(r"(?P<location>[A-Z]{3}) = \((?P<left>[A-Z]{3}), (?P<right>[A-Z]{3})\)").unwrap();
    let mut locations: HashMap<String, (String, String)> = HashMap::new();
    for line in node_lines {
        if line.is_empty() {
            continue;
        }
        let m = node_regex.captures(line).unwrap();
        let loc = String::from(&m["location"]);
        let l = String::from(&m["left"]);
        let r = String::from(&m["right"]);
        locations.insert(loc, (l, r));
    }

    let mut steps = 0;
    let mut current_direction_index = 0;
    while location != "ZZZ" {
        println!("Currently at {location}");
        
        let (l, r) = &locations[location];
        let d = directions.get(current_direction_index).unwrap();
        println!("taking {d}");
        location = match d {
            'L' => l,
            'R' => r,
            _ => panic!("should never get here"),
        };
        steps += 1;
        current_direction_index += 1;
        if current_direction_index >= directions.len() {
            current_direction_index = 0;
        }
    }

    println!("{}", steps);




    return Ok(());
}
