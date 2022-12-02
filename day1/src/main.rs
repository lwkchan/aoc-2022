use std::fs;

fn main() {
    let elves = get_elves();
    solve_1(&elves);
    solve_2(&elves);
}

fn get_elves() -> Vec<i32> {
    let file_name = "src/input.txt";
    let file_data: String = fs::read_to_string(file_name).unwrap();
    let lines: Vec<&str> = file_data.lines().collect();

    let mut elves: Vec<i32> = Vec::new();
    let mut current_elf: i32 = 0;

    let line_count = &lines.len();

    for (n, line) in lines.into_iter().enumerate() {
        if line.len() > 0 {
            let num: i32 = line.parse().unwrap();
            current_elf += num;
        }

        if line.len() == 0 || (n + 1) == *line_count {
            elves.push(current_elf);
            current_elf = 0;
        }
    }

    elves
}

fn solve_1(elves: &Vec<i32>) {
    let biggest_elf = elves.iter().max();
    match biggest_elf {
        Some(total_cals) => println!("Total cals: {}", total_cals),
        _ => panic!(),
    }
}
fn solve_2(elves: &Vec<i32>) {
    let mut sorted_elves = elves.clone();
    sorted_elves.sort();

    let largest_elves = &sorted_elves[(elves.len() - 3)..(elves.len())];
    let total: i32 = largest_elves.iter().sum();

    print!("Total cals of largest 3: {}", total)
}
