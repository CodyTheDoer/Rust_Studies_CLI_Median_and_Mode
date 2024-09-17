use std::io;

fn main() {
    let user_entry = inital_user_selection_data_type();
    println!("{user_entry}");
    main_again();
}

// fn manual_entry() {}

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
