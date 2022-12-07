use std::{collections::HashMap, fs};

fn main() {
    let file_name = "src/input.txt";
    let file_data: String = fs::read_to_string(file_name).unwrap();
    let lines: Vec<&str> = file_data.lines().collect();

    let mut lines_iter = lines.into_iter();
    let mut dir_depth_tracker: Vec<&str> = vec!["/"];
    let mut current_line = lines_iter.next();

    let mut dir_size_tracker: HashMap<String, i32> = HashMap::new();
    dir_size_tracker.insert("/".to_string(), 0);
    // skip lines starting with ls or dir
    while current_line != None {
        let line_to_process = current_line.unwrap();
        let line_parts: Vec<&str> = line_to_process.split_whitespace().collect();

        match &line_parts[..] {
            ["$", "cd", dir] => {
                match dir {
                    &"/" => {}
                    &".." => {
                        // if .., pop file from file_depth_tracker
                        dir_depth_tracker.pop();
                    }
                    dir_name => {
                        let mut dir_name_nested = dir_depth_tracker.join(".");
                        dir_name_nested.push_str(".");
                        dir_name_nested.push_str(dir_name);
                        dir_size_tracker.entry(dir_name_nested).or_insert(0);

                        dir_depth_tracker.push(dir_name);
                    }
                }
            }
            ["$", "ls"] => {}
            ["dir", _] => {}
            [file_size, _] => {
                for i in 0..dir_depth_tracker.len() {
                    let dir_to_increase = &dir_depth_tracker[0..=i].join(".");
                    let file_size: i32 = dir_size_tracker.get(dir_to_increase).unwrap()
                        + file_size.parse::<i32>().unwrap();
                    dir_size_tracker.insert(dir_to_increase.to_string(), file_size);
                }
            }
            _ => {
                panic!("Line format unhandled")
            }
        }
        current_line = lines_iter.next();
    }

    let mut total_sizes: i32 = 0;
    for size in dir_size_tracker.values() {
        if size <= &100_000 {
            total_sizes += size;
        }
    }
    // part 1
    println!("total sizes {:?}", total_sizes);

    let root_dir_size = dir_size_tracker.get("/").unwrap();
    let total_size = 70_000_000;
    let required_size = 30_000_000;
    let min_size_to_delete = required_size + root_dir_size - total_size;
    let mut smallest = root_dir_size;

    for size in dir_size_tracker.values() {
        if size > &min_size_to_delete && size < smallest {
            smallest = size;
        }
    }

    // part 2
    println!("{:?}", smallest);
}
