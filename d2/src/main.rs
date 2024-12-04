use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Define the path to your file
    let path = "input.txt";

    // Open the file
    let file = File::open(&path)?;

    // Create a buffered reader for efficient reading
    let reader = io::BufReader::new(file);

    let mut number_of_safe_lines:u64 = 0;

    // Process each line in the file
    for line in reader.lines() {
        let line = line?; // Handle any errors that might occur
        // Split the line into integers
        let numbers: Vec<i32> = line
            .split_whitespace() // Split by whitespace
            .filter_map(|s| s.parse::<i32>().ok()) // Parse to i32 and filter invalid inputs
            .collect();

        if numbers.is_empty() {
            println!("Line is empty or contains no valid numbers.");
            continue;
        }

        // Check if the numbers are increasing
        let is_increasing = numbers.windows(2).all(|pair| pair[0] < pair[1] && (pair[1] - pair[0]) <= 3);

        // Check if the numbers are decreasing
        let is_decreasing = numbers.windows(2).all(|pair| pair[0] > pair[1] && (pair[0] - pair[1]) <= 3);

        // Print the results
        if is_increasing {
            number_of_safe_lines += 1;
        } else if is_decreasing {
            number_of_safe_lines +=1;
        } 
    }
    
    println!("Number of safe lines: {:?}", number_of_safe_lines);

    Ok(())
}

