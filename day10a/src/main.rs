use petgraph::{algo::dijkstra, stable_graph::NodeIndex, Graph};
use std::error::Error;

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

    // let mut longest_path = 0;
    // for node in graph.node_indices() {
    //     if node == start_node {
    //         continue;
    //     }

    //     let (dist, _) = astar(&graph, node,|n| n == start_node, |_| 1,|_| 0).unwrap();
    //     longest_path = longest_path.max(dist);
    // }

    let d = dijkstra(&graph, start_node, None, |_| 1);
    let longest = d.values().max().unwrap();
    println!("{longest}");

    // println!("{:?}", longest_path);

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
