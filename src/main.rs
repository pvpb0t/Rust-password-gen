
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::prelude::*;
use std::{env, process};
use std::process::Command;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: ./password-gen.exe NUM_PASSWORDS PASSWORD_LENGTH");
        process::exit(1);
    }
    
    let num_passwords: u32 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: NUM_PASSWORDS must be a positive integer.");
            process::exit(1);
        }
    };
    let password_length: u32 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: PASSWORD_LENGTH must be a positive integer.");
            process::exit(1);
        }
    };
    
    let rng = thread_rng();
    let mut passwords = Vec::new();
    
    println!("Generating {} random passwords of length {}...", num_passwords, password_length);
    for i in 0..num_passwords {
        let password: String = rng.sample_iter(&rand::distributions::Alphanumeric).take(password_length as usize).collect();
        passwords.push(password);
        display_progress_bar(i, num_passwords, 50);
    }
    
    let mut file = match File::create("passwords.json") {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error creating file: {}", error);
            process::exit(1);
        }
    };
    
    match file.write_all(format!("{:?}", passwords).as_bytes()) {
        Ok(_) => println!("Successfully wrote passwords to file"),
        Err(e) => println!("Error writing to file: {}", e),
    }
}


fn display_progress_bar(done: u32, total: u32, width: u32) {
    let progress = (done as f64 / total as f64) * 100.0;
    let progress_width = (done as f64 / total as f64) * (width as f64);
    println!("[{:>width$}] {:.2}%\r", "=".repeat(progress_width as usize), progress, width=width as usize);
}

