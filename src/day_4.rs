pub fn part_1(input: &Vec<String>) -> i32 {
    let mut num_full_overlap = 0;

    for pair in input {
        let pair_split = pair.split(",");
        let pair_split_vec = pair_split.collect::<Vec<&str>>();

        let pair_1_split = pair_split_vec[0].split("-");
        let pair_1_split_vec = pair_1_split.collect::<Vec<&str>>();
        let range_1_start = pair_1_split_vec[0].parse::<i32>().unwrap();
        let range_1_stop = pair_1_split_vec[1].parse::<i32>().unwrap();

        let pair_2_split = pair_split_vec[1].split("-");
        let pair_2_split_vec = pair_2_split.collect::<Vec<&str>>();
        let range_2_start = pair_2_split_vec[0].parse::<i32>().unwrap();
        let range_2_stop = pair_2_split_vec[1].parse::<i32>().unwrap();

        if range_1_start >= range_2_start && range_1_stop <= range_2_stop {
            num_full_overlap += 1;
            continue;
        }

        if range_2_start >= range_1_start && range_2_stop <= range_1_stop {
            num_full_overlap += 1;
        }
    }

    num_full_overlap
}

pub fn part_2(input: &Vec<String>) -> i32 {
    let mut num_overlap = 0;

    for pair in input {
        let pair_split = pair.split(",");
        let pair_split_vec = pair_split.collect::<Vec<&str>>();

        let pair_1_split = pair_split_vec[0].split("-");
        let pair_1_split_vec = pair_1_split.collect::<Vec<&str>>();
        let range_1_start = pair_1_split_vec[0].parse::<i32>().unwrap();
        let range_1_stop = pair_1_split_vec[1].parse::<i32>().unwrap();

        let pair_2_split = pair_split_vec[1].split("-");
        let pair_2_split_vec = pair_2_split.collect::<Vec<&str>>();
        let range_2_start = pair_2_split_vec[0].parse::<i32>().unwrap();
        let range_2_stop = pair_2_split_vec[1].parse::<i32>().unwrap();

        if range_1_start >= range_2_start && range_1_start <= range_2_stop {
            num_overlap += 1;
            continue;
        }

        if range_1_stop >= range_2_start && range_1_stop <= range_2_stop {
            num_overlap += 1;
            continue;
        }

        if range_2_stop >= range_1_start && range_2_stop <= range_1_stop {
            num_overlap += 1;
            continue;
        }

        if range_2_start >= range_1_start && range_2_start <= range_1_stop {
            num_overlap += 1;
            continue;
        }
    }

    num_overlap
}
