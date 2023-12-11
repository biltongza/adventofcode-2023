use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_as_string = std::fs::read_to_string("./src/input.txt")?;
    let lines: Vec<&str> = file_as_string.lines().map(str::trim).collect();
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
            if start_of_active_number != None && (!c.is_digit(10) || offset == line.len()-1) {
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
                let take = if line_number == 0 {2} else {3};
                
                // figure out bounds to search within the line
                let search_offset = if offset == 0 || start_of_active_number == Some(0) {
                    0
                } else {
                    start_of_active_number.unwrap() - 1
                };
                let search_size = active_number_buffer.len() + 2;

                active_number_buffer.clear();
                start_of_active_number = None;


                for line_to_search in lines.iter().skip(start_search_line_number).take(take) {
                    // get our actual search data
                    let search_space: Vec<char> = line_to_search
                        .chars()
                        .skip(search_offset)
                        .take(search_size).collect();
                    println!("searching {:?}", search_space);

                    for search_character in search_space {
                        if search_character != '.' && !search_character.is_digit(10) {
                            is_part_number = true;
                            println!("{} is a part number", active_number.unwrap());
                            sum += active_number.unwrap();
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

    println!("{sum}");
    return Ok(());
}
