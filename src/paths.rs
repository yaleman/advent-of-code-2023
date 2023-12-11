use itertools::Itertools;

pub fn num_columns(input: &str) -> usize {
    input.lines().next().unwrap().len()
}

pub fn num_rows(input: &str) -> usize {
    input.lines().collect::<Vec<_>>().len()
}

/// Identify any columns in the input that contain only a .
///
/// Returns the list of columns which are empty
pub fn empty_columns(input: &str) -> Vec<usize> {
    let columns = num_columns(input);
    let mut result = vec![true; columns];
    for line in input.lines() {
        for (idx, char) in line.chars().enumerate() {
            if char == '#' {
                result[idx] = false;
            }
        }
    }

    result
        .into_iter()
        .enumerate()
        .filter_map(|(idx, val)| if val { Some(idx) } else { None })
        .collect()
}

/// Identify any rows in the input that contain only a .
///
/// Returns the list of columns which are empty
pub fn empty_rows(input: &str) -> Vec<usize> {
    input
        .lines()
        .enumerate()
        .filter_map(|(idx, val)| if val.contains('#') { None } else { Some(idx) })
        .collect()
}

/// Expand the string value of the galaxy to include the "double size" rows and columns
pub fn expand_galaxies(input: &str, extra_size: Option<usize>) -> String {
    let extra_size = extra_size.unwrap_or(1);

    let em_rows = empty_rows(input);
    let em_cols = empty_columns(input);

    let mut result = String::new();
    for (row_idx, row) in input.lines().enumerate() {
        let mut new_row = String::new();
        for (col_idx, char) in row.chars().enumerate() {
            if em_cols.contains(&col_idx) {
                for _ in 0..extra_size {
                    new_row.push('.');
                }
            }
            new_row.push(char);
        }
        result.push_str(&format!("{}\n", new_row));
        if em_rows.contains(&row_idx) {
            // add an extra row
            let row_string = vec!['.'; new_row.len()].iter().join("");
            for _ in 0..extra_size {
                result.push_str(&format!("{}\n", row_string));
            }
        }
    }

    result.trim().to_string()
}

/// Work out the coordinates of the galaxies
pub fn get_coordinates(input: &str) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, char) in row.chars().enumerate() {
            if char == '#' {
                result.push((col_idx, row_idx));
            }
        }
    }

    result
}

/// Calculate the shortest set of moves to get from one galaxy to another

pub fn shortest_paths(input: &[(usize, usize)]) -> Vec<usize> {
    let mut res = Vec::new();

    input.iter().combinations(2).for_each(|pair| {
        let (col1, row1) = pair[0];
        let (col2, row2) = pair[1];
        let x_moves = col1.abs_diff(*col2);
        let y_moves = row1.abs_diff(*row2);
        res.push(x_moves + y_moves);
    });
    res
}
