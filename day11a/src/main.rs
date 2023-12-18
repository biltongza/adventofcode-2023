use std::{collections::HashMap, error::Error};



fn main() -> Result<(), Box<dyn Error>> {
    let file_as_string = std::fs::read_to_string("./src/input.txt")?;
    let mut lines: Vec<Vec<char>> = file_as_string
        .split('\n')
        .map(|x| x.chars().collect())
        .collect();

    grow_space(&mut lines);

    let galaxy_locations = get_galaxy_locations(&lines);
    let pairs = determine_galaxy_pairs(&galaxy_locations);
    let mut sum = 0;
    for (galaxy, others) in pairs {
        for other in others {
            sum += manhattan_distance(galaxy, other);
        }
    }

    println!("{}", sum);

    return Ok(());
}



fn manhattan_distance(p1: (usize, usize), p2: (usize, usize)) -> usize {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let diff_x = x1.abs_diff(x2);
    let diff_y = y1.abs_diff(y2);
    return diff_x + diff_y;
}

fn get_galaxy_locations(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut locations = vec![];
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                locations.push((x, y));
            }
        }
    }

    return locations;
}

fn determine_galaxy_pairs(
    locations: &Vec<(usize, usize)>,
) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    let mut pairs = HashMap::new();
    for galaxy in locations {
        let mut other_galaxies = vec![];
        for other_galaxy in locations {
            if galaxy == other_galaxy || pairs.contains_key(other_galaxy) {
                continue;
            }

            other_galaxies.push(*other_galaxy);
        }
        pairs.insert(*galaxy, other_galaxies);
    }
    return pairs;
}

fn grow_space(map: &mut Vec<Vec<char>>) {
    let mut height = map.len();
    let mut width = map.first().unwrap().len();

    // scan rows for empty
    let mut y_indexes_to_add = vec![];
    for y in 0..height {
        let line = &map[y];
        if line.iter().all(|x| *x == '.') {
            y_indexes_to_add.push(y);
        }
    }

    // add new rows
    let mut y_offset = 0;
    for y in y_indexes_to_add {
        let new_y = y + y_offset;
        map.insert(new_y, vec!['.'; width]);
        y_offset += 1;
        height += 1;
    }

    // scan columns
    let mut x_indexes_to_add = vec![];
    for x in 0..width {
        if map.iter().map(|v| v[x]).all(|c| c == '.') {
            x_indexes_to_add.push(x);
        }
    }

    // add new columns
    let mut x_offset = 0;
    for x in x_indexes_to_add {
        let new_x = x + x_offset;
        for y in &mut *map {
            y.insert(new_x, '.');
        }
        x_offset += 1;
        width += 1;
    }
}
