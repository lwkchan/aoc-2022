use std::fs;

use num_integer::div_floor;

#[derive(Debug)]
enum InstructionType {
    Addx(i32),
    Noop,
}

struct CPUProcess {
    current_cycle: i32,
    x: i32,
    signal_strength_counter: i32,
}

impl CPUProcess {
    fn new() -> Self {
        Self {
            current_cycle: 1,
            signal_strength_counter: 0,
            x: 1,
        }
    }

    fn process_instruction(&mut self, instruction: &InstructionType) {
        match instruction {
            InstructionType::Addx(number) => {
                self.increment_cycle();
                self.increment_cycle();
                self.x += number; // edit sprit position
            }
            InstructionType::Noop => {
                // one cycle
                self.increment_cycle();
            }
        }
    }

    fn increment_cycle(&mut self) {
        // process the current cycle first, as we want to check everything during the current cycle
        if CPUProcess::is_target_cycle(&self.current_cycle) {
            self.signal_strength_counter += self.current_cycle * self.x
        }
        self.current_cycle += 1;
    }

    fn is_target_cycle(cycle_number: &i32) -> bool {
        if cycle_number < &20 {
            return false;
        } else {
            (cycle_number - 20) % 40 == 0
        }
    }

    fn get_signal_strength_calc(&self) -> i32 {
        self.signal_strength_counter
    }
}

#[derive(Debug)]
enum Pixel {
    Light,
    Dark,
}

impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        use Pixel::*;

        match (self, other) {
            (Light, Light) => true,
            (Dark, Dark) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct CRTProcess {
    screen: Vec<Pixel>,
    sprite_position: i32,
}

impl CRTProcess {
    fn new() -> Self {
        Self {
            screen: vec![],
            sprite_position: 1,
        }
    }

    fn increment_cycle(&mut self) {
        let current_cycle = self.screen.len();
        let row_index = div_floor(current_cycle, 40);
        let offset = row_index * 40;
        let offset_sprit_position = self.sprite_position + offset as i32;

        let is_light: bool = (offset_sprit_position - 1..=offset_sprit_position + 1)
            .contains(&(current_cycle as i32));

        if is_light {
            self.screen.push(Pixel::Light)
        } else {
            self.screen.push(Pixel::Dark)
        }
    }

    fn process_instruction(&mut self, instruction: &InstructionType) {
        match instruction {
            InstructionType::Addx(number) => {
                self.increment_cycle();
                self.increment_cycle();
                self.sprite_position += number; // edit sprit position
            }
            InstructionType::Noop => {
                // one cycle
                self.increment_cycle();
            }
        }
    }

    fn print_screen(&self) {
        let mut screen_rows: Vec<String> = vec![];
        let mut count = 0;
        for _ in 0..6 {
            let mut current_row: String = String::new();
            for _ in 0..40 {
                if self.screen[count] == Pixel::Light {
                    current_row.push('#')
                } else {
                    current_row.push('.')
                }
                count += 1;
            }

            screen_rows.push(current_row);
        }

        for l in screen_rows {
            println!("{}", l);
        }
    }
}

fn main() {
    let file_data = fs::read_to_string("src/input.txt").unwrap();
    let lines = file_data.lines();

    // parse the instructions
    let instructions: Vec<InstructionType> = lines
        .into_iter()
        .map(|l| {
            let line_parts: Vec<&str> = l.split_whitespace().collect();
            match line_parts[..] {
                ["noop"] => InstructionType::Noop,
                ["addx", num_string] => {
                    let number: i32 = num_string.parse().unwrap();
                    InstructionType::Addx(number)
                }
                _ => panic!("Unable to parse line"),
            }
        })
        .collect();

    // part 1
    let mut process = CPUProcess::new();
    for current_instruction in &instructions {
        process.process_instruction(&current_instruction)
    }
    println!("{}", process.get_signal_strength_calc());

    // part 2
    let mut process = CRTProcess::new();
    for current_instruction in &instructions {
        process.process_instruction(&current_instruction)
    }
    process.print_screen();
}
