
use clap::{App, Arg};

fn main() {
    let _matches = App::new("rwc")
                    .version("0.1")
                    .author("hpirlo")
                    .about("equivalent of `wc` command written in rust")
                    .arg(Arg::new("bytes")
                        .short('c')
                        .long("bytes")
                        .help("print the byte counts")
                    )
                    .arg(Arg::new("chars")
                        .short('m')
                        .long("chars")
                        .help("print the character counts")
                    )
                    .arg(Arg::new("lines")
                        .short('l')
                        .long("lines")
                        .help("print the newline counts")
                    )
                    .arg(Arg::new("debug")
                        .long("debug")
                    )
                    .arg(Arg::new("files0-from")
                        .long("files0-from")
                        .help("read input from the files specified by NUL-terminated names in file F; If F is - then read names from standard input")
                    )
                    .arg(Arg::new("max-line-length")
                        .long("max-line-length")
                        .help("print the maximum display width")
                    )
                    .arg(Arg::new("words")
                        .short('w')
                        .long("words")
                        .help("print the word counts")
                    )
                    .arg(Arg::new("help")
                        .long("help")
                        .help("output version information and exit")
                    )
                    .get_matches();

    // Retrieve values of command-line arguments
    // if let Some(input) = matches.value_of("input") {
    //     println!("Input file: {}", input);
    // }

    // if matches.is_present("verbose") {
    //     println!("Verbose mode is enabled");
    // }

}
