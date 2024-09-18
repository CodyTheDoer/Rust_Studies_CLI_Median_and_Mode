use std::io;

fn main() {
    println!("Welcome to the Mean and Median Idenitifier!");

    let mut storage_vec = Vec::new();
    data_entry_type_selection(&mut storage_vec);
    main_again();
}

fn data_entry_type_selection(v: &mut Vec<String>) {
    let user_entry = inital_user_selection_data_type();

    if user_entry == "M" {
        manual_entry(v);
    } else { // Direct vector entry for first input and then run manual input loop
        let mut clean_float = Vec::new();
        for char in user_entry.chars() {
            if char.is_ascii_digit() || char == '.'{
                clean_float.push(char as u8)
            }
        }
        v.push(String::from_utf8(clean_float).expect("Our bytes should be valid utf8"));
        manual_entry(v);
    }

    println!("Vector Review:");
    for i in v.iter() { 
        println!("{}", i); 
    }
}

fn inital_user_selection_data_type() -> String {
    println!("Please let me know, are you entering integers manually 'M' or using a pregenerated file 'F'? (M/F)");

    let mut data_entry_type_manual_or_file = String::new();

    io::stdin()
        .read_line(&mut data_entry_type_manual_or_file)
        .expect("Failed to read line");

    let trimmed_input = data_entry_type_manual_or_file.trim();

    match trimmed_input {
        "m" | "M" => {"M".to_string()},
        _=> {
            match trimmed_input.parse::<f32>() {
                Ok(value) => {value.to_string()},
                Err(_) => {
                    "Invalid Input".to_string()
                },
            }
        },
    }
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
    
        v.push(String::from_utf8(clean_float).expect("Our bytes should be valid utf8"));
    
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

