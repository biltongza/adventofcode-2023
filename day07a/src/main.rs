use itertools::Itertools;
use std::{cmp::Ordering, error::Error};

#[derive(Debug, PartialEq, Eq, Clone)]
enum HandType {
    FiveOfAKind(usize),
    FourOfAKind(usize),
    FullHouse(usize),
    ThreeOfAKind(usize),
    TwoPair(usize),
    OnePair(usize),
    HighCard(usize),
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        get_hand_value(self).partial_cmp(&get_hand_value(other))
    }
}
impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        get_hand_value(self).cmp(&get_hand_value(other))
    }
}

fn get_hand_value(hand: &HandType) -> usize {
    match hand {
        HandType::FiveOfAKind(v) => 7000 + v,
        HandType::FourOfAKind(v) => 6000 + v,
        HandType::FullHouse(v) => 5000 + v,
        HandType::ThreeOfAKind(v) => 4000 + v,
        HandType::TwoPair(v) => 3000 + v,
        HandType::OnePair(v) => 2000 + v,
        HandType::HighCard(v) => 1000 + v,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_as_string = std::fs::read_to_string("./src/input.txt")?;
    let lines = file_as_string.lines();

    const CARDS: [char; 14] = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];
    let mut evaluated_hands: Vec<(HandType, usize)> = vec![];
    for line in lines {
        let mut parts = line.split_whitespace();
        let hand = parts.next().unwrap();
        let bid = parts.next().unwrap().parse::<usize>().unwrap();
        let mut groups = Vec::new();
        let mut chars: Vec<char> = hand.chars().collect();
        chars.sort();
        for (key, group) in &chars.iter().group_by(|c| *c) {
            groups.push((
                CARDS.iter().position(|x| *x == *key).unwrap() + 1,
                group.count(),
            ));
        }

        let score = has_five_of_a_kind(&groups)
            .or(has_four_of_a_kind(&groups))
            .or(has_full_house(&groups))
            .or(has_three_of_a_kind(&groups))
            .or(has_two_pair(&groups))
            .or(has_one_pair(&groups))
            .or(has_high_card(&groups)).unwrap();

        evaluated_hands.push((score, bid));
    }
    for l in evaluated_hands {
        println!("{:?}", l);
    }
    // evaluated_hands.sort_by(|(a, _), (b, _)| a.cmp(b));

    // let winnings: usize = evaluated_hands
    //     .iter()
    //     .enumerate()
    //     .map(|(rank, (_, bid))| (rank + 1) * bid)
    //     .sum();
    // println!("{:?}", winnings);
    return Ok(());
}

fn has_five_of_a_kind(groups: &Vec<(usize, usize)>) -> Option<HandType> {
    let (val, _) = groups.iter().filter(|x| x.1 == 5).next()?;
    return Some(HandType::FiveOfAKind(*val));
}

fn has_four_of_a_kind(groups: &Vec<(usize, usize)>) -> Option<HandType> {
    let (val, _) = groups.iter().filter(|x| x.1 == 4).next()?;
    return Some(HandType::FourOfAKind(*val));
}

fn has_full_house(groups: &Vec<(usize, usize)>) -> Option<HandType> {
    let (val, _) = groups.iter().filter(|x| x.1 == 3).next()?;
    if groups.iter().any(|(_, count)| *count == 2) {
        return Some(HandType::FullHouse(*val));
    }
    return None;
}

fn has_three_of_a_kind(groups: &Vec<(usize, usize)>) -> Option<HandType> {
    let (val, _) = groups.iter().filter(|x| x.1 == 3).next()?;
    return Some(HandType::ThreeOfAKind(*val));
}

fn has_two_pair(groups: &Vec<(usize, usize)>) -> Option<HandType> {
    let mut v: Vec<&(usize, usize)> = groups.iter().filter(|x| x.1 == 2).collect();
    if v.len() > 1 {
        v.sort_by(|(a, _), (b, _)| b.cmp(a));
        let (val, _) = v.first().unwrap();
        return Some(HandType::TwoPair(*val));
    }
    return None;
}

fn has_one_pair(groups: &Vec<(usize, usize)>) -> Option<HandType> {
    let (val, _) = groups.iter().filter(|x| x.1 == 2).next()?;
    return Some(HandType::OnePair(*val));
}

fn has_high_card(groups: &Vec<(usize, usize)>) -> Option<HandType> {
    let mut v: Vec<&(usize, usize)> = groups.iter().filter(|x| x.1 == 1).collect();
    if v.len() > 0 {
        v.sort_by(|(a, _), (b, _)| b.cmp(a));
        let (val, _) = v.first().unwrap();
        return Some(HandType::HighCard(*val));
    }
    return None;
}
