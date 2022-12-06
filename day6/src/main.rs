use std::{collections::HashSet, fs};

fn main() {
    let file_name = "src/input.txt";
    let file_data: String = fs::read_to_string(file_name).unwrap();

    let chars: Vec<char> = file_data.chars().collect();

    // part 1
    println!("{:?}", process_chars(4, &chars));

    // part 2
    println!("{:?}", process_chars(14, &chars));
}

fn process_chars(set_size: usize, chars: &Vec<char>) -> usize {
    let mut found: bool = false;
    let mut number_of_chars_processed = set_size;
    let mut start_index = 0;
    while !found {
        let char_slice = &chars[start_index..number_of_chars_processed];

        // check if there's any unique

        let mut uniq = HashSet::new();
        let is_all_unique = char_slice.into_iter().all(|x| uniq.insert(x));
        if is_all_unique {
            found = true;
        } else {
            number_of_chars_processed += 1;
            start_index += 1;
        }
    }

    number_of_chars_processed
}
