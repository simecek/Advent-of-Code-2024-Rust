use std::fs;

fn main() {
    // Read the content of the input.txt file
    let file_content = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    // Process the file into a 2D vector
    let square: Vec<Vec<char>> = file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = square.len();
    let cols = if rows > 0 { square[0].len() } else { 0 };

    // Print the dimensions
    println!("Dimensions: {} rows x {} columns", rows, cols);

    if rows == 0 || cols == 0 {
        println!("Empty square. No rows, columns, or diagonals to process.");
        return;
    }

    // Define the function to process each sequence
    //let process_sequence = |sequence: &str| -> usize {
        // println!("Processing sequence: {}", sequence);
    //    sequence.len() // Count the number of characters in the sequence
    //};

    let process_sequence = |sequence: &str| -> usize {
        let substrings = ["XMAS", "SAMX"];
        substrings.iter().map(|&sub| sequence.matches(sub).count()).sum()
    };

    // Initialize the total sum
    let mut total_sum = 0;

    // Process all rows
    for row in &square {
        let row_string: String = row.iter().collect();
        total_sum += process_sequence(&row_string);
    }

    println!("Total sum of all sequence lengths: {}", total_sum);

    // Process all columns
    for col_idx in 0..cols {
        let col_string: String = (0..rows)
            .map(|row_idx| square[row_idx][col_idx])
            .collect();
        total_sum += process_sequence(&col_string);
    }

    println!("Total sum of all sequence lengths: {}", total_sum);

    // Process all diagonals (top-left to bottom-right)
    for start in 0..rows {
        let diagonal_string: String = (0..rows.min(cols))
            .map(|offset| square.get(start + offset).and_then(|row| row.get(offset)))
            .flatten()
            .collect();
        total_sum += process_sequence(&diagonal_string);
    }
    for start in 1..cols {
        let diagonal_string: String = (0..rows.min(cols))
            .map(|offset| square.get(offset).and_then(|row| row.get(start + offset)))
            .flatten()
            .collect();
        total_sum += process_sequence(&diagonal_string);
    }

    println!("Total sum of all sequence lengths: {}", total_sum);

    // Process all diagonals (top-right to bottom-left)
    for start in 0..cols {
        // Start from the top row, iterate diagonals going down-left
        let diagonal_string: String = (0..rows.min(cols - start))
            .map(|offset| {
                square
                    .get(offset)
                    .and_then(|row| row.get(cols - 1 - start - offset))
            })
            .flatten()
            .collect();
        total_sum += process_sequence(&diagonal_string);
    }

    for start in 1..rows {
        // Start from the first column of subsequent rows, iterate diagonals going down-left
        let diagonal_string: String = (0..rows.min(cols))
            .map(|offset| {
                square
                    .get(start + offset)
                    .and_then(|row| row.get(cols - 1 - offset))
            })
            .flatten()
            .collect();
        total_sum += process_sequence(&diagonal_string);
    }



    // Print the total sum
    println!("Total sum of all sequence lengths: {}", total_sum);
}
