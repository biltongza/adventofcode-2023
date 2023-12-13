use std::{error::Error, num::ParseIntError};

fn main() -> Result<(), Box<dyn Error>> {
    let file_as_string = std::fs::read_to_string("./src/input.txt")?;
    let mut lines = file_as_string.lines();
    let time_line = lines.next().unwrap();
    let times_result: Result<Vec<u64>, ParseIntError> = time_line
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(str::parse::<u64>)
        .collect();
    let times = times_result.unwrap();
    let distance_line = lines.next().unwrap();
    let distance_result: Result<Vec<u64>, ParseIntError> = distance_line
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(str::parse::<u64>)
        .collect();
    let distances = distance_result.unwrap();
    println!("{:?}", times);
    println!("{:?}", distances);

    let mut margin = 1;
    for (index, time) in times.iter().enumerate() {
        let mut count = 0;
        let record_distance = *distances.get(index).unwrap();
        let mut iterations: Vec<(u64, u64)> = vec![];
        for i in 0..*time+1 {
            let time_button_held = i;
            let distance = (time - time_button_held) * time_button_held;
            if distance > record_distance {
                count += 1;
            }
        }
        margin *= count;
    }

    println!("{}", margin);
    return Ok(());
}
