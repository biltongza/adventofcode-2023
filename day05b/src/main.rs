use std::{collections::HashMap, error::Error, num::ParseIntError, ops::Range, thread, sync::Arc};

fn main() -> Result<(), Box<dyn Error>> {
    let file_as_string = std::fs::read_to_string("./src/input.txt")?;
    let chunks: Vec<&str> = file_as_string.split("\n\n").collect();
    let mut seeds: Vec<Range<u64>> = vec![];
    let mut all_maps: HashMap<String, Vec<(Range<u64>, Range<u64>)>> = HashMap::new();
    for chunk in chunks {
        let lines: Vec<&str> = chunk.split('\n').collect();
        let first_line = *lines.first().unwrap();
        if first_line.starts_with("seeds:") {
            seeds = parse_seeds(first_line)?;
        } else {
            let data_lines: Vec<&&str> = lines.iter().skip(1).collect();
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

    let nearest_location_threads: Vec<thread::JoinHandle<u64>> = seeds
        .into_iter()
        .map(|r| {let all_maps = Arc::new(all_maps.clone()); thread::spawn(move|| get_min_seed_location_for_range(&r, &all_maps.clone()))})
        .collect();

    let mut min = u64::MAX;
    for t in nearest_location_threads {
        let val = t.join().unwrap();
        min = min.min(val);
    }

    println!("{}", min);

    return Ok(());
}

fn parse_seeds(str: &str) -> Result<Vec<Range<u64>>, Box<dyn Error>> {
    let mut seeds: Vec<Range<u64>> = vec![];
    let parsed_numbers_maybe: Result<Vec<u64>, ParseIntError> = str
        .split(':')
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(str::parse::<u64>)
        .collect();
    let parsed_numbers = parsed_numbers_maybe.unwrap();
    let chunks = parsed_numbers.chunks(2);
    for chunk in chunks {
        let start = *chunk.get(0).unwrap();
        let length = *chunk.get(1).unwrap();
        let range = start..start + length;
        seeds.push(range);
    }
    return Ok(seeds);
}

fn parse_ranges(lines: &Vec<&&str>) -> Vec<(Range<u64>, Range<u64>)> {
    let mut ranges: Vec<(Range<u64>, Range<u64>)> = vec![];
    for &line in lines {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split_whitespace();
        let destination_range_start = parts.next().unwrap().parse::<u64>().unwrap();
        let source_range_start = parts.next().unwrap().parse::<u64>().unwrap();
        let range_length = parts.next().unwrap().parse::<u64>().unwrap();

        let source_range = source_range_start..source_range_start + range_length;
        let dest_range = destination_range_start..destination_range_start + range_length;

        ranges.push((source_range, dest_range));
    }

    return ranges;
}

fn get_min_seed_location_for_range(
    seed_range: &Range<u64>,
    all_maps: &HashMap<String, Vec<(Range<u64>, Range<u64>)>>,
) -> u64 {
    let seed_to_soil_map = all_maps.get("seed-to-soil").unwrap();
    let soil_to_fertilizer_map = all_maps.get("soil-to-fertilizer").unwrap();
    let fertilizer_to_water_map = all_maps.get("fertilizer-to-water").unwrap();
    let water_to_light_map = all_maps.get("water-to-light").unwrap();
    let light_to_temperature_map = all_maps.get("light-to-temperature").unwrap();
    let temperature_to_humidity_map = all_maps.get("temperature-to-humidity").unwrap();
    let humidity_to_location_map = all_maps.get("humidity-to-location").unwrap();
    println!("getting min loc for range {:?}", seed_range);
    let loc = seed_range
        .clone()
        .map(|seed| {
            let soil = get_mapping(seed, seed_to_soil_map);
            let fertilizer = get_mapping(soil, soil_to_fertilizer_map);
            let water = get_mapping(fertilizer, fertilizer_to_water_map);
            let light = get_mapping(water, water_to_light_map);
            let temperature = get_mapping(light, light_to_temperature_map);
            let humidity = get_mapping(temperature, temperature_to_humidity_map);
            let location = get_mapping(humidity, humidity_to_location_map);
            return location;
        })
        .min()
        .unwrap();
    println!("min loc for range {:?} is {}", seed_range, loc);
    return loc;
}

fn get_mapping(index: u64, ranges: &Vec<(Range<u64>, Range<u64>)>) -> u64 {
    for (src_range, dest_range) in ranges {
        if index >= src_range.start && index <= src_range.end {
            let offset = index - src_range.start;
            return dest_range.start + offset;
        }
    }
    return index;
}
