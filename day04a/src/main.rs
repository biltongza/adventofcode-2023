use std::{collections::HashSet, error::Error};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let file_as_string = std::fs::read_to_string("./src/input.txt")?;
    let lines: Vec<&str> = file_as_string.lines().map(str::trim).collect();
    let re: Regex = Regex::new(r"^.+?(?P<cardnum>\d+): (?P<wins>.+)\|(?P<nums>.+)").unwrap();
    let mut sum = 0;
    for line in lines {
        let captures = re.captures(line).unwrap();
        let card_num = &captures["cardnum"].parse::<i32>().unwrap();
        let winning_nums = split_nums(&captures["wins"]);
        let nums = split_nums(&captures["nums"]);

        let matches: Vec<_> = nums
            .iter()
            .filter(|num| winning_nums.contains(num))
            .collect();
        
        let num_matches = matches.len();
        let mut score = 0;
        for n in 0..num_matches {
            score = if score == 0 {1} else {usize::pow(2, n as u32)};
        }
        
        println!("card {} matches {} score {}", card_num, matches.len(), score);
        sum += score;
    }

    println!("{}", sum);
    return Ok(());
}

fn split_nums(str: &str) -> HashSet<i32> {
    return HashSet::from_iter(
        str.trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap()),
    );
}
