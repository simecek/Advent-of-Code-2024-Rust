use std::fs;
use regex::Regex;

fn main() {
    // Read the content of the input.txt file
    let file_content = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    // Define the regular expression pattern
    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)";
    let re = Regex::new(pattern).unwrap();

    let mut total_sum: i64 = 0; // Initialize the sum
    let mut do_multiplication = true; // Flag to indicate if multiplication should be performed

    for caps in re.captures_iter(&file_content) {
        if let Some(mul_x) = caps.get(1) {
            // Match mul(X, Y)
            let x: i64 = mul_x.as_str().parse().unwrap();
            let y: i64 = caps.get(2).unwrap().as_str().parse().unwrap();
            if do_multiplication {
                let result = x * y;
                total_sum += result;
                println!("Found: mul({}, {}) = {}", x, y, result);
            } else {
                println!("Found: mul({}, {}), but multiplication is disabled", x, y);
            }
        } else if caps.get(0).map_or(false, |m| m.as_str() == "do()") {
            // Match do()
            do_multiplication = true;
            println!("Found: do(), enabling multiplication.");
        } else if caps.get(0).map_or(false, |m| m.as_str() == "don't()") {
            // Match don't()
            do_multiplication = false;
            println!("Found: don't(), disabling multiplication.");
        }
    }

    println!("Total Sum: {}", total_sum);
}
