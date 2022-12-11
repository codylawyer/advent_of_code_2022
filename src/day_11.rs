#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: String,
    operation_factor: String,
    test: String,
    test_factor: i64,
    test_true: i64,
    test_false: i64,
    inspection_count: i64,
}

pub fn part_1(input: &Vec<String>) -> i64 {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut monkey_idx = 0;

    for line in input {
        let line_split = line.trim().split(" ");
        let line_split_vec = line_split.collect::<Vec<&str>>();

        match line_split_vec[0] {
            "Monkey" => {
                monkey_idx += 1;
                let starting_items: Vec<i64> = Vec::new();
                let operation = "".to_string();
                let operation_factor = "".to_string();
                let test: String = "".to_string();
                let test_factor = 0;
                let test_true = 0;
                let test_false = 0;

                monkeys.push(Monkey {
                    items: starting_items,
                    operation: operation,
                    operation_factor: operation_factor,
                    test: test,
                    test_factor: test_factor,
                    test_true: test_true,
                    test_false: test_false,
                    inspection_count: 0,
                })
            }
            "Starting" => {
                for item in line_split_vec[2..].to_vec() {
                    monkeys[monkey_idx - 1]
                        .items
                        .push(item.trim_matches(&[','] as &[_]).parse::<i64>().unwrap());
                }
            }
            "Operation:" => {
                monkeys[monkey_idx - 1].operation = line_split_vec[4].to_string();
                monkeys[monkey_idx - 1].operation_factor = line_split_vec[5].to_string();
            }
            "Test:" => {
                monkeys[monkey_idx - 1].test = line_split_vec[1].to_string();
                monkeys[monkey_idx - 1].test_factor = line_split_vec[3].parse::<i64>().unwrap();
            }
            "If" => match line_split_vec[1] {
                "true:" => {
                    monkeys[monkey_idx - 1].test_true = line_split_vec[5].parse::<i64>().unwrap();
                }
                "false:" => {
                    monkeys[monkey_idx - 1].test_false = line_split_vec[5].parse::<i64>().unwrap();
                }
                _ => {}
            },
            _ => {}
        }
    }

    for _ in 1..21 {
        for monkey_idx in 0..monkeys.len() {
            let mut throw_item_to = Vec::new();
            for item_idx in 0..monkeys[monkey_idx].items.len() {
                monkeys[monkey_idx].inspection_count += 1;
                let mut operation_factor = 0;
                if monkeys[monkey_idx].operation_factor == "old".to_string() {
                    operation_factor = monkeys[monkey_idx].items[item_idx];
                } else {
                    operation_factor = monkeys[monkey_idx].operation_factor.parse::<i64>().unwrap();
                }
                match monkeys[monkey_idx].operation.as_str() {
                    "+" => {
                        monkeys[monkey_idx].items[item_idx] =
                            monkeys[monkey_idx].items[item_idx] + operation_factor;
                    }
                    "-" => {
                        monkeys[monkey_idx].items[item_idx] =
                            monkeys[monkey_idx].items[item_idx] - operation_factor;
                    }
                    "*" => {
                        monkeys[monkey_idx].items[item_idx] =
                            monkeys[monkey_idx].items[item_idx] * operation_factor;
                    }
                    "/" => {
                        monkeys[monkey_idx].items[item_idx] =
                            monkeys[monkey_idx].items[item_idx] / operation_factor;
                    }
                    _ => {}
                }

                monkeys[monkey_idx].items[item_idx] = monkeys[monkey_idx].items[item_idx] / 3;

                let mut test_result = false;
                match monkeys[monkey_idx].test.as_str() {
                    "divisible" => {
                        test_result = monkeys[monkey_idx].items[item_idx]
                            % monkeys[monkey_idx].test_factor
                            == 0;
                    }
                    _ => {}
                }

                if test_result {
                    throw_item_to.push(monkeys[monkey_idx].test_true);
                } else {
                    throw_item_to.push(monkeys[monkey_idx].test_false);
                }
            }

            let mut num_thrown = 0;
            for throw_idx in 0..throw_item_to.len() {
                let item_to_throw = monkeys[monkey_idx].items.remove(throw_idx - num_thrown);
                num_thrown += 1;
                monkeys[throw_item_to[throw_idx] as usize]
                    .items
                    .push(item_to_throw);
            }
        }
    }
    let mut inspection_counts: Vec<i64> = Vec::new();
    for monkey in &monkeys {
        inspection_counts.push(monkey.inspection_count);
    }
    inspection_counts.sort();
    inspection_counts.reverse();
    inspection_counts[0] * inspection_counts[1]
}

pub fn part_2(input: &Vec<String>) -> i64 {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut monkey_idx = 0;

    for line in input {
        let line_split = line.trim().split(" ");
        let line_split_vec = line_split.collect::<Vec<&str>>();

        match line_split_vec[0] {
            "Monkey" => {
                monkey_idx += 1;
                let starting_items: Vec<i64> = Vec::new();
                let operation = "".to_string();
                let operation_factor = "".to_string();
                let test: String = "".to_string();
                let test_factor = 0;
                let test_true = 0;
                let test_false = 0;

                monkeys.push(Monkey {
                    items: starting_items,
                    operation: operation,
                    operation_factor: operation_factor,
                    test: test,
                    test_factor: test_factor,
                    test_true: test_true,
                    test_false: test_false,
                    inspection_count: 0,
                })
            }
            "Starting" => {
                for item in line_split_vec[2..].to_vec() {
                    monkeys[monkey_idx - 1]
                        .items
                        .push(item.trim_matches(&[','] as &[_]).parse::<i64>().unwrap());
                }
            }
            "Operation:" => {
                monkeys[monkey_idx - 1].operation = line_split_vec[4].to_string();
                monkeys[monkey_idx - 1].operation_factor = line_split_vec[5].to_string();
            }
            "Test:" => {
                monkeys[monkey_idx - 1].test = line_split_vec[1].to_string();
                monkeys[monkey_idx - 1].test_factor = line_split_vec[3].parse::<i64>().unwrap();
            }
            "If" => match line_split_vec[1] {
                "true:" => {
                    monkeys[monkey_idx - 1].test_true = line_split_vec[5].parse::<i64>().unwrap();
                }
                "false:" => {
                    monkeys[monkey_idx - 1].test_false = line_split_vec[5].parse::<i64>().unwrap();
                }
                _ => {}
            },
            _ => {}
        }
    }

    for round in 1..10001 {
        for monkey_idx in 0..monkeys.len() {
            let mut throw_item_to = Vec::new();
            for item_idx in 0..monkeys[monkey_idx].items.len() {
                monkeys[monkey_idx].inspection_count += 1;
                let mut operation_factor = 0;
                if monkeys[monkey_idx].operation_factor == "old".to_string() {
                    operation_factor = monkeys[monkey_idx].items[item_idx];
                } else {
                    operation_factor = monkeys[monkey_idx].operation_factor.parse::<i64>().unwrap();
                }
                match monkeys[monkey_idx].operation.as_str() {
                    "+" => {
                        monkeys[monkey_idx].items[item_idx] =
                            monkeys[monkey_idx].items[item_idx] + operation_factor;
                    }
                    "-" => {
                        monkeys[monkey_idx].items[item_idx] =
                            monkeys[monkey_idx].items[item_idx] - operation_factor;
                    }
                    "*" => {
                        monkeys[monkey_idx].items[item_idx] =
                            monkeys[monkey_idx].items[item_idx] * operation_factor;
                    }
                    "/" => {
                        monkeys[monkey_idx].items[item_idx] =
                            monkeys[monkey_idx].items[item_idx] / operation_factor;
                    }
                    _ => {}
                }

                let mut test_result = false;
                match monkeys[monkey_idx].test.as_str() {
                    "divisible" => {
                        test_result = monkeys[monkey_idx].items[item_idx]
                            % monkeys[monkey_idx].test_factor
                            == 0;
                    }
                    _ => {}
                }

                if test_result {
                    throw_item_to.push(monkeys[monkey_idx].test_true);
                } else {
                    throw_item_to.push(monkeys[monkey_idx].test_false);
                }
            }

            let mut num_thrown = 0;
            for throw_idx in 0..throw_item_to.len() {
                let item_to_throw = monkeys[monkey_idx].items.remove(throw_idx - num_thrown);
                num_thrown += 1;
                monkeys[throw_item_to[throw_idx] as usize]
                    .items
                    .push(item_to_throw);
            }
        }
    }
    let mut inspection_counts: Vec<i64> = Vec::new();
    for monkey in &monkeys {
        inspection_counts.push(monkey.inspection_count);
    }
    inspection_counts.sort();
    inspection_counts.reverse();
    inspection_counts[0] * inspection_counts[1]
}
