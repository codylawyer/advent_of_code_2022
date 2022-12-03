mod day_1;
mod day_2;
mod day_3;
mod utils;
use std::env;

fn main() {
    // get day and filename
    let args: Vec<String> = env::args().collect();
    let day = &args[1].parse::<i32>().unwrap();
    let filename = &args[2];

    match day {
        1 => {
            // convert to vector of ints
            let mut input = utils::day_1_read(&filename);
            println!("{}", day_1::part_1(&input));
            println!("{}", day_1::part_2(&mut input));
        }
        2 => {
            let input: (String, String) = utils::day_2_read(&filename);
            let mine = input.0;
            let opponent = input.1;
            println!("{}", day_2::part_1(&mine, &opponent));
            println!("{}", day_2::part_2(&mine, &opponent));
        }
        3 => {
            let input: Vec<String> = utils::day_3_read(&filename);
            println!("{}", day_3::part_1(&input));
            println!("{}", day_3::part_2(&input));
        }
        _ => (),
    }
}
