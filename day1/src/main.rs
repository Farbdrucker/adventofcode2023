use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn open_file(file_path: &PathBuf) -> File {
    match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Unable to open the file at '{:?}'", file_path);
            std::process::exit(1);
        }
    }
}

fn read_to_string(file: &File) -> String {
    let mut content = String::new();
    if let Err(_) = file.take(usize::MAX as u64).read_to_string(&mut content) {
        eprintln!("Error: Unable to read the content of the file");
        std::process::exit(1);
    };
    content
}

fn extract_coords(input: &str) -> Option<u32> {
    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;

    for c in input.chars() {
        if c.is_digit(10) {
            if first_digit.is_none() {
                // Found the first digit
                first_digit = Some(c.to_digit(10).unwrap());
            }
            // Always update the last_digit to get the very last digit
            last_digit = Some(c.to_digit(10).unwrap());
        }
    }

    // Stack the first and last digits to make a two-digit number
    if let (Some(first), Some(last)) = (first_digit, last_digit) {
        let result = first * 10 + last;
        Some(result)
    } else {
        None // No digits found
    }
}

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    // Extract the file path from the command-line arguments
    let file_path = PathBuf::from(&args[1]);

    // Attempt to open the file
    let file = open_file(&file_path);

    // read the content of the file
    let content = read_to_string(&file);

    // Split the content into a vector of strings based on newline
    let lines: Vec<&str> = content.lines().collect();

    let mut sum: u32 = 0;
    let mut _coord: Option<u32>;
    // Print each line
    for line in lines {
        _coord = extract_coords(&line);

        if let Some(value) = _coord {
            sum += value;
        }
    }

    println!("Sum: {}", sum)
}
