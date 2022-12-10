use std::{collections::HashSet, fs};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    step_count: usize,
}

impl Instruction {
    fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let direction: Direction = match parts.get(0) {
            Some(&"R") => Direction::Right,
            Some(&"L") => Direction::Left,
            Some(&"U") => Direction::Up,
            Some(&"D") => Direction::Down,
            _ => panic!("unhandled direction"),
        };

        let step_count = parts.get(1).unwrap().parse().unwrap();

        Self {
            direction,
            step_count,
        }
    }
}

type Coordinates = (usize, usize);

fn main() {
    let file_data = fs::read_to_string("src/input.txt").unwrap();
    let lines = file_data.lines();
    let instructions: Vec<Instruction> = lines.map(|l| Instruction::new(l)).collect();
    let starting_coords = (800, 800);

    let mut tail_visited_coords: HashSet<Coordinates> = HashSet::from([starting_coords]);
    let mut current_knot_coords: Vec<(usize, usize)> = vec![starting_coords; 2];

    // process directions to create the head's next visited coord;
    for instruction in &instructions {
        for _ in 0..instruction.step_count {
            let next_knot_coords = process_step(&instruction.direction, &current_knot_coords);
            tail_visited_coords.insert(*next_knot_coords.last().unwrap());
            current_knot_coords = next_knot_coords;
        }
    }

    // part 1
    println!("tail_visited_coords {:?}", tail_visited_coords.len());

    // part 2
    let mut long_tail_visited_coords: HashSet<Coordinates> = HashSet::from([starting_coords]);
    let mut current_knot_coords: Vec<(usize, usize)> = vec![starting_coords; 10];

    // process directions to create the head's next visited coord;
    for instruction in &instructions {
        for _ in 0..instruction.step_count {
            let next_knot_coords = process_step(&instruction.direction, &current_knot_coords);
            long_tail_visited_coords.insert(*next_knot_coords.last().unwrap());
            current_knot_coords = next_knot_coords;
        }
    }

    println!("{:?}", long_tail_visited_coords.len());
}

fn process_step(direction: &Direction, knot_coords: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut next_knot_coords: Vec<(usize, usize)> = vec![];

    let (head_x, head_y) = knot_coords.first().unwrap();

    let next_head_coord: (usize, usize) = match direction {
        Direction::Right => (head_x + 1, *head_y),
        Direction::Left => (head_x - 1, *head_y),
        Direction::Up => (*head_x, head_y + 1),
        Direction::Down => (*head_x, head_y - 1),
    };

    next_knot_coords.push(next_head_coord);

    for i in 1..knot_coords.len() {
        let (current_knot_x, current_knot_y) = knot_coords[i];
        let (prev_knot_x, prev_knot_y) = next_knot_coords[i - 1];

        next_knot_coords.push(knot_coords[i]);

        if prev_knot_x == current_knot_x && current_knot_y.abs_diff(prev_knot_y) == 2 {
            // the y is different, y will change
            // y is below
            if prev_knot_y < current_knot_y {
                next_knot_coords[i] = (current_knot_x, current_knot_y - 1);
            }

            // y is above
            if prev_knot_y > current_knot_y {
                next_knot_coords[i] = (current_knot_x, current_knot_y + 1);
            }
        } else if prev_knot_y == current_knot_y && current_knot_x.abs_diff(prev_knot_x) == 2 {
            // changing x
            // x is lower
            if prev_knot_x < current_knot_x {
                next_knot_coords[i] = (current_knot_x - 1, current_knot_y);
            }

            // x is higher
            if prev_knot_x > current_knot_x {
                next_knot_coords[i] = (current_knot_x + 1, current_knot_y);
            }

            // Otherwise, if the head and tail aren't touching and aren't
            // in the same row or column, the tail always moves one step
            // diagonally to keep up
        } else if current_knot_y.abs_diff(prev_knot_y) > 1
            && current_knot_x.abs_diff(prev_knot_x) > 1
        {
            // cover pure diagonal
            next_knot_coords[i] = knot_coords[i - 1]
        } else if current_knot_y.abs_diff(prev_knot_y) == 2 {
            let next_x = prev_knot_x;
            if prev_knot_y > current_knot_y {
                let next_y = prev_knot_y - 1;
                next_knot_coords[i] = (next_x, next_y);
            } else {
                let next_y = prev_knot_y + 1;
                next_knot_coords[i] = (next_x, next_y);
            }
        } else if current_knot_x.abs_diff(prev_knot_x) == 2 {
            if prev_knot_x > current_knot_x {
                let next_x = prev_knot_x - 1;
                next_knot_coords[i] = (next_x, prev_knot_y);
            } else {
                let next_x = prev_knot_x + 1;
                next_knot_coords[i] = (next_x, prev_knot_y);
            }
        }
    }

    next_knot_coords
}
