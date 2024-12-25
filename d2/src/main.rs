use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Define the path to your file
    let path = "input.txt";

    // Open the file
    let file = File::open(&path)?;

    // Create a buffered reader for efficient reading
    let reader = io::BufReader::new(file);

    let mut number_of_safe_lines: u64 = 0;

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

        // To get the first part of Day2, just change this function to is_line_safe
        if is_line_safe_with_skip(&numbers) {
            number_of_safe_lines += 1;
        }
    }

    println!("Number of safe lines with skip: {:?}", number_of_safe_lines);

    Ok(())
}

// Function to check if a line is safe (increasing or decreasing with differences <= 3)
fn is_line_safe(numbers: &[i32]) -> bool {
    let is_increasing = numbers.windows(2).all(|pair| pair[0] < pair[1] && (pair[1] - pair[0]) <= 3);
    let is_decreasing = numbers.windows(2).all(|pair| pair[0] > pair[1] && (pair[0] - pair[1]) <= 3);

    is_increasing || is_decreasing
}

fn is_line_safe_with_skip(numbers: &[i32]) -> bool {
    // First, check the line as it is
    if is_line_safe(numbers) {
        return true;
    }

    // Try skipping one element at a time
    for i in 0..numbers.len() {
        let mut skipped = numbers.to_vec();
        skipped.remove(i); // Remove the element at index `i`
        if is_line_safe(&skipped) {
            return true;
        }
    }

    // If none of the cases are safe, return false
    false
}
