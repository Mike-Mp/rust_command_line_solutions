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
            .default_value("10")
            .conflicts_with("bytes")
            .takes_value(true)
        )
        .arg(
            Arg::new("bytes")
            .short('c')
            .long("bytes")
            .takes_value(true)
        )
        .arg(
            Arg::new("files")
            .takes_value(true)
            .default_value("-")
        )
        .get_matches();

    let lines = matches.value_of("lines").unwrap()
        .parse_positive_int()
        .map_err(|e| println!("{}", e));

    return Ok(Config {
        files: matches.value_of("files").unwrap(),
        lines: 10,
        bytes: std::option::Option::Some(1),
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