use std::{error::Error, num::ParseIntError};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let file_as_string = std::fs::read_to_string("./src/input.txt")?;
    let lines = file_as_string.lines();
    let mut sum = 0;
    for line in lines {
        let vals_result: Result<Vec<i64>, ParseIntError> =
            line.split_whitespace().map(str::parse::<i64>).collect();
        let vals = vals_result.unwrap();

        let t = extrapolate_next_value(&vals);
        sum += t;
    }

    println!("{sum}");
    return Ok(());
}

fn get_difference_vec(vec: &[i64]) -> Vec<i64> {
    let mut vals = vec.iter().peekable();
    let mut r = vec![];
    loop {
        let v = match vals.next() {
            Some(v) => *v,
            None => break,
        };
        let next = match vals.peek() {
            Some(v) => **v,
            None => continue,
        };

        let diff = next - v;
        r.push(diff);
    }

    return r;
}

fn extrapolate_next_value(v: &[i64]) -> i64 {
    let mut history: Vec<Vec<i64>> = vec![];
    history.push(v.to_vec());
    loop {
        let last = history.last().unwrap();
        let diff = get_difference_vec(last.as_slice());

        let count = diff.iter().unique().count();

        history.push(diff);
        if count == 1 {
            break;
        }
    }

    history.reverse();
    let mut next_val = 0;
    let mut history_peekable = history.iter().peekable();
    loop {
        let h = match history_peekable.next() {
            Some(x) => *x.last().unwrap(),
            None => break,
        };
        next_val = next_val + h;
    }
    return next_val;
}
