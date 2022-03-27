use clap::{Command, Arg};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Mike-Mp")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                 .value_name("TEXT")
                 .allow_invalid_utf8(true)
                 .help("Input text")
                 .required(true)
                 .min_values(1),
        )
        .arg(
            Arg::new("omit_newline")
                 .short('n')
                 .help("Do not print newline")
                 .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    let ending = if omit_newline { "" } else { "\n" };
    print!("{}{}", text.join(" "), ending);
}
