use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let file_as_string = std::fs::read_to_string("./src/input.txt")?;
    let lines: Vec<Vec<char>> = file_as_string
        .split('\n')
        .map(|x| x.chars().collect())
        .collect();

    let empty_columns = get_empty_column_indices(&lines);
    let empty_rows = get_empty_row_indices(&lines);
    let mut galaxy_locations = get_galaxy_locations(&lines);

    grow_space(&mut galaxy_locations, &empty_rows, &empty_columns);

    // let width = *galaxy_locations.iter().map(|(x, _)|x).max().unwrap()+1;
    // let height = *galaxy_locations.iter().map(|(_, y)|y).max().unwrap()+1;
    
    // let mut new_map = vec![vec!['.'; width]; height];
    // for (gx, gy) in &galaxy_locations {
    //     new_map[*gy][*gx] = '#';
    // }
    // for l in new_map {
    //     println!("{}", String::from_iter(l));
    // }

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

fn get_empty_row_indices(map: &Vec<Vec<char>>) -> Vec<usize> {
    let height = map.len();
    let mut y_indexes_to_add = vec![];
    for y in 0..height {
        let line = &map[y];
        if line.iter().all(|x| *x == '.') {
            y_indexes_to_add.push(y);
        }
    }
    return y_indexes_to_add;
}

fn get_empty_column_indices(map: &Vec<Vec<char>>) -> Vec<usize> {
    let width = map.first().unwrap().len();
    let mut x_indexes_to_add = vec![];
    for x in 0..width {
        if map.iter().map(|v| v[x]).all(|c| c == '.') {
            x_indexes_to_add.push(x);
        }
    }
    return x_indexes_to_add;
}

fn grow_space(galaxy_locations: &mut Vec<(usize, usize)>, empty_rows: &Vec<usize>, empty_columns: &Vec<usize>) {
    let offset = 1000000;

    for x in empty_columns.iter().rev() {
        for i in 0..galaxy_locations.len() {
            let (galaxy_x, galaxy_y) = galaxy_locations[i];
            if galaxy_x >= *x {
                galaxy_locations[i] = (galaxy_x + offset - 1, galaxy_y);
            }
        }
    }
    for y in empty_rows.iter().rev() {
        for i in 0..galaxy_locations.len() {
            let (galaxy_x, galaxy_y) = galaxy_locations[i];
            if galaxy_y >= *y {
                galaxy_locations[i] = (galaxy_x, galaxy_y + offset - 1);
            }
        }
    }
}