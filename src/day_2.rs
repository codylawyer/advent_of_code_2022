pub fn part_1(mine: &String, opponent: &String) -> i32 {
    let mut total_score = 0;

    for idx in 0..opponent.len() {
        let current_opponent = opponent.chars().nth(idx).unwrap();
        let current_mine = mine.chars().nth(idx).unwrap();

        if current_opponent == 'A' {
            if current_mine == 'X' {
                // Rock v Rock Tie
                total_score += 3 + 1;
            }
            if current_mine == 'Y' {
                // Rock v Paper Win
                total_score += 6 + 2;
            }
            if current_mine == 'Z' {
                // Rock v Scissors Loss
                total_score += 0 + 3;
            }
        }
        if current_opponent == 'B' {
            if current_mine == 'X' {
                // Paper v Rock Loss
                total_score += 0 + 1;
            }
            if current_mine == 'Y' {
                // Paper v Paper Tie
                total_score += 3 + 2;
            }
            if current_mine == 'Z' {
                // Paper v Scissors Win
                total_score += 6 + 3;
            }
        }
        if current_opponent == 'C' {
            if current_mine == 'X' {
                // Scissors v Rock Win
                total_score += 6 + 1;
            }
            if current_mine == 'Y' {
                // Scissors v Paper Loss
                total_score += 0 + 2;
            }
            if current_mine == 'Z' {
                // Scissors v Scissors Tie
                total_score += 3 + 3;
            }
        }
    }
    total_score
}

pub fn part_2(result: &String, opponent: &String) -> i32 {
    let mut total_score = 0;

    for idx in 0..result.len() {
        let current_opponent = opponent.chars().nth(idx).unwrap();
        let current_result = result.chars().nth(idx).unwrap();

        if current_result == 'X' {
            // need to lose
            if current_opponent == 'A' {
                // Rock, need to play scissors
                total_score += 3;
            }
            if current_opponent == 'B' {
                // Paper, need to play rock
                total_score += 1;
            }
            if current_opponent == 'C' {
                // Scissors, need to play paper
                total_score += 2;
            }
        }
        if current_result == 'Y' {
            // need to tie
            total_score += 3;
            if current_opponent == 'A' {
                // Rock, need to play rock
                total_score += 1;
            }
            if current_opponent == 'B' {
                // Paper, need to play paper
                total_score += 2;
            }
            if current_opponent == 'C' {
                // Scissors, need to play scissors
                total_score += 3;
            }
        }
        if current_result == 'Z' {
            // need to win
            total_score += 6;
            if current_opponent == 'A' {
                // Rock, need to play paper
                total_score += 2;
            }
            if current_opponent == 'B' {
                // Paper, need to play scissors
                total_score += 3;
            }
            if current_opponent == 'C' {
                // Scissors, need to play rock
                total_score += 1;
            }
        }
    }
    total_score
}
