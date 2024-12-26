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

    if rows < 3 || cols < 3 {
        println!("Grid is too small to contain any valid 'A'.");
        return;
    }

    // Store valid positions
    let mut valid_positions = vec![];

    // Iterate through the grid
    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            // Check if the current character is 'A'
            if square[i][j] == 'A' {
                // Get the diagonal positions
                let top_left = square[i - 1][j - 1];
                let bottom_left = square[i + 1][j - 1];
                let top_right = square[i - 1][j + 1];
                let bottom_right = square[i + 1][j + 1];

                // Check if there are two 'M' and two 'S' diagonally
                let diagonals = vec![top_left, bottom_left, top_right, bottom_right];
                let m_count = diagonals.iter().filter(|&&c| c == 'M').count();
                let s_count = diagonals.iter().filter(|&&c| c == 'S').count();

                // Check the side positions for two connected 'M's
                let side_match = top_left != bottom_right && bottom_left != top_right;

                if m_count == 2 && s_count == 2 && side_match {
                    valid_positions.push((i, j));
                }
            }
        }
    }

    // Output the valid positions
    if valid_positions.is_empty() {
        println!("No valid 'A' positions found.");
    } else {
        println!("Valid 'A' positions:");
        for (i, j) in &valid_positions { // Iterate over a reference to avoid moving
            println!("({}, {})", i, j);
        }
        println!("Number of valid 'A' positions: {}", valid_positions.len());
    }
}
