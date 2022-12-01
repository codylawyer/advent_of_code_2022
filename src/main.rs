mod day_1;
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
        _ => (),
    }
}
