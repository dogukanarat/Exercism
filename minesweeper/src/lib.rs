fn check_mine_and_count(row: usize, col: usize, vec: &mut Vec<Vec<i32>>) {
    if vec[row][col] != -1 {
        vec[row][col] += 1;
    }
}
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = vec![];
    let size_row = minefield.len();

    if size_row > 0 {
        let size_col = minefield[0].len();
        let mut number_field: Vec<Vec<i32>> = vec![vec![0; size_col + 2]; size_row + 2];

        for (row, row_str) in minefield.iter().enumerate() {
            for (col, ch) in row_str.chars().enumerate() {
                let map_row = row + 1;
                let map_col = col + 1;
                if ch == '*' {
                    number_field[map_row][map_col] = -1;
                }
            }
        }

        for row in 1..(size_row + 1) {
            for col in 1..(size_col + 1) {
                if number_field[row][col] == -1 {
                    check_mine_and_count(row - 1, col - 1, &mut number_field);
                    check_mine_and_count(row - 1, col + 0, &mut number_field);
                    check_mine_and_count(row - 1, col + 1, &mut number_field);
                    check_mine_and_count(row + 0, col - 1, &mut number_field);
                    check_mine_and_count(row + 0, col + 1, &mut number_field);
                    check_mine_and_count(row + 1, col - 1, &mut number_field);
                    check_mine_and_count(row + 1, col + 0, &mut number_field);
                    check_mine_and_count(row + 1, col + 1, &mut number_field);
                }
            }
        }

        for (row_index, row) in number_field.iter().skip(1).enumerate() {
            let mut line = String::new();
            for (col_index, col) in row.iter().skip(1).enumerate() {
                if col_index < size_col && row_index < size_row {
                    if *col == -1 {
                        line.push('*');
                    } else {
                        if *col != 0 {
                            line.push(char::from(col.to_string().chars().next().unwrap()));
                        } else {
                            line.push(' ');
                        }
                    }
                }
            }
            if row_index < size_row {
                result.push(line);
            }
        }
    }

    result
}
