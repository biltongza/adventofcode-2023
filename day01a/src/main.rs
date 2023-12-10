
use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;

fn main() -> Result<(), Error> {
    let lines = read_lines("./src/input.txt")?;
    let sum: u32 = lines.map(|line| get_num_from_line(line)).sum();
    println!("sum = {}", sum);
    Ok(())
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_num_from_line(line: Result<String, Error>) -> u32 {
    let unwrapped = line.unwrap();
    let numbers: Vec<&str> = unwrapped
        .split(char::is_alphabetic)
        .filter(|x| !&x.is_empty())
        .collect();

    println!("line = {}", unwrapped);
    println!{"split = {:?}", numbers};
    let firstDigit = numbers.first().unwrap().chars().next().unwrap();
    let lastDigit = numbers.last().unwrap().chars().nth_back(0).unwrap();
    format!("{}{}", firstDigit, lastDigit).parse::<u32>().unwrap()
}