use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use regex::Regex;

#[derive(Eq, Debug)]
struct Card {
    num: u32,
    winning_numbers: HashSet<i32>,
    numbers: HashSet<i32>,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.num.partial_cmp(&other.num)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_as_string = std::fs::read_to_string("./src/input.txt")?;
    let lines: Vec<&str> = file_as_string.lines().map(str::trim).collect();
    let re: Regex = Regex::new(r"^.+?(?P<cardnum>\d+): (?P<wins>.+)\|(?P<nums>.+)").unwrap();
    let mut cards: HashMap<u32, Card> = HashMap::new();
    for line in lines {
        let captures = re.captures(line).unwrap();
        let card_num = &captures["cardnum"].parse::<u32>().unwrap();
        let winning_nums = split_nums(&captures["wins"]);
        let nums = split_nums(&captures["nums"]);

        let card = Card {
            num: *card_num,
            winning_numbers: winning_nums,
            numbers: nums,
        };
        cards.insert(*card_num, card);
    }

    let mut card_counter : HashMap<u32, u32> = HashMap::new();
    for (card_num, card) in &cards {
        println!("main: {card_num}");
        get_card_score_with_copies(&card, &cards, &mut card_counter);
    }

    let sum: u32 = card_counter.values().sum();

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

fn get_card_score_with_copies(card: &Card, all_cards: &HashMap<u32, Card>, card_counter: &mut HashMap<u32, u32>) {
    *card_counter.entry(card.num).or_insert(0) += 1;

    let copies = get_card_copies(card);
    if copies.is_none() {
        return;
    }

    for copy_card_num in copies.unwrap() {
        let copy_card = all_cards.get(&copy_card_num).unwrap();
        get_card_score_with_copies(copy_card, all_cards, card_counter);
    }
}

fn get_score(card: &Card) -> usize {
    return card
        .numbers
        .iter()
        .filter(|num| card.winning_numbers.contains(num))
        .count();
}

fn get_card_copies(card: &Card) -> Option<Vec<u32>> {
    let score = get_score(&card);
    if score == 0 {
        return None;
    }

    let mut copies: Vec<u32> = vec![];
    for n in 1..score + 1 {
        let winning_card_num = card.num + n as u32;
        copies.push(winning_card_num);
    }

    return Some(copies);
}
