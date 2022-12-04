use std::fs;

#[derive(Debug)]
struct ElfPair {
    first: [i32; 2],
    second: [i32; 2],
}

impl ElfPair {
    fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.split(',').collect();

        let first: Vec<i32> = parts[0].split('-').map(|n| n.parse().unwrap()).collect();
        let first: [i32; 2] = first.try_into().unwrap();

        let second: Vec<i32> = parts[1].split('-').map(|n| n.parse().unwrap()).collect();
        let second: [i32; 2] = second.try_into().unwrap();

        Self { first, second }
    }

    fn some_range_is_covered(&self) -> bool {
        !(self.first[0] > self.second[1] || self.second[0] > self.first[1])
    }

    fn full_range_is_covered(&self) -> bool {
        let first_range = &self.first[1] - &self.first[0];
        let second_range = &self.second[1] - &self.second[0];

        let mut smaller_range: &[i32; 2] = &self.second;
        let mut larger_range: &[i32; 2] = &self.first;

        // check if a range is smaller or equal to the other
        if first_range <= second_range {
            smaller_range = &self.first;
            larger_range = &self.second;
        }

        let is_lower_in_range = smaller_range[0] >= larger_range[0];
        let is_larger_in_range = smaller_range[1] <= larger_range[1];

        is_lower_in_range && is_larger_in_range
    }
}

fn main() {
    let file_name = "src/input.txt";
    let file_data: String = fs::read_to_string(file_name).unwrap();
    let lines: Vec<&str> = file_data.lines().collect();

    let mut num_with_fully_covered_range: i32 = 0;
    for l in &lines {
        let elf_pair = ElfPair::new(l);
        if elf_pair.full_range_is_covered() {
            num_with_fully_covered_range += 1;
        }
    }
    println!(
        "num in fully covered range {}",
        num_with_fully_covered_range
    );

    let mut num_with_partial_coverage = 0;
    for l in &lines {
        let elf_pair = ElfPair::new(l);
        if elf_pair.some_range_is_covered() {
            num_with_partial_coverage += 1;
        }
    }

    println!("num with partial coverage {}", num_with_partial_coverage)
}
