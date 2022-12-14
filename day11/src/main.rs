use std::{collections::VecDeque, fs, ops::Mul};

#[derive(Debug)]

enum Operation {
    Addition(i64),
    Multiplication(i64),
    MultiplySelf,
    AdditionSelf,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    test_divisible_by: i64,
    operation: Operation,
    inspection_count: usize,
    pass_to: (usize, usize), // (true, false)
}

impl Monkey {
    fn new(
        items: Vec<i64>,
        operation: Operation,
        test_divisible_by: i64,
        pass_to: (usize, usize),
    ) -> Self {
        Self {
            items: VecDeque::from(items),
            inspection_count: 0,
            operation,
            test_divisible_by,
            pass_to,
        }
    }

    fn get_items(&mut self) -> VecDeque<i64> {
        let return_items = self.items.to_owned();
        self.items.clear();

        return_items
    }

    fn inspect_item(
        &mut self,
        current_item: i64,
        with_relief_reduction: bool,
        common_multiple: &i64,
    ) -> (usize, i64) {
        self.inspection_count += 1;

        let mut current_item: i64 = match &self.operation {
            Operation::AdditionSelf => current_item.mul(2),
            Operation::MultiplySelf => current_item.pow(2),
            Operation::Addition(n) => current_item + n,
            Operation::Multiplication(n) => current_item * n,
        };

        if with_relief_reduction {
            current_item /= 3
        }
        current_item %= common_multiple;

        let (when_true, when_false) = self.pass_to;
        // Current worry level is not divisible by 23.
        if &current_item % &self.test_divisible_by == 0 {
            (when_true, current_item)
        } else {
            (when_false, current_item)
        }
    }
}

fn make_monkeys(lines: &Vec<&str>) -> Vec<Monkey> {
    let mut line_chunks = lines.chunks(7);
    let mut current_chunk = line_chunks.next();
    let mut monkeys: Vec<Monkey> = vec![];

    while current_chunk != None {
        let monkey_info = current_chunk.unwrap();
        let items = monkey_info[1]
            .split_whitespace()
            .fold(vec![], |mut prev: Vec<i64>, part| {
                if part.chars().any(|c| c.is_numeric()) {
                    let num_string: String = part.chars().filter(|n| n.is_numeric()).collect();
                    prev.push(num_string.parse().unwrap());
                    prev
                } else {
                    prev
                }
            });

        let operation_parts: Vec<&str> = monkey_info[2].split_whitespace().collect();
        let operation: Operation = match operation_parts[..] {
            [_, "new", "=", "old", "+", "old"] => Operation::AdditionSelf,
            [_, "new", "=", "old", "*", "old"] => Operation::MultiplySelf,
            [_, "new", "=", "old", "*", num_string] => {
                Operation::Multiplication(num_string.parse().unwrap())
            }
            [_, "new", "=", "old", "+", num_string] => {
                Operation::Addition(num_string.parse().unwrap())
            }
            _ => panic!("Unhandled operation"),
        };
        let test_divisble_by: i64 = monkey_info[3]
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let pass_to = (
            monkey_info[4]
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap(),
            monkey_info[5]
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap(),
        );

        monkeys.push(Monkey::new(items, operation, test_divisble_by, pass_to));

        current_chunk = line_chunks.next();
    }

    monkeys
}
fn main() {
    let data = fs::read_to_string("src/input.txt").unwrap();
    let lines: Vec<&str> = data.lines().collect();

    // pt 1
    let mut monkeys = make_monkeys(&lines);
    let common_multiple = &monkeys.iter().fold(1, |acc, m| acc * m.test_divisible_by);

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let mut current_items = monkeys[i].get_items();
            let mut items_to_pass: Vec<(usize, i64)> = vec![];
            while current_items.len() > 0 {
                let item_to_inspect = current_items.pop_front().unwrap();
                items_to_pass.push(monkeys[i].inspect_item(
                    item_to_inspect,
                    false,
                    common_multiple,
                ));
            }

            for (next_monkey_index, item) in items_to_pass {
                monkeys[next_monkey_index].items.push_back(item);
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspection_count.partial_cmp(&a.inspection_count).unwrap());
    let monkey_business = &monkeys[0..=1]
        .into_iter()
        .map(|m| m.inspection_count)
        .reduce(|a, b| a * b)
        .unwrap();

    println!("relief monkey business {}", monkey_business);

    // pt 2
    let mut monkeys = make_monkeys(&lines);
    let common_multiple = &monkeys.iter().fold(1, |acc, m| acc * m.test_divisible_by);
    for _round in 0..10_000 {
        for i in 0..monkeys.len() {
            let mut current_items = monkeys[i].get_items();
            let mut items_to_pass: Vec<(usize, i64)> = vec![];
            while current_items.len() > 0 {
                let item_to_inspect = current_items.pop_front().unwrap();
                items_to_pass.push(monkeys[i].inspect_item(
                    item_to_inspect,
                    false,
                    common_multiple,
                ));
            }

            for (next_monkey_index, item) in items_to_pass {
                monkeys[next_monkey_index].items.push_back(item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspection_count.partial_cmp(&a.inspection_count).unwrap());
    let monkey_business = &monkeys[0..=1]
        .into_iter()
        .map(|m| m.inspection_count)
        .reduce(|a, b| a * b)
        .unwrap();

    println!("no relief monkey business {}", monkey_business);
}
