use std::{collections::HashMap, error::Error, num::ParseIntError};

struct Range {
    start: u64,
    count: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_as_string = std::fs::read_to_string("./src/input.txt")?;
    let chunks: Vec<&str> = file_as_string.split("\n\n").collect();
    let mut seeds: Vec<u64> = vec![];
    let mut all_maps: HashMap<String, Vec<(Range, Range)>> = HashMap::new();
    for chunk in chunks {
        let lines: Vec<&str> = chunk.split('\n').collect();
        let first_line = *lines.first().unwrap();
        if first_line.starts_with("seeds:") {
            seeds = parse_seeds(first_line)?;
        } else {
            let data_lines: Vec<&&str> = lines.iter().skip(1).collect();
            println!("{}", first_line);
            match first_line as &str {
                "seed-to-soil map:" => {
                    let ranges = parse_ranges(&data_lines);
                    all_maps.insert(String::from("seed-to-soil"), ranges);
                }
                "soil-to-fertilizer map:" => {
                    let ranges = parse_ranges(&data_lines);
                    all_maps.insert(String::from("soil-to-fertilizer"), ranges);
                }
                "fertilizer-to-water map:" => {
                    let ranges = parse_ranges(&data_lines);
                    all_maps.insert(String::from("fertilizer-to-water"), ranges);
                }
                "water-to-light map:" => {
                    let ranges = parse_ranges(&data_lines);
                    all_maps.insert(String::from("water-to-light"), ranges);
                }
                "light-to-temperature map:" => {
                    let ranges = parse_ranges(&data_lines);
                    all_maps.insert(String::from("light-to-temperature"), ranges);
                }
                "temperature-to-humidity map:" => {
                    let ranges = parse_ranges(&data_lines);
                    all_maps.insert(String::from("temperature-to-humidity"), ranges);
                }
                "humidity-to-location map:" => {
                    let ranges = parse_ranges(&data_lines);
                    all_maps.insert(String::from("humidity-to-location"), ranges);
                }
                _ => panic!("missing map implementation {}", first_line),
            }
        }
    }

    let nearest_location = seeds
        .iter()
        .map(|x| (*x, get_seed_location(*x, &all_maps)))
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    println!("{:#?}", nearest_location);

    return Ok(());
}

fn parse_seeds(str: &str) -> Result<Vec<u64>, ParseIntError> {
    str.split(':')
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(str::parse::<u64>)
        .collect()
}

fn parse_ranges(lines: &Vec<&&str>) -> Vec<(Range, Range)> {
    let mut ranges: Vec<(Range, Range)> = vec![];
    for &line in lines {
        if line.is_empty() {
            continue;
        }
        println!("{}", line);
        let mut parts = line.split_whitespace();
        let destination_range_start = parts.next().unwrap().parse::<u64>().unwrap();
        let source_range_start = parts.next().unwrap().parse::<u64>().unwrap();
        let range_length = parts.next().unwrap().parse::<u64>().unwrap();

        let source_range = Range {
            start: source_range_start,
            count: range_length,
        };
        let dest_range = Range {
            start: destination_range_start,
            count: range_length,
        };

        ranges.push((source_range, dest_range));
    }

    return ranges;
}

fn get_seed_location(seed: u64, all_maps: &HashMap<String, Vec<(Range, Range)>>) -> u64 {
    println!("getting seed location for {seed}");
    let soil = get_mapping(seed, all_maps.get("seed-to-soil").unwrap());
    let fertilizer = get_mapping(soil, all_maps.get("soil-to-fertilizer").unwrap());
    let water = get_mapping(fertilizer, all_maps.get("fertilizer-to-water").unwrap());
    let light = get_mapping(water, all_maps.get("water-to-light").unwrap());
    let temperature = get_mapping(light, all_maps.get("light-to-temperature").unwrap());
    let humidity = get_mapping(
        temperature,
        all_maps.get("temperature-to-humidity").unwrap(),
    );
    let location = get_mapping(humidity, all_maps.get("humidity-to-location").unwrap());
    return location;
}

fn get_mapping(index: u64, ranges: &Vec<(Range, Range)>) -> u64 {
    for (src_range, dest_range) in ranges {
        if index >= src_range.start && index <= src_range.start + src_range.count {
            let offset = index - src_range.start;
            return dest_range.start + offset;
        }
    }
    return index;
}
