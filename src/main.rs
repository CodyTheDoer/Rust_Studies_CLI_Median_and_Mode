use std::io;
use std::collections::HashMap;

fn main() {
    println!("Welcome to the Mean and Median Idenitifier!");

    let mut storage_vec = Vec::new();
    manual_entry(&mut storage_vec);

    println!("Data Review: Presort");
    for i in storage_vec.iter() { 
        println!("{}", i); 
    }
    
    // convert stored vectors to floats
    let mut float_vec: Vec<f64> = storage_vec
        .iter()
        .map(|s| s.parse::<f64>().expect("Failed to parse string to integer"))
        .collect();

    // sort the floats
    float_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    parse_mode(&mut float_vec);
    parse_median(&mut float_vec);

    main_again();
}

fn parse_median(v: &mut Vec<f64>) {
    if v.len() % 2 == 0 { // Even Number
        let median_index_lower = (v.len() / 2) - 1;
        let median_index_upper = v.len() / 2;
        let median_value_lower = &v[median_index_lower];
        let median_value_upper = &v[median_index_upper];
        let median_value = (median_value_lower + median_value_upper) / 2.0;
        println!("Median: {median_value}");
    } else { // Odd Number
        let median_index = v.len() / 2;
        let median_value = &v[median_index];
        println!("Median: {median_value}");
    }
}

fn parse_mode(v: &mut Vec<f64>) {
    let mut map = HashMap::new();

    // Map values
    for i in v.iter() { 
        let count = map.entry(i.to_string()).or_insert(0);
        *count += 1;
    }

    let mut outer_vec: Vec<Vec<String>> = Vec::new();
    for (key, value) in map.iter() {
        let mut temp_vec: Vec<_> = Vec::new();
        temp_vec.push(format!("{key}: {value}"));
        let inner_vec = split_colon_space_return_two_part_vec(temp_vec[0].clone());
        outer_vec.push(inner_vec);
    }

    // Sort the data by most common occurance
    outer_vec.sort_by(|a, b| b[1].cmp(&a[1]));

    println!("Data Review: Postsort");
    println!("{:?}", outer_vec);

    if outer_vec[0][1] == outer_vec[1][1] {
        println!("Multi-Modal Dataset:");

        let mut matching_mode_occurance_counter: u8 = 1;
        let vec_total_count = outer_vec.len();

        for i in 1..vec_total_count {
            if outer_vec[i][1] == outer_vec[i-1][1] {
                matching_mode_occurance_counter += 1;
            } else {
                break;
            }
        }

        for i in 0..matching_mode_occurance_counter {
            println!("Mode: {:?}", outer_vec[i as usize]);
        }

    } else {
        println!("Mode: {:?}", outer_vec[0]);
    }
}

fn split_colon_space_return_two_part_vec(s: String) -> Vec<String> {
    let parts: Vec<String> = s
        .split(": ")
        .map(|part| part.to_string())
        .collect();
    parts
}

fn manual_entry(v: &mut Vec<String>) {
    let mut loop_check = true;

    while loop_check {
        println!("Please enter integer: 'X' to Exit");
    
        let mut user_entry = String::new();
        let mut clean_float = Vec::new();
    
        io::stdin()
            .read_line(&mut user_entry)
            .expect("Failed to read line");
    
        for char in user_entry.chars() {
            if char.is_ascii_digit() || char == '.'{
                clean_float.push(char as u8)
            }
        }
        
        if !clean_float.is_empty() {
            v.push(String::from_utf8(clean_float).expect("Our bytes should be valid utf8"));
        }
    
        for char in user_entry.chars() {
            if char == 'x' || char == 'X'{
                loop_check = false;
                println!("Exiting manual entry");
            }
        }
    }
}

fn main_again() {
    println!("Would you like to sequence a new Mean and Median? (Y/N)");

    let mut continue_yn = String::new();
    io::stdin()
        .read_line(&mut continue_yn)
        .expect("Failed to read line");

    match continue_yn.trim() {
        "y" | "Y" => {
            main()
        },
        "n" | "N" => {
            println!("Exiting...")
        },
        _ => {
            println!("Failure: Bad Input, exiting...")
        },
    };
}

