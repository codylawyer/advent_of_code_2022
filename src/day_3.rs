pub fn part_1(rucksacks: &Vec<String>) -> i32 {
    let mut priority_total = 0;
    for rucksack in rucksacks {
        let rucksack_length = rucksack.len();
        let (first, second) = rucksack.split_at(rucksack_length / 2);

        for letter in first.chars() {
            if second.contains(letter) {
                if letter.is_ascii_lowercase() {
                    priority_total += (letter.to_ascii_uppercase() as u32 - 64) as i32;
                } else {
                    priority_total += (letter.to_ascii_lowercase() as u32 - 96 + 26) as i32;
                }
                break;
            }
        }
    }
    priority_total
}

pub fn part_2(rucksacks: &Vec<String>) -> i32 {
    let mut priority_total = 0;

    for group in 0..(rucksacks.len() / 3) {
        let member_1 = &rucksacks[group * 3];
        let member_2 = &rucksacks[group * 3 + 1];
        let member_3 = &rucksacks[group * 3 + 2];

        for letter in member_1.chars() {
            if member_2.contains(letter) && member_3.contains(letter) {
                if letter.is_ascii_lowercase() {
                    priority_total += (letter.to_ascii_uppercase() as u32 - 64) as i32;
                } else {
                    priority_total += (letter.to_ascii_lowercase() as u32 - 96 + 26) as i32;
                }
                break;
            }
        }
    }

    priority_total
}
