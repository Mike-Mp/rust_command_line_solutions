use clap::{Command, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("Mike-Mp")
        .about("Rust head")
        .arg(
            Arg::new("lines")
            .short('n')
            .long("lines")
            .value_name("LINES")
            .default_value("10")
            .conflicts_with("bytes")
            .takes_value(true)
        )
        .arg(
            Arg::new("bytes")
            .short('c')
            .value_name("BYTES")
            .long("bytes")
            .takes_value(true)
        )
        .arg(
            Arg::new("files")
            .value_name("FILE")
            .allow_invalid_utf8(true)
            .takes_value(true)
            .multiple_values(true)
            .default_value("-")
        )
        .get_matches();
    
    let lines = matches.value_of("lines")
    .map(parse_positive_int)
    .transpose()
    .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches.value_of("bytes")
    .map(parse_positive_int)
    .transpose()
    .map_err(|e| format!("illegal byte count -- {}", e))?;

    return Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes
    })
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    return match val.parse() {
        Ok(v) => {
            if v > 0 {
                Ok(v)
            } else {
                Err(val.into())
            }
        },
        Err(_) => Err(val.into()),
    };

    // way from book
    // match val.parse() {
    //     Ok(n) if n > 0 => Ok(n),
    //     _ => Err(From::from(val)),
    // }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");

    println!("{:?}", res);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}