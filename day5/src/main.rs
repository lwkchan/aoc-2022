use std::fs;

type CratePile = Vec<char>;

#[derive(Debug)]
struct Command {
    source: usize,
    target: usize,
    number_of_crates: usize,
}

impl Command {
    fn new(line: &str) -> Self {
        // move 1 from 2 to 1
        let words: Vec<&str> = line.split_whitespace().collect();

        let number_of_crates = words.get(1).unwrap().parse().unwrap();
        let source = words.get(3).unwrap().parse().unwrap();
        let target = words.get(5).unwrap().parse().unwrap();

        Self {
            source,
            target,
            number_of_crates,
        }
    }
}
fn main() {
    let file_data = fs::read_to_string("src/input.txt").unwrap();
    let lines: Vec<&str> = file_data.lines().collect();

    // parse stack
    let mut lines_iter = lines.into_iter();

    let mut stacks: Vec<CratePile> = vec![Vec::new(); 100];
    stacks.push(Vec::new()); // Push a zeroth stack so you don't have to care later

    // let mut current_stack: CratePile = Vec::new();
    loop {
        let current_line = lines_iter.next().unwrap();
        let current_line = current_line.replace("[", " ");
        let current_line = current_line.replace("]", " ");

        if current_line.chars().nth(1).unwrap().is_numeric() {
            // This line represents the labels, so just move on
            break;
        }

        let mut current_stack: usize = 1;
        let mut chars = current_line.chars().into_iter();

        // Process the current line's chars
        loop {
            match (chars.next(), chars.next(), chars.next(), chars.next()) {
                (Some(_), Some(item), Some(_), _) => {
                    if item != ' ' {
                        stacks[current_stack].push(item);
                    }
                    current_stack += 1;
                }
                (None, _, _, _) => {
                    // panic!();
                    break;
                } // The line has ended, move to next line
                (_, _, _, _) => panic!(),
            }
        }
    }

    let stacks: Vec<CratePile> = stacks.into_iter().filter(|v| v.len() > 0).collect();
    let stacks: Vec<CratePile> = stacks
        .into_iter()
        .map(|mut v| {
            // reverse each pile so that it has the correct one on the top
            v.reverse();
            v
        })
        .collect();

    // skip the blank line
    if lines_iter.next().unwrap() != "" {
        panic!()
    };

    // process commands
    let mut commands: Vec<Command> = Vec::new();
    let mut current_line = lines_iter.next();
    while current_line != None {
        commands.push(Command::new(current_line.unwrap()));
        current_line = lines_iter.next();
    }

    let tops = process_with_cratemover_9000(&commands, &stacks);
    println!("tops part 1: {:?}", tops);

    let tops = process_with_cratemover_9001(&commands, &stacks);
    println!("tops part 2: {:?}", tops);
}

fn process_with_cratemover_9000(commands: &Vec<Command>, stacks: &Vec<CratePile>) -> String {
    let mut stacks = stacks.clone(); // clone in here so the above stack isn't mutated before part 2
    for command in commands {
        let target = command.target;
        let source = command.source;
        let number_of_crates = command.number_of_crates;

        let mut temp_stack: Vec<char> = Vec::new();
        for _ in 0..number_of_crates {
            temp_stack.push(stacks[source - 1].pop().unwrap());
        }

        stacks[target - 1].append(&mut temp_stack);
    }

    // get the top of each stack

    let mut tops = Vec::new();
    for stack in &stacks {
        let top = stack.last().unwrap();
        tops.push(top)
    }

    let tops: String = tops.into_iter().collect();
    tops
}

fn process_with_cratemover_9001(commands: &Vec<Command>, stacks: &Vec<CratePile>) -> String {
    let mut stacks = stacks.clone(); // clone in here so the above stack isn't mutated before part 2
    for command in commands {
        let target = command.target;
        let source = command.source;
        let number_of_crates = command.number_of_crates;

        let mut temp_stack: Vec<char> = Vec::new();
        for _ in 0..number_of_crates {
            temp_stack.push(stacks[source - 1].pop().unwrap());
        }

        temp_stack.reverse();
        stacks[target - 1].append(&mut temp_stack);
    }

    // get the top of each stack

    let mut tops = Vec::new();
    for stack in &stacks {
        let top = stack.last().unwrap();
        tops.push(top)
    }

    let tops: String = tops.into_iter().collect();
    tops
}
