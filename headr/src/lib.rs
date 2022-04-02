use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::{any::Any, error::Error};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    bytes: Option<usize>,
    lines: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("Mike-Mp")
        .about("Rust head")
        .arg(
            Arg::new("files")
            .value_name("FILES")
            .allow_invalid_utf8(true)
            .multiple_values(true)
            .default_value("-")
            .help("File name for cat to display it's contents")
        )
        .arg(
            Arg::new("bytes")
            .short('c')
            .long("bytes")
            .value_name("BYTES")
            .takes_value(true)
            .conflicts_with("lines")
            .help("print the first NUM bytes of each file; with the leading '-', print all but the last NUM bytes of each file")
        )
        .arg(
            Arg::new("lines")
            .short('n')
            .long("lines")
            .value_name("LINES")
            .default_value("10")
            .takes_value(true)
            .help("print the first NUM lines instead of the first 10; with the leading '-', print all but the last NUM lines of each file")
        )
        .get_matches();

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        bytes,
        lines: lines.unwrap(),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut i) => {
                print_contents(&mut i, config.lines, config.bytes);
            }
        }
    }
    Ok(())
}

fn print_contents(file: &mut Box<dyn BufRead>, lines: usize, bytes: Option<usize>) {
    if bytes.is_none() {
        for (i, line) in file.lines().enumerate() {
            if i != lines {
                println!("{}", line.unwrap());
            } else {
                break;
            }
        }
    } else {
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer);

        for value in buffer {
            println!("{}", value);
        }
    } 
}
