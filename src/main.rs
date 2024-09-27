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

    parse_median(&mut float_vec);
    parse_mode(&mut float_vec);

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

    // convert extract mapped values to vector for sorting data
    let mut vec: Vec<_> = Vec::new();
    for (key, value) in map.iter() {
        vec.push(format!("{key}: {value}"));
    }

    // Sort the data by most common occurance
    vec.sort_by(|a, b| b.cmp(a));

    println!("Data Review: Postsort");
    println!("{:?}", vec);
    println!("{:?}", vec[0].split(": "));

    let vec_key_zero = parse_sorted_vec_key(vec[0].clone());
    let vec_key_one = parse_sorted_vec_key(vec[1].clone());

    let vec_value_zero = parse_sorted_vec_value(vec[0].clone());
    
    if vec_key_zero == vec_key_one {
        println!("Multi-Modal Dataset Detected:");
        // If matching key values figure out how many, return that info as index_comp_count  
        let mut match_bool = true;
        let mut index_comp_count: u8 = 0;
        let vec_length = vec.len();
        let adj_vec_length = vec.len() - 1;
        while match_bool == true {
            for i in 0..adj_vec_length {
                let temp_zero = parse_sorted_vec_key(vec[i].clone());
                let temp_one = parse_sorted_vec_key(vec[i + 1].clone());
                if temp_zero != temp_one {
                    match_bool = false;
                }
                index_comp_count += 1;
                if usize::from(index_comp_count) == vec_length {
                    match_bool = false;                    
                }
            }
        }        
        // return n "Mode: Value" for each matching mode value
        for i in 0..index_comp_count {
            let temp_mode = parse_sorted_vec_value(vec[i as usize].clone());
            println!("Mode: {:?}", temp_mode)
        }
    }
    else {
        println!("Mode: {:?}", vec_value_zero);
    }
}

fn parse_sorted_vec_key(s: String) -> String {
    let mut post_whitespace = false;
    let mut vec_count = String::new();
    for char in s.chars() {
        if post_whitespace == true {
            vec_count.push(char);
        }
        if char.is_whitespace() {
            post_whitespace = true;
        }
    }
    return vec_count;
}

fn parse_sorted_vec_value(s: String) -> String {
    let mut post_whitespace = false;
    let mut vec_value = String::new();
    for char in s.chars() {
        if post_whitespace == false && (char.is_ascii_digit() || char == '.' ) {
            vec_value.push(char);            
        }
        if char.is_whitespace() {
            post_whitespace = true;
        }
    }
    return vec_value;
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

