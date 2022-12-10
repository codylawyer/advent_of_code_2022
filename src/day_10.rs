use std::collections::HashMap;

fn print_crt(crt: &HashMap<(i32, i32), bool>) {
    let mut crt_output = String::new();
    for y in 0..6 {
        for x in 0..40 {
            match crt.get(&(x, y)).unwrap() {
                true => crt_output.push_str("#"),
                false => crt_output.push_str("."),
                _ => {}
            }
        }
        crt_output.push_str("\n");
    }
    print!("{}", crt_output);
}

pub fn part_1(input: &Vec<String>) -> i32 {
    let mut cycle = 0;
    let mut x = 1;

    let mut history = HashMap::new();
    history.insert(cycle, x);

    for instruction in input {
        let instruction_split = instruction.split(" ");
        let instruction_split_vec = instruction_split.collect::<Vec<&str>>();

        match instruction_split_vec[0] {
            "noop" => {
                cycle += 1;
                history.insert(cycle, x);
            }
            "addx" => {
                cycle += 1;
                history.insert(cycle, x);
                cycle += 1;
                history.insert(cycle, x);
                x += instruction_split_vec[1].parse::<i32>().unwrap();
            }
            _ => {}
        }
    }
    (20 * history.get(&20).unwrap())
        + (60 * history.get(&60).unwrap())
        + (100 * history.get(&100).unwrap())
        + (140 * history.get(&140).unwrap())
        + (180 * history.get(&180).unwrap())
        + (220 * history.get(&220).unwrap())
}

pub fn part_2(input: &Vec<String>) -> i32 {
    let mut x = 1;

    let mut crt_x = 0;
    let mut crt_y: i32 = 0;

    let mut crt: HashMap<(i32, i32), bool> = HashMap::new();
    for y in 0..6 {
        for x in 0..40 {
            crt.insert((x, y), false);
        }
    }

    for instruction in input {
        let instruction_split = instruction.split(" ");
        let instruction_split_vec = instruction_split.collect::<Vec<&str>>();

        match instruction_split_vec[0] {
            "noop" => {
                if crt_x >= x - 1 && crt_x <= x + 1 {
                    crt.insert((crt_x, crt_y), true);
                } else {
                    crt.insert((crt_x, crt_y), false);
                }
                crt_x += 1;
                if crt_x > 39 {
                    crt_x = 0;
                    crt_y += 1;
                    if crt_y > 5 {
                        crt_y = 0;
                    }
                }
                //
            }
            "addx" => {
                if crt_x >= x - 1 && crt_x <= x + 1 {
                    crt.insert((crt_x, crt_y), true);
                } else {
                    crt.insert((crt_x, crt_y), false);
                }
                crt_x += 1;
                if crt_x > 39 {
                    crt_x = 0;
                    crt_y += 1;
                    if crt_y > 5 {
                        crt_y = 0;
                    }
                }
                //
                if crt_x >= x - 1 && crt_x <= x + 1 {
                    crt.insert((crt_x, crt_y), true);
                } else {
                    crt.insert((crt_x, crt_y), false);
                }
                crt_x += 1;
                if crt_x > 39 {
                    crt_x = 0;
                    crt_y += 1;
                    if crt_y > 5 {
                        crt_y = 0;
                    }
                }
                //
                x += instruction_split_vec[1].parse::<i32>().unwrap();
            }
            _ => {}
        }
    }
    print_crt(&crt);
    0
}
