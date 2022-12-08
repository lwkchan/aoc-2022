use grid::Grid;
use std::fs;

fn main() {
    let file_data = fs::read_to_string("src/input.txt").unwrap();
    let lines: Vec<&str> = file_data.lines().collect();
    let col_count = lines.first().unwrap().len();

    let trees: Vec<usize> = file_data
        .chars()
        .filter(|c| c.is_numeric()) // take away any newline chars
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let tree_grid: Grid<usize> = Grid::from_vec(trees, col_count);

    let mut visible_trees_count: usize = 0;

    // First count all the trees on the edge;
    let (rows, cols) = tree_grid.size();
    visible_trees_count += rows * 2;
    visible_trees_count += (cols - 2) * 2;

    // count the central square
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if is_current_tree_visible((row, col), &tree_grid) {
                visible_trees_count += 1;
            }
        }
    }

    // part 1
    println!("is visible {:?}", visible_trees_count);

    // part 2
    // To measure the viewing distance from a given tree, look up, down, left, and right from that tree;
    // stop if you reach an edge or at the first tree that is the same height or taller than the tree under consideration.
    // (If a tree is right on the edge, at least one of its viewing distances will be zero.)
    // is found by multiplying together its viewing distance

    // count the central square only, since the side ones will be 0 anyway
    let mut scores: Vec<usize> = vec![];
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            let current_tree_height = tree_grid.get(row, col).unwrap();
            let surrounding_trees = get_surrounding_trees((row, col), &tree_grid);

            let surrounding_scores = surrounding_trees.into_iter().map(|trees_in_direction| {
                // getting length here before it's dropped after borrow
                let len = trees_in_direction.len();

                let blocking_tree_index = &trees_in_direction
                    .into_iter()
                    .position(|tree_height| tree_height >= current_tree_height);

                match blocking_tree_index {
                    Some(index) => *index + 1,
                    None => len,
                }
            });

            let score_for_current_tree = surrounding_scores.reduce(|a, b| a * b).unwrap();

            scores.push(score_for_current_tree);
        }
    }

    let highest = scores.into_iter().max().unwrap();

    println!("highest {:?}", highest);
}

fn is_current_tree_visible((row, col): (usize, usize), tree_grid: &Grid<usize>) -> bool {
    let current_tree = tree_grid.get(row, col).unwrap();
    get_surrounding_trees((row, col), tree_grid)
        .iter()
        .any(|trees| trees.iter().all(|tree_height| tree_height < &current_tree))
}

fn get_surrounding_trees((row, col): (usize, usize), tree_grid: &Grid<usize>) -> [Vec<&usize>; 4] {
    let mut above_trees: Vec<&usize> = vec![];
    for measure_row in (0..row).rev() {
        match tree_grid.get(measure_row, col) {
            Some(tree_size) => above_trees.push(tree_size),
            None => panic!("Tried to measure tree outside grid"),
        }
    }

    let mut right_trees: Vec<&usize> = vec![];
    for measure_col in col + 1..tree_grid.cols() {
        match tree_grid.get(row, measure_col) {
            Some(tree_size) => right_trees.push(tree_size),
            None => panic!("Tried to measure tree outside grid"),
        }
    }

    let mut below_trees: Vec<&usize> = vec![];
    for measure_row in row + 1..tree_grid.rows() {
        match tree_grid.get(measure_row, col) {
            Some(tree_size) => below_trees.push(tree_size),
            None => panic!("Tried to measure tree outside grid"),
        }
    }

    let mut left_trees: Vec<&usize> = vec![];
    for measure_col in (0..col).rev() {
        match tree_grid.get(row, measure_col) {
            Some(tree_size) => left_trees.push(tree_size),
            None => panic!("Tried to measure tree outside grid"),
        }
    }

    [above_trees, right_trees, below_trees, left_trees]
}
