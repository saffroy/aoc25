use std::env;
use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 4 {
        panic!("Usage: {} <day num> <part num> <input_file>", args[0]);
    }

    // Read the input file
    let day = &args[1];
    let part = &args[2];
    let input_file_path = &args[3];
    let content = fs::read_to_string(input_file_path).expect("Failed to read the file");

    // Parse the content and print the result
    let result = match &day[..] {
        "1" => match &part[..] {
            "1" => day1::parse_1(&content),
            "2" => day1::parse_2(&content),
            _ => panic!("Invalid part {}", part)
        },
        "2" => match &part[..] {
            "1" => day2::parse_1(&content),
            "2" => day2::parse_2(&content),
            _ => panic!("Invalid part {}", part)
        },
        "3" => match &part[..] {
            "1" => day3::parse_1(&content),
            "2" => day3::parse_2(&content),
            _ => panic!("Invalid part {}", part)
        },
        "4" => match &part[..] {
            "1" => day4::parse_1(&content),
            "2" => day4::parse_2(&content),
            _ => panic!("Invalid part {}", part)
        },
        "5" => match &part[..] {
            "1" => day5::parse_1(&content),
            "2" => day5::parse_2(&content),
            _ => panic!("Invalid part {}", part)
        },
        "6" => match &part[..] {
            "1" => day6::parse_1(&content),
            "2" => day6::parse_2(&content),
            _ => panic!("Invalid part {}", part)
        },
        _ => panic!("Invalid day {}", day)
    };
    println!("Parsed result: {}", result);
}
