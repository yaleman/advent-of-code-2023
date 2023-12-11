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
pub fn expand_galaxies(input: &str) -> String {
    let em_rows = empty_rows(input);
    let em_cols = empty_columns(input);

    let mut result = String::new();
    for (row_idx, row) in input.lines().enumerate() {
        if em_rows.contains(&row_idx) {
            for _ in 0..=(row.len() + em_rows.len()) {
                result.push('.');
            }
            result.push('\n');
        }
        for (col_idx, char) in row.chars().enumerate() {
            if em_cols.contains(&col_idx) {
                result.push('.');
            }
            result.push(char);
        }
        result.push('\n');
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
