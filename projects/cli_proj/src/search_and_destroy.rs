use text_colorizer::*;
use std::env;
use::std::fs;
use regex::Regex;

#[derive(Debug)]
#[allow(dead_code)]
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String
}

fn print_help() {
    eprintln!("{} - replace a string with a new string", "Search and Destroy".cyan());
    eprintln!("Usage: <target string> <repalcement string> <INPUT_FILE> <OUTPUT_FILE>");

}

fn parse_args() -> Arguments {
    print_help();
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        eprintln!("{}: wrong number of arguments given, Expected 4, got {}", "ERROR".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments { 
        pattern: args[0].clone(), 
        replace: args[1].clone(), 
        input_file: args[2].clone(), 
        output_file: args[3].clone() 
    }
}

fn replace(target: &str, rep: &str, data: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(data, rep).to_string())
}


fn read_and_write(args: &Arguments) {
    let data = match fs::read_to_string(&args.input_file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}: failed to read from file {}: {:?}", "ERROR".red(), args.input_file, e);
            std::process::exit(1);
        }
    };

    let replace_data = match replace(&args.pattern, &args.replace, &data) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}", "ERROR".red().bold(), e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output_file, &replace_data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{}: failed to write to file {}: {:?}", "ERROR".red().bold(), args.input_file, e);
            std::process::exit(1);
        }
    }

}

pub fn run() {
    let args: Arguments = parse_args();
    read_and_write(&args);
}
