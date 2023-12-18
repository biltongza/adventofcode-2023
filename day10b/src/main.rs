use petgraph::{stable_graph::NodeIndex, Graph};
use std::{error::Error, f64::consts::PI};

fn main() -> Result<(), Box<dyn Error>> {
    let file_as_string = std::fs::read_to_string("./src/input.txt")?;
    let lines: Vec<Vec<char>> = file_as_string
        .split('\n')
        .map(|x| x.chars().collect())
        .collect();

    let height = lines.len();
    let width = lines.first().unwrap().len();
    let mut graph = Graph::new_undirected();
    let mut start_node: NodeIndex = Default::default();
    for y in 0..height {
        for x in 0..width {
            let c = lines[y][x];
            if c == 'S' {
                let up = if y > 0 { Some(lines[y - 1][x]) } else { None };
                let down = if y < height {
                    Some(lines[y + 1][x])
                } else {
                    None
                };
                let left = if x > 0 { Some(lines[y][x - 1]) } else { None };
                let right = if x < width {
                    Some(lines[y][x + 1])
                } else {
                    None
                };
                let start_char = determine_start_char(up, down, left, right);
                start_node = graph.add_node((x, y));

                let mut search_x = x;
                let mut search_y = y;
                let mut search_c = start_char;
                let mut last_offset = None;
                let mut last_node = start_node;
                loop {
                    let offset = determine_direction(search_c, last_offset);
                    let (offset_x, offset_y) = offset;
                    last_offset = Some(offset);

                    let new_x = match offset_x {
                        nx if nx.is_negative() => search_x - nx.wrapping_abs() as u32 as usize,
                        nx => search_x + nx as usize,
                    };
                    let new_y = match offset_y {
                        ny if ny.is_negative() => search_y - ny.wrapping_abs() as u32 as usize,
                        ny => search_y + ny as usize,
                    };

                    if new_x == x && new_y == y {
                        graph.add_edge(last_node, start_node, "");
                        break;
                    }

                    let new_node = graph.add_node((new_x, new_y));
                    graph.add_edge(last_node, new_node, "");
                    search_x = new_x;
                    search_y = new_y;
                    search_c = lines[search_y][search_x];
                    last_node = new_node;
                }

                break;
            }
        }
        if graph.node_count() > 0 {
            break;
        }
    }

    let all_nodes: Vec<(usize, usize)> = graph.raw_nodes().iter().map(|x| x.weight).collect();
    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            let is_path = all_nodes.contains(&(x, y));
            if !is_path && is_inside_polygon(&all_nodes, (x, y)) {
                count += 1;
            }
        }
    }

    println!("{:?}", count);

    return Ok(());
}

fn determine_start_char(
    up: Option<char>,
    down: Option<char>,
    left: Option<char>,
    right: Option<char>,
) -> char {
    match (
        is_upward(up),
        is_downward(down),
        is_left(left),
        is_right(right),
    ) {
        (true, true, _, _) => '|',
        (true, _, true, _) => 'J',
        (true, _, _, true) => 'L',
        (_, _, true, true) => '-',
        (_, true, true, _) => '7',
        (_, true, _, true) => 'F',
        _ => panic!("unable to determine start character"),
    }
}

fn is_upward(c: Option<char>) -> bool {
    matches!(c, Some(x) if x == '|' || x == '7' || x == 'F')
}

fn is_downward(c: Option<char>) -> bool {
    matches!(c, Some(x) if x == '|' || x == 'J' || x == 'L')
}

fn is_left(c: Option<char>) -> bool {
    matches!(c, Some(x) if x == '-' || x == 'L' || x == 'F')
}

fn is_right(c: Option<char>) -> bool {
    matches!(c, Some(x) if x == '-' || x == 'J' || x == '7')
}

fn determine_direction(current: char, last_direction: Option<(i32, i32)>) -> (i32, i32) {
    const LEFT: (i32, i32) = (-1, 0);
    const RIGHT: (i32, i32) = (1, 0);
    const UP: (i32, i32) = (0, -1);
    const DOWN: (i32, i32) = (0, 1);

    match current {
        'F' => {
            if last_direction == Some(LEFT) {
                DOWN
            } else {
                RIGHT
            }
        }
        '-' => {
            if last_direction == Some(LEFT) {
                LEFT
            } else {
                RIGHT
            }
        }
        'L' => {
            if last_direction == Some(LEFT) {
                UP
            } else {
                RIGHT
            }
        }
        '|' => {
            if last_direction == Some(UP) {
                UP
            } else {
                DOWN
            }
        }
        'J' => {
            if last_direction == Some(RIGHT) {
                UP
            } else {
                LEFT
            }
        }
        '7' => {
            if last_direction == Some(RIGHT) {
                DOWN
            } else {
                LEFT
            }
        }
        _ => panic!("don't know where to go!"),
    }
}

// https://www.eecs.umich.edu/courses/eecs380/HANDOUTS/PROJ2/InsidePoly.html
fn is_inside_polygon(graph: &Vec<(usize, usize)>, point: (usize, usize)) -> bool {
    let num_points = graph.len();
    let mut angle = 0.0;
    for i in 0..num_points {
        let current_point = graph[i];
        let next_point = graph[(i + 1) % num_points];
        let x1 = current_point.0 as f64 - point.0 as f64;
        let y1 = current_point.1 as f64 - point.1 as f64;
        let x2 = next_point.0 as f64 - point.0 as f64;
        let y2 = next_point.1 as f64 - point.1 as f64;
        angle += angle_2d(x1, x2, y1, y2);
    }
    
    return !(angle.abs() < PI);
}

fn angle_2d(x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    let theta1 = y1.atan2(x1);
    let theta2 = y2.atan2(x2);
    let mut dtheta = theta2 - theta1;
    while dtheta > PI {
        dtheta -= PI * 2.0;
    }
    while dtheta < -PI {
        dtheta += PI * 2.0;
    }
    return dtheta;
}
