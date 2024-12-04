use polars::prelude::*;
use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Open the file
    let file = File::open("input.txt")?;

    // Read the file into a DataFrame
    let df = CsvReader::new(file)
        .has_header(false)     // No header row
        .with_separator(b' ')  // Specify space as the delimiter
        .finish()?;

    // Extract and sort column_1 as a Series
    let mut col1 = df.column("column_1")?.clone(); // Clone to make it mutable
    col1 = col1.sort(false, false); // Sort in ascending order, nulls not last

    // Extract and sort column_4 as a Series
    let mut col4 = df.column("column_4")?.clone(); // Clone to make it mutable
    col4 = col4.sort(false, false); // Sort in ascending order, nulls not last

    let abs_diff: i64 = (&col1 - &col4)
        .i64()?
        .into_iter()
        .map(|x| x.map(|v| v.abs()).unwrap_or(0)) // Handle nulls gracefully
        .sum();

    // Print the result
    println!("Sum of absolute differences: {}", abs_diff);

    let col4_counts = df
        .column("column_4")?
        .value_counts(false, false)?;

    //println!("Schema of value_counts DataFrame:");
    //println!("{:?}", col4_counts.schema());

    let counts_map: std::collections::HashMap<i64, u32> = col4_counts
        .column("column_4")?
        .i64()?
        .into_iter()
        .zip(col4_counts.column("count")?.u32()?.into_iter())
        .filter_map(|(val, count)| val.zip(count)) // Handle nulls safely
        .collect();

    // Compute the total score by iterating through column_1
    let total_score: i64 = df
        .column("column_1")?
        .i64()?
        .into_iter()
        .filter_map(|val| val) // Filter out nulls
        .map(|x| x * *counts_map.get(&x).unwrap_or(&0) as i64) // Multiply by counts
        .sum();


    println!("Similarity score: {}", total_score);

    Ok(())
}
