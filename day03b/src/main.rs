use std::{
    collections::{HashMap, HashSet},
    error::Error,
    hash::Hash,
};

fn main() -> Result<(), Box<dyn Error>> {
    let file_as_string = std::fs::read_to_string("./src/input.txt")?;
    let lines: Vec<&str> = file_as_string.lines().map(str::trim).collect();
    let mut graph: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    let mut sum = 0;
    for (line_number, line) in lines.iter().enumerate() {
        println!("line {}", line_number + 1);
        let mut active_number_buffer: Vec<char> = vec![];
        let mut active_number: Option<i32>;
        let mut start_of_active_number: Option<usize> = None;
        let mut is_part_number = false;
        // loop through each line
        for (offset, c) in line.chars().enumerate() {
            // check if we are looking at a number
            if c.is_digit(10) {
                if start_of_active_number == None {
                    start_of_active_number = Some(offset);
                }
                active_number_buffer.push(c);
            }
            if start_of_active_number != None && (!c.is_digit(10) || offset == line.len() - 1) {
                // we have found a number, now look around for a symbol

                // first parse our number
                active_number = Some(
                    active_number_buffer
                        .iter()
                        .collect::<String>()
                        .parse::<i32>()?,
                );
                println!("activeNumber = {}", active_number.unwrap());

                // figure out which lines we should search
                let start_search_line_number = if line_number == 0 { 0 } else { line_number - 1 };
                let take = if line_number == 0 { 2 } else { 3 };

                // figure out bounds to search within the line
                let search_offset = if offset == 0 || start_of_active_number == Some(0) {
                    0
                } else {
                    start_of_active_number.unwrap() - 1
                };
                let search_size = active_number_buffer.len() + 2;

                active_number_buffer.clear();
                start_of_active_number = None;

                for (search_line_number, search_line) in lines
                    .iter()
                    .skip(start_search_line_number)
                    .take(take)
                    .enumerate()
                {
                    // get our actual search data
                    let search_space: Vec<char> = search_line
                        .chars()
                        .skip(search_offset)
                        .take(search_size)
                        .collect();
                    println!("searching {:?}", search_space);

                    for (search_index, search_character) in search_space.iter().enumerate() {
                        let y = search_line_number + start_search_line_number;
                        let x = search_offset + search_index;
                        if *search_character == '*' {
                            println!(
                                "found gear at {},{} pushing {}",
                                x,
                                y,
                                active_number.unwrap()
                            );
                            graph
                                .entry((x, y))
                                .or_insert(vec![])
                                .push(active_number.unwrap());
                            break;
                        }
                    }
                    if is_part_number {
                        is_part_number = false;
                        break;
                    }
                }
            }
        }
    }

    println!("{:?}", graph);
    for (_, gear_ratios) in graph {
        if (gear_ratios.len() == 2) {
            let mult = gear_ratios[0] * gear_ratios[1];
            sum += mult;
        }
    }

    println!("{sum}");
    return Ok(());
}
