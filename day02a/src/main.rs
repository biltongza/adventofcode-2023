use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufRead},
    num::ParseIntError,
    path::Path,
};

#[derive(Debug)]
struct GameInfo {
    game_id: i32,
    sets: Vec<HashMap<CubeColour, i32>>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum CubeColour {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
enum ParseColourError {
    ParseError(ParseIntError),
    UnknownColour,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = read_lines("./src/input.txt")?;
    let parsed_games = lines.map(parse_game);
    let mut sum = 0;
    let max_possible_red = 12;
    let max_possible_green = 13;
    let max_possible_blue = 14;
    for game in parsed_games {
        let game_actually = game?;

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for set in game_actually.sets {
            for pair in set.iter() {
                match pair.0 {
                    CubeColour::Red => max_red = max_red.max(*pair.1),
                    CubeColour::Green => max_green = max_green.max(*pair.1),
                    CubeColour::Blue => max_blue = max_blue.max(*pair.1),
                }
            }
        }

        let possible = max_blue <= max_possible_blue
            && max_green <= max_possible_green
            && max_red <= max_possible_red;
        if possible {
            sum += game_actually.game_id;
        }
        println!("gameid = {}, possible = {possible}, sum = {sum}, red = {max_red}, green = {max_green}, blue = {max_blue}", game_actually.game_id);
    }
    println!("sum = {}", sum);
    return Ok(());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_game(line: Result<String, std::io::Error>) -> Result<GameInfo, Box<dyn Error>> {
    let thing = line?;
    let parts: Vec<&str> = thing.split(':').map(str::trim).collect();
    let game_id = parse_game_id(parts[0])?;
    let game_details = parse_game_infos(parts[1]);
    return Ok(GameInfo {
        game_id,
        sets: game_details,
    });
}

fn parse_game_id(str: &str) -> Result<i32, ParseIntError> {
    let parts: Vec<&str> = str.split(' ').collect();
    return parts[1].parse::<i32>();
}

fn parse_game_infos(str: &str) -> Vec<HashMap<CubeColour, i32>> {
    return str.split(';').map(str::trim).map(parse_game_set).collect();
}

fn parse_game_set(str: &str) -> HashMap<CubeColour, i32> {
    let results: Vec<Result<(CubeColour, i32), ParseColourError>> =
        str.split(',').map(str::trim).map(parse_colour).collect();

    let mut summed: HashMap<CubeColour, i32> = HashMap::new();
    for x in results {
        let a = x.unwrap();
        let key = a.0;
        let val = a.1;
        *summed.entry(key).or_insert(0) += val;
    }
    return summed;
}

fn parse_colour(str: &str) -> Result<(CubeColour, i32), ParseColourError> {
    let parts: Vec<&str> = str.split(' ').map(str::trim).collect();
    let num = parts[0]
        .parse::<i32>()
        .map_err(ParseColourError::ParseError)?;
    let colour = match parts[1] {
        "red" => Ok(CubeColour::Red),
        "green" => Ok(CubeColour::Green),
        "blue" => Ok(CubeColour::Blue),
        _ => Err(ParseColourError::UnknownColour),
    }?;

    return Ok((colour, num));
}
