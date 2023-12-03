use std::cmp;
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

#[derive(Debug)]
struct Number {
    value: u32,
    row: usize,
    col: usize,
}

impl Number {
    // Method to calculate the number of digits in the value
    fn len(&self) -> usize {
        // Convert the u32 value to a string and return the length
        self.value.to_string().len()
    }

    fn end(&self) -> usize {
        self.value.to_string().len() + self.col
    }
}

fn extract_numbers(line: &str, row: usize) -> Vec<Number> {
    let mut numbers = Vec::new();
    let mut start = 0;

    for (col, c) in line.chars().enumerate() {
        if c.is_ascii_alphanumeric() {
            if start == col
                || !line[start..=col]
                    .chars()
                    .all(|c| char::is_ascii_alphanumeric(&c))
            {
                let value: u32 = line[start..=col].parse().unwrap_or(0);
                numbers.push(Number { value, row, col });
            }
        } else {
            start = col + 1;
        }
    }

    numbers
}

fn compute_sum(lines: &[&str]) -> u32 {
    let mut sum: u32 = 0;
    let max_row: usize = lines.len();

    for (row, line) in lines.iter().enumerate() {
        let numbers: Vec<Number> = extract_numbers(line, row);
        for number in numbers {
            let row_start: usize = row.saturating_sub(1);
            let row_end: usize = cmp::min(row + 2, max_row);
            let col_start: usize = number.col.saturating_sub(1);
            let col_end: usize = number.col + number.value as usize;

            for inspect_line in lines.iter().skip(row_start).take(row_end - row_start) {
                let col_end: usize = cmp::min(col_end, inspect_line.len());
                if inspect_line[col_start..col_end]
                    .chars()
                    .all(|c| char::is_ascii_alphanumeric(&c))
                {
                    sum += number.value;
                    break;
                }
            }
        }
    }
    sum
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

    let lines: Vec<&str> = content.lines().collect();
    let sum = compute_sum(&lines);

    println!("Sum is {}", sum);
}
