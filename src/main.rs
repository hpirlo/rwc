
use clap::{App, Arg};

fn main() {
    let _matches = App::new("rwc")
                    .version("0.1")
                    .author("hpirlo")
                    .about("equivakent of `wc` command written in rust")
                    .arg(Arg::new("bytes")
                        .short('c')
                        .long("bytes")
                        // .about("Sets the input file to use")
                        // .index(1))
                    )
                    .arg(Arg::new("chars")
                        .short('m')
                        .long("chars")
                        // .about("Sets the level of verbosity")
                    )
                    .arg(Arg::new("lines")
                        .short('l')
                        .long("lines")
                        // .about("Sets the level of verbosity")
                    )
                    .arg(Arg::new("words")
                        .short('w')
                        .long("words")
                        // .about("Sets the level of verbosity")
                    )
                    .arg(Arg::new("debug")
                        .long("debug")
                        // .about("Sets the level of verbosity")
                    )
                    .arg(Arg::new("files0-from")
                        .long("files0-from")
                        // .about("Sets the level of verbosity")
                    )
                    .arg(Arg::new("max-line-length")
                        .long("max-line-length")
                        // .about("Sets the level of verbosity")
                    )
                    .arg(Arg::new("total")
                        .long("total")
                        // .index(1)
                        // .about("Sets the level of verbosity")
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
