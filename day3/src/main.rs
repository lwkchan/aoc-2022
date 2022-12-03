use std::fs;

#[derive(Debug, PartialEq)]
struct Bag {
    first_compartment: Vec<char>,
    second_compartment: Vec<char>,
}

const PRIORITY_MAP: &str = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

impl Bag {
    fn new(contents: &str) -> Self {
        let all_contents: Vec<char> = contents.chars().collect();

        let bag_size = all_contents.len();
        let compartment_size: usize = all_contents.len() / 2;
        let first_compartment = &all_contents[0..compartment_size];
        let second_compartment = &all_contents[compartment_size..bag_size];

        Self {
            first_compartment: first_compartment.to_vec(),
            second_compartment: second_compartment.to_vec(),
        }
    }

    fn get_item_of_both_compartments(&self) -> char {
        let mut item: Option<char> = None;

        for item_to_check in &self.first_compartment {
            // check if second compartment contains this char

            if self.second_compartment.contains(item_to_check) {
                item = Some(*item_to_check);
                break;
            }
        }

        item.unwrap()
    }

    fn get_all_contents(&self) -> Vec<char> {
        let all_contents = &mut self.first_compartment.clone();
        all_contents.extend(&self.second_compartment.clone());

        all_contents.to_vec()
    }
}

fn main() {
    let file_name = "src/input.txt";
    let file_data: String = fs::read_to_string(file_name).unwrap();
    let lines: Vec<&str> = file_data.lines().collect();

    solve1(&lines);
    solve2(&lines)
}

fn solve1(lines: &Vec<&str>) {
    let mut priority_sum: i32 = 0;

    for line in lines {
        let bag = Bag::new(line);
        let dup = bag.get_item_of_both_compartments();
        priority_sum += PRIORITY_MAP.find(dup).unwrap() as i32;
    }

    println!("{}", priority_sum);
}

fn solve2(lines: &Vec<&str>) {
    let bags: Vec<Bag> = lines.iter().map(|l| Bag::new(l)).collect();
    let mut bag_iter = bags.iter();
    let mut total = 0;

    loop {
        let bag_1 = bag_iter.next();
        let bag_2 = bag_iter.next();
        let bag_3 = bag_iter.next();

        if bag_3 == None {
            break;
        }

        for item in bag_1.unwrap().get_all_contents() {
            let contains_in_2: bool = bag_2.unwrap().get_all_contents().contains(&item);
            let contains_in_3: bool = bag_3.unwrap().get_all_contents().contains(&item);

            if contains_in_2 && contains_in_3 {
                total += PRIORITY_MAP.find(item).unwrap() as i32;
                break;
            }
        }
    }

    println!("{}", total)
}
