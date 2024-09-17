use std::io;

fn main() {
    let user_entry = inital_user_selection_data_type();
    let mut storage_vec = Vec::new();

    if user_entry == "M" {
        manual_entry(&mut storage_vec);            
    } else if user_entry == "F" {
        println!("F");
    } else {
        println!("{user_entry}");
    }

    println!("Vector Review:");
    for i in storage_vec.iter() { 
        println!("{}", i); 
    }

    // main_again();
}

fn manual_entry(v: &mut Vec<String>) {
    println!("Please enter integer:");

    let mut user_entry = String::new();
    let mut clean_float = Vec::new();

    io::stdin()
        .read_line(&mut user_entry)
        .expect("Failed to read line");

    for char in user_entry.chars(){
        if char.is_ascii_digit() || char == '.'{
            clean_float.push(char as u8)
        }
    }

    v.push(String::from_utf8(clean_float).expect("Our bytes should be valid utf8"));
}

// fn file_selection() {}

fn inital_user_selection_data_type() -> String {
    println!("Welcome to the Mean and Median Idenitifier!");
    println!("Please let me know, are you entering integers manually 'M' or using a pregenerated file 'F'? (M/F)");

    let mut data_entry_type_manual_or_file = String::new();

    io::stdin()
        .read_line(&mut data_entry_type_manual_or_file)
        .expect("Failed to read line");

    let trimmed_input = data_entry_type_manual_or_file.trim();

    match trimmed_input {
        "m" | "M" => {"M".to_string()},
        "f" | "F" => {"F".to_string()},
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
            println!("Exiting...");
            return;
        },
        _ => {
            println!("Failure: Bad Input, exiting...");
            return;
        },
    };
}
