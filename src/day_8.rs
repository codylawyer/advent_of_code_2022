fn check_visibility(tree_grid: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let current_tree_height = tree_grid[y][x];

    // check up
    let mut visible_up = true;
    let check_x = x;
    for check_y in 0..y {
        if tree_grid[check_y][check_x] >= current_tree_height {
            visible_up = false;
            break;
        }
    }

    // check right
    let mut visible_right = true;
    let check_y = y;
    for check_x in x + 1..tree_grid[0].len() {
        if tree_grid[check_y][check_x] >= current_tree_height {
            visible_right = false;
            break;
        }
    }

    // check down
    let mut visible_down = true;
    let check_x = x;
    for check_y in y + 1..tree_grid.len() {
        if tree_grid[check_y][check_x] >= current_tree_height {
            visible_down = false;
            break;
        }
    }

    // check left
    let mut visible_left = true;
    let check_y = y;
    for check_x in 0..x {
        if tree_grid[check_y][check_x] >= current_tree_height {
            visible_left = false;
            break;
        }
    }

    return visible_up || visible_right || visible_down || visible_left;
}

pub fn part_1(input: &Vec<String>) -> i32 {
    let mut tree_grid = Vec::new();

    for line in input {
        let mut current_row = Vec::new();
        for char in line.chars() {
            current_row.push(char.to_digit(10).unwrap());
        }
        tree_grid.push(current_row);
    }

    let grid_width = tree_grid[0].len();
    let grid_height = tree_grid.len();
    let mut num_visible = 0;

    for y in 0..grid_height {
        for x in 0..grid_width {
            if x == 0 || x == grid_width - 1 || y == 0 || y == grid_height - 1 {
                num_visible += 1;
                continue;
            }
            if check_visibility(&tree_grid, x, y) {
                num_visible += 1;
            }
        }
    }

    num_visible
}

pub fn part_2(input: &Vec<String>) -> i32 {
    0
}
