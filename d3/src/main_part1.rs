use std::fs;
use regex::Regex;

fn main() {
    // Read the content of the input.txt file
    let file_content = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    // Define the regular expression pattern
    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let re = Regex::new(pattern).unwrap();

    let mut total_sum: i64 = 0; // Initialize the sum
    
    // Find all matches
    for caps in re.captures_iter(&file_content) {
        let x: i64 = caps[1].parse().unwrap(); // Parse X as an integer
        let y: i64 = caps[2].parse().unwrap(); // Parse Y as an integer
        let result = x * y; // Compute the multiplication result
        total_sum += result;
        println!("Found: mul({}, {}) = {}", x, y, result);
    }
    println!("Total Sum: {}", total_sum);
}
