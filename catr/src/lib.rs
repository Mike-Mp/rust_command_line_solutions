use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
    show_ends: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => read(file, config.clone()),
        }
    }
    Ok(())
}

pub fn read(file: Box<dyn BufRead>, config: Config) {
    let lines_iter = file.lines();

    let mut num = 0;
    for (i, line_result) in lines_iter.enumerate() {
        match (config.number_lines, config.number_nonblank_lines, config.show_ends) {
            (true, false ,false) => print_number_lines(i, line_result.unwrap(), config.show_ends),
            (true, false, true) => print_number_lines(i, line_result.unwrap(), config.show_ends),
            (false, true, false) => {
                let line = line_result.unwrap();
                if line.is_empty() {
                    println!("{}", line);
                } else {
                    num += 1;
                    println!("{:6}\t{}", num, line);
                }
            },
            (false, true, true) => {
                let line: String = line_result.unwrap();
                if line.is_empty() {
                    println!("{}?", line);
                } else {
                    num += 1;
                    println!("{:6}\t{}$", num, line);
                }
            }
            (false, false, false) => println!("{}", line_result.unwrap()),
            (true, true, true) => println!(""),
            (true, true, false) => println!(""),
            (false, false, true) => println!("{}$", line_result.unwrap()),
        }
    }
}

fn print_number_lines(index: usize, line: String, show_ends: bool) {
    if show_ends {
        println!("{:6}\t{}$", index+1, line)
    } else {
        println!("{:6}\t{}", index+1, line)
    }
}


pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Mike-Mp")
        .about("rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .allow_invalid_utf8(true)
                .multiple_values(true)
                .default_value("-")
                .help("File name for cat to display it's contents"),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .help("number all output lines")
                .takes_value(false),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank")
                .help("number nonempty output lines, ovverides -n")
                .conflicts_with("number_lines")
                .takes_value(false),
        )
        .arg(
            Arg::new("show_ends")
                .short('E')
                .long("show-ends")
                .help("display $ at the end of each line")
                .takes_value(false)
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
        show_ends: matches.is_present("show_ends"),
    })
}
