use std::{env, fs};

use regex::Regex;
use text_colorizer::Colorize;


struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String
}

fn print_help() {
    eprintln!("Usage: <target string> <replacement string> <INPUT FILE> <OUTPUT FILE>");
}

fn parse_args () -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        eprintln!("{} wrong number of arguments, Expected 4 got {}", "Error".red().bold(), args.len());
        print_help();
        std::process::exit(1);
    }
    
    Arguments { pattern: args[0].clone(), replace: args[1].clone(), input_file: args[2].clone(), output_file: args[3].clone() }
}

fn read_and_write(args: &Arguments) {
    let data = match fs::read_to_string(&args.input_file) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("{}", err.to_string().red());
            std::process::exit(1);
        }
    };
    
    let result = match replace(&data, &args.pattern, &args.replace) {
      Ok(str) => str,
      Err(err) => {
          eprintln!("{}", err.to_string().red());
          std::process::exit(1);
      }
    };
    
    match fs::write(&args.output_file, result) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("{}", err.to_string().red());
            std::process::exit(1);
        }
    };
    
}


fn replace (data: &str, target: &str, replace: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(data, replace).to_string())
}
pub fn run () {
    let args = parse_args();
    read_and_write(&args);
}