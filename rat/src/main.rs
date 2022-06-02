use colored::*;
use std::env;
use std::fs;
use termion::*;

fn main() {
    let (x, _) = termion::terminal_size().unwrap();
    let mut divider = String::with_capacity(x as usize);
    for _ in 0..x {
        divider.push('-');
    }

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: {} <filename>", args[0]);
    }
    let file_path = &args[1];

    println!("{}", divider);
    println!("| {} | {}", "File".blue(), file_path.green());
    println!("{}", divider);

    let content =
        fs::read_to_string(file_path).expect(&format!("Error reading the file {}", file_path));
    for (line_number, line) in content.lines().enumerate() {
        let line_number = format!("{:4}", line_number + 1).yellow();
        println!("| {} | {}", line_number, line);
    }
    println!("{}", divider);
}
