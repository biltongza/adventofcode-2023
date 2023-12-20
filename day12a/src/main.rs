use std::error::Error;

// this is a (partial) port of https://github.com/encse/adventofcode/blob/6058884a70ecaa57e9029a809dec04d31883f962/2023/Day12/Solution.cs
// I don't feel right using someone else's code for stars so I will skip day 12
// but it was cool understanding the approach to the problem


fn main() -> Result<(), Box<dyn Error>> {
    let file_as_string = std::fs::read_to_string("./src/input.txt")?;
    let mut sum = 0;    
    for line in file_as_string.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let pattern: Vec<char> = parts[0].chars().collect();
        let groups_str = parts[1];
        let groups_maybe: Result<Vec<usize>, std::num::ParseIntError> =
            groups_str.split(',').map(str::parse::<usize>).collect();
        let groups = groups_maybe.unwrap();

        sum += get_num_permutations(pattern.as_slice(), groups.as_slice());
    }
    println!("{}", sum);

    return Ok(());
}

fn get_num_permutations(pattern: &[char], groups: &[usize]) -> u64 {
    let current_char = pattern.first();
    match current_char {
        Some('.') => process_dot(pattern, groups),
        Some('#') => process_hash(pattern, groups),
        Some('?') => process_question(pattern, groups),
        _ => process_end(groups),
    }
}

fn process_hash(pattern: &[char], groups: &[usize]) -> u64 {
    if groups.len() == 0 {
        return 0;
    }

    let group_size = *groups.first().unwrap();
    let new_groups = &groups[1..];

    let num_dead_springs = pattern
        .iter()
        .take_while(|x| **x == '#' || **x == '?')
        .count();

    if num_dead_springs < group_size {
        return 0;
    } else if pattern.len() == group_size {
        return get_num_permutations(&[], new_groups);
    } else if matches!(pattern.get(group_size), Some(x) if *x == '#') {
        return 0;
    } else {
        let new_pattern = &pattern[(group_size + 1)..];
        return get_num_permutations(new_pattern, new_groups);
    }
}

fn process_dot(pattern: &[char], groups: &[usize]) -> u64 {
    get_num_permutations(&pattern[1..], groups)
}

fn process_question(pattern: &[char], groups: &[usize]) -> u64 {
    let first_removed = &pattern[1..];
    let with_hash = [&['#'], first_removed].concat();
    let hash_pattern = with_hash.as_slice();
    let with_dot = [&['.'], first_removed].concat();
    let dot_pattern = with_dot.as_slice();
    return get_num_permutations(hash_pattern, groups) + get_num_permutations(dot_pattern, groups);
}

fn process_end(groups: &[usize]) -> u64 {
    return if groups.len() > 0 { 0 } else { 1 };
}
