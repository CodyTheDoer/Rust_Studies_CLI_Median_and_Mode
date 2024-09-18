use std::io;

fn main() {
    println!("Welcome to the Mean and Median Idenitifier!");

    let mut storage_vec = Vec::new();
    manual_entry(&mut storage_vec);

    println!("Data Review:");
    for i in storage_vec.iter() { 
        println!("{}", i); 
    }

    parse_median(&storage_vec);
    parse_mode(&storage_vec);

    main_again();
}

fn parse_median(v: &Vec<String>) {
    for i in v.iter() { 
        println!("Median {}", i); 
    }
}
fn parse_mode(v: &Vec<String>) {
    for i in v.iter() { 
        println!("Mode {}", i); 
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

