fn get_mine_count(x: i32, y: i32, minefield: &[&str]) -> i32 {
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i != 0 || j != 0 {
                let n_x = x + i;
                let n_y = y + j;

                if n_x >= 0
                    && n_y >= 0
                    && n_x < minefield[0].len() as i32
                    && n_y < minefield.len() as i32
                {
                    let c = minefield[n_y as usize].as_bytes()[n_x as usize];
                    if c == b'*' {
                        count += 1;
                    }
                }
            }
        }
    }
    return count;
}
pub fn annotate(minefield: &Vec<&str>) -> Vec<String> {
    let mut new_board: Vec<String> = Vec::new();

    for (i, row) in minefield.iter().enumerate() {
        new_board.push(String::new());
        for (j, col) in row.chars().enumerate() {
            let mut count = 0;
            if col == ' ' {
                count = get_mine_count(j as i32, i as i32, minefield);
                if count > 0 {
                    new_board[i].push_str(&count.to_string());
                }
            }
            if count == 0 {
                new_board[i].push_str(&col.to_string());
            }
        }
    }

    return new_board;
}
