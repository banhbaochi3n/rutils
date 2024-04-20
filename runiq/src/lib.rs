use clap::{Arg, App};
use std::{error::Error, fs::File, io::{self, BufRead, BufReader}};

type R<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> R<Config> {
    let matches = App::new("runiq")
        .version("0.1.0")
        .author("Mark Vien <iluvshinonomenano@waifu.club")
        .about("Rust uniq")
        .arg(
            Arg::with_name("in_file")
                .value_name("IN_FILE")    
                .help("Input file")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("out_file")
                .value_name("OUT_FILE")    
                .help("Output file"),
        )
        .arg(
            Arg::with_name("count")
                .short("c")    
                .long("count")
                .help("Show counts")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        in_file: matches.value_of_lossy("in_file").map(|v| v.into()).unwrap(),
        out_file: matches.value_of("out_file").map(|v| v.to_string()),
        count: matches.is_present("count"),
    })
}

fn open(filename: &str) -> R<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> R<()> {
    let mut line = String::new();
    let mut previous = String::new();
    let mut count: usize = 0;
    let mut file = open(&config.in_file).map_err(|e| format!("{}: {}", config.in_file, e))?;
    // let mut out_file: Box<dyn Write> = match &config.out_file {
    //     Some(out_name) => Box::new(File::create(out_name)?),
    //     _ => Box::new(io::stdin()),
    // };

    let print = |count: usize, text: &str| -> R<()> {
        if count > 0 {
            if config.count {
                println!("{:>4} {}", count, text);
            } else {
                println!("{}", text);
            }
        }

        Ok(())
    };

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if line.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous = line.clone();
            count = 0;
        }
        count += 1;
        line.clear();
    }

    print(count, &previous)?;

    Ok(())
}
