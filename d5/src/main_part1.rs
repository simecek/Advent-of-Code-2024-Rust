use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "input.txt";
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut second_section: Vec<Vec<u32>> = Vec::new();

    let mut is_second_section = false;

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            is_second_section = true;
            continue;
        }

        if !is_second_section {
            if let Some((j_str, i_str)) = line.split_once('|') {
                if let (Ok(j), Ok(i)) = (j_str.parse::<u32>(), i_str.parse::<u32>()) {
                    map.entry(i).or_insert_with(Vec::new).push(j);
                }
            }
        } else {
            let values: Result<Vec<u32>, _> = line.split(',').map(|s| s.trim().parse::<u32>()).collect();
            if let Ok(vec) = values {
                second_section.push(vec);
            }
        }
    }

    let mut final_output:u32 = 0;

    for sequence in &second_section {
        let mut present = HashMap::new();
        for &i in sequence {
            present.insert(i, true);
        }

        let mut visited = HashMap::new();
        let mut valid = true;

        for &i_j in sequence {
            visited.insert(i_j, true);

            if let Some(values) = map.get(&i_j) {
                for &value in values {
                    if !present.contains_key(&value) {
                        continue;
                    }
                    if !visited.contains_key(&value) {
                        println!("Validation failed for value {} that expects prior {} in sequence {:?} {:?}", &i_j, value, sequence, map.get(&i_j)); // Debug failure
                        valid = false;
                        break;
                    }
                }
            }

            if !valid {
                break;
            }
        }

        if valid {
            let middle_index = sequence.len() / 2;
            final_output += sequence[middle_index];
        }

    }

    println!("Final output: {}", final_output); // Display final output

    Ok(())
}
