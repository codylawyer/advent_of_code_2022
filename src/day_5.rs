pub fn part_1(input: &Vec<String>) -> String {
    let num_stacks = (input[0].len() + 1) / 4;

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }

    for line in input {
        if line.contains("move") {
            let line_split = line.split(" ");
            let line_split_vec = line_split.collect::<Vec<&str>>();

            let amount = line_split_vec[1].parse::<i32>().unwrap();
            let origin = line_split_vec[3].parse::<i32>().unwrap();
            let dest = line_split_vec[5].parse::<i32>().unwrap();

            for _ in 0..amount {
                let moving_crate = stacks[(origin - 1) as usize].pop().unwrap();
                stacks[(dest - 1) as usize].push(moving_crate);
            }
        } else {
            if line.contains("[") {
                for idx in 0..num_stacks {
                    let current_char = line.chars().nth(idx * 4 + 1).unwrap();
                    if current_char.is_ascii_alphabetic() {
                        stacks[idx].insert(0, current_char);
                    }
                }
            }
        }
    }

    let mut top_stack = String::new();

    for idx in 0..num_stacks {
        top_stack.push_str(&stacks[idx].pop().unwrap().to_string())
    }

    top_stack
}

pub fn part_2(input: &Vec<String>) -> String {
    let num_stacks = (input[0].len() + 1) / 4;

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }

    for line in input {
        if line.contains("move") {
            let line_split = line.split(" ");
            let line_split_vec = line_split.collect::<Vec<&str>>();

            let amount = line_split_vec[1].parse::<i32>().unwrap();
            let origin = line_split_vec[3].parse::<i32>().unwrap();
            let dest = line_split_vec[5].parse::<i32>().unwrap();

            let mut moved_crates = Vec::new();
            for _ in 0..amount {
                moved_crates.insert(0, stacks[(origin - 1) as usize].pop().unwrap());
            }
            stacks[(dest - 1) as usize].append(&mut moved_crates);
        } else {
            if line.contains("[") {
                for idx in 0..num_stacks {
                    let current_char = line.chars().nth(idx * 4 + 1).unwrap();
                    if current_char.is_ascii_alphabetic() {
                        stacks[idx].insert(0, current_char);
                    }
                }
            }
        }
    }

    let mut top_stack = String::new();

    for idx in 0..num_stacks {
        top_stack.push_str(&stacks[idx].pop().unwrap().to_string())
    }

    top_stack
}
