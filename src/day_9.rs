use std::collections::HashMap;

fn print_knots(knots: &Vec<(i32, i32)>, min_x: i32, max_x: i32, min_y: i32, max_y: i32) {
    let mut output = String::new();
    for y in (min_y..max_y).rev() {
        for x in min_x..max_x {
            let mut knot_found = false;
            for (idx, knot) in knots.into_iter().enumerate() {
                if knot.0 == x && knot.1 == y {
                    knot_found = true;
                    output.push_str(&idx.to_string());
                    break;
                }
            }
            if !knot_found {
                output.push_str(".");
            }
        }
        output.push_str("\n");
    }

    print!("{}", output);
    println!(" ");
}

pub fn part_1(input: &Vec<String>) -> usize {
    let mut head_position = (0, 0);
    let mut tail_position = (0, 0);

    let mut tail_positions = HashMap::new();
    tail_positions.insert(tail_position, 1);

    for instruction in input {
        let instruction_split = instruction.split(" ");
        let instruction_split_vec = instruction_split.collect::<Vec<&str>>();

        let direction = instruction_split_vec[0];
        let amount = instruction_split_vec[1].parse::<i32>().unwrap();

        for _ in 0..amount {
            // move head
            match direction {
                "U" => head_position.1 += 1,
                "D" => head_position.1 += -1,
                "L" => head_position.0 += -1,
                "R" => head_position.0 += 1,
                _ => {}
            }

            // move tail
            let delta_x = head_position.0 - tail_position.0;
            let delta_y = head_position.1 - tail_position.1;

            // UDLR moves
            // head 2 steps to right
            if delta_x == 2 && delta_y == 0 {
                tail_position.0 += 1; // move 1 right
            }
            // head 2 steps to left
            if delta_x == -2 && delta_y == 0 {
                tail_position.0 += -1; // move 1 left
            }
            // head 2 steps above
            if delta_x == 0 && delta_y == 2 {
                tail_position.1 += 1; // move 1 up
            }
            // head 2 steps below
            if delta_x == 0 && delta_y == -2 {
                tail_position.1 += -1; // move 1 down
            }

            // Diagonal moves
            // head 2 up and 1 right
            if delta_x == 1 && delta_y == 2 {
                tail_position.1 += 1; // move up
                tail_position.0 += 1; // move right
            }
            // head 1 up and 2 right
            if delta_x == 2 && delta_y == 1 {
                tail_position.1 += 1; // move up
                tail_position.0 += 1; // move right
            }

            // head 2 up and 1 left
            if delta_x == -1 && delta_y == 2 {
                tail_position.1 += 1; // move up
                tail_position.0 += -1; // move left
            }
            // head 1 up and 2 left
            if delta_x == -2 && delta_y == 1 {
                tail_position.1 += 1; // move up
                tail_position.0 += -1; // move left
            }

            // head 2 down and 1 left
            if delta_x == -1 && delta_y == -2 {
                tail_position.1 += -1; // move down
                tail_position.0 += -1; // move left
            }
            // head 1 down and 2 left
            if delta_x == -2 && delta_y == -1 {
                tail_position.1 += -1; // move down
                tail_position.0 += -1; // move left
            }

            // head 2 down and 1 right
            if delta_x == 1 && delta_y == -2 {
                tail_position.1 += -1; // move down
                tail_position.0 += 1; // move right
            }
            // head 1 down and 2 right
            if delta_x == 2 && delta_y == -1 {
                tail_position.1 += -1; // move down
                tail_position.0 += 1; // move right
            }

            if !tail_positions.contains_key(&tail_position) {
                tail_positions.insert(tail_position, 1);
            }
        }
    }
    tail_positions.len()
}

pub fn part_2(input: &Vec<String>) -> usize {
    let mut knots: Vec<(i32, i32)> = [(0, 0); 10].to_vec();

    let min_x = -16;
    let max_x = 16;
    let min_y = -16;
    let max_y = 16;

    let mut tail_positions = HashMap::new();
    tail_positions.insert(knots[9], 1);

    for instruction in input {
        let instruction_split = instruction.split(" ");
        let instruction_split_vec = instruction_split.collect::<Vec<&str>>();

        let direction = instruction_split_vec[0];
        let amount = instruction_split_vec[1].parse::<i32>().unwrap();

        for _ in 0..amount {
            // move head
            match direction {
                "U" => knots[0].1 += 1,
                "D" => knots[0].1 += -1,
                "L" => knots[0].0 += -1,
                "R" => knots[0].0 += 1,
                _ => {}
            }

            // move knots
            for knot_idx in 1..knots.len() {
                let delta_x = knots[knot_idx - 1].0 - knots[knot_idx].0;
                let delta_y = knots[knot_idx - 1].1 - knots[knot_idx].1;

                // UDLR moves
                // head 2 steps to right
                if delta_x == 2 && delta_y == 0 {
                    knots[knot_idx].0 += 1; // move 1 right
                    continue;
                }
                // head 2 steps to left
                if delta_x == -2 && delta_y == 0 {
                    knots[knot_idx].0 += -1; // move 1 left
                    continue;
                }
                // head 2 steps above
                if delta_x == 0 && delta_y == 2 {
                    knots[knot_idx].1 += 1; // move 1 up
                    continue;
                }
                // head 2 steps below
                if delta_x == 0 && delta_y == -2 {
                    knots[knot_idx].1 += -1; // move 1 down
                    continue;
                }

                // Diagonal moves
                // head 2 up and 1 right
                if delta_x >= 1 && delta_y == 2 {
                    knots[knot_idx].1 += 1; // move up
                    knots[knot_idx].0 += 1; // move right
                    continue;
                }
                // head 1 up and 2 right
                if delta_x == 2 && delta_y >= 1 {
                    knots[knot_idx].1 += 1; // move up
                    knots[knot_idx].0 += 1; // move right
                }

                // head 2 up and 1 left
                if delta_x <= -1 && delta_y == 2 {
                    knots[knot_idx].1 += 1; // move up
                    knots[knot_idx].0 += -1; // move left
                    continue;
                }
                // head 1 up and 2 left
                if delta_x == -2 && delta_y >= 1 {
                    knots[knot_idx].1 += 1; // move up
                    knots[knot_idx].0 += -1; // move left
                    continue;
                }

                // head 2 down and 1 left
                if delta_x <= -1 && delta_y == -2 {
                    knots[knot_idx].1 += -1; // move down
                    knots[knot_idx].0 += -1; // move left
                    continue;
                }
                // head 1 down and 2 left
                if delta_x == -2 && delta_y <= -1 {
                    knots[knot_idx].1 += -1; // move down
                    knots[knot_idx].0 += -1; // move left
                    continue;
                }

                // head 2 down and 1 right
                if delta_x >= 1 && delta_y == -2 {
                    knots[knot_idx].1 += -1; // move down
                    knots[knot_idx].0 += 1; // move right
                    continue;
                }
                // head 1 down and 2 right
                if delta_x == 2 && delta_y <= -1 {
                    knots[knot_idx].1 += -1; // move down
                    knots[knot_idx].0 += 1; // move right
                    continue;
                }
            }

            //println!("{:?}", knots);
            if !tail_positions.contains_key(&knots[9]) {
                tail_positions.insert(knots[9], 1);
            }
        }
        //println!("{:?}",knots);
        //print_knots(&knots, min_x, max_x, min_y, max_y)
    }
    tail_positions.len()
}
