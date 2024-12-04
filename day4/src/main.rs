fn matrix<P: AsRef<str>>(p: P) -> Vec<Vec<char>> {
    let lines = utils::read_lines(p);
    lines.into_iter().map(|v| v.chars().into_iter().collect()).collect()
}


// I misinterpreted the problem as a snake search type of situation. We can ignore :(
//
// fn count(needle: char, want: char, haystack: &Vec<Vec<char>>, counts: &mut Vec<Vec<usize>>) -> usize {
//     let rows = haystack.len();
//     let cols = haystack[0].len();
//
//     let mut sum = 0;
//
//     for row in 0..rows {
//         for col in 0..cols {
//             if haystack[row][col] != needle {
//                 continue;
//             }
//
//             for row_delta in vec![-1_isize, 0, 1] {
//                 for col_delta in vec![-1_isize, 0, 1] {
//                     let search_row = row as isize + row_delta;
//                     let search_col = col as isize + col_delta;
//
//                     if search_row < 0 || search_col < 0 || search_row as usize >= rows || search_col as usize >= cols {
//                         continue;
//                     }
//
//                     let search_row = search_row as usize;
//                     let search_col = search_col as usize;
//
//                     if haystack[search_row][search_col] == want {
//                         counts[row][col] += counts[search_row][search_col];
//                         sum += counts[row][col];
//                     }
//                 }
//             }
//         }
//     }
//
//     sum
// }

fn search_from(start: (usize, usize), char_matrix: &Vec<Vec<char>>, direction: (isize, isize)) -> bool {
    let rows = char_matrix.len() as isize;
    let cols = char_matrix[0].len() as isize;

    let mut want = vec!['S', 'A', 'M'];
    let mut pos = start;

    let (rd, cd) = direction;

    // Invariant: the current position is valid and is the position
    // where we move from first to find the `want` character.
    while let Some(want) = want.pop() {
        let (row, col) = pos;

        // Apply direction
        let row = row as isize + rd;
        let col = col as isize + cd;

        // Out of bounds?
        if row < 0 || col < 0 || row >= rows || col >= cols {
            return false;
        }

        // Prepare for next iteration
        let row = row as usize;
        let col = col as usize;

        // Ensure we got the right character
        if char_matrix[row][col] != want {
            return false;
        }

        pos = (row, col);
    }

    true
}

fn part1<P: AsRef<str>>(p: P) -> usize {
    let char_matrix = matrix(p);

    let rows = char_matrix.len();
    let cols = char_matrix[0].len();

    let mut xmas = 0;

    for row in 0..rows {
        for col in 0..cols {
            if char_matrix[row][col] == 'X' {
                let start = (row, col);

                for row_delta in vec![-1, 0, 1_isize] {
                    for col_delta in vec![-1, 0, 1_isize] {
                        if row_delta == 0 && col_delta == 0 {
                            continue;
                        }

                        if search_from(start, &char_matrix, (row_delta, col_delta)) {
                            xmas += 1;
                        }
                    }
                }
            }
        }
    }


    xmas
}


fn main() {
    println!("Part 1: {0}", part1("../inputs/day4.txt"))
}


#[cfg(test)]
mod tests {
    #[test]
    fn print_directions() {
        for row_delta in vec![-1, 0, 1_isize] {
            for col_delta in vec![-1, 0, 1_isize] {
                if row_delta == 0 && col_delta == 0 {
                    continue;
                }

                println!("{row_delta} {col_delta}")
            }
        }
    }
}