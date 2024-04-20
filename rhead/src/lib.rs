use clap::{Arg, App};
use std::{error::Error, fs::File, io::{self, Read, BufRead, BufReader}, u64};

type R<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

fn parse_positive_int(val: &str) -> R<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

pub fn get_args() -> R<Config> {
    let matches = Command::new("rhead")
        .version("0.1.0")
        .author("Mark Vien <iluvshinonomenano@waifu.club")
        .about("Rust head")
        .arg(
            Arg::with_name("lines")
                .short("n")    
                .long("lines")
                .value_name("LINES")
                .help("Number of lines to read")
                .default_value("10")
                .multiple(false),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c") 
                .long("bytes")
                .value_name("BYTES")
                .help("Number of bytes to read")
                .takes_value(true)
                .conflicts_with("lines")
                .multiple(false),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")    
                .help("Input file(s)")
                .default_value("-")
                .multiple(true),
        )
        .get_matches();
    let lines = matches
        .value_of("lines") 
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("Illegal line count -- {}", e))?;
    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("Illegal byte count -- {}", e))?;
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes,
    })
}

fn open(filename: &str) -> R<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }    
}

pub fn run(config: Config) -> R<()> {
    let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("Could not read from {}: {}", filename, err),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }
                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    println!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            },
        }
    }
    Ok(())
}
