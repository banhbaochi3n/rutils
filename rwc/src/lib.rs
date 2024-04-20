use clap::{Arg, App};
use std::{error::Error, fs::File, io::{self, BufRead, BufReader}};

type R<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    file: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn get_args() -> R<Config> {
    let matches = App::new("rwc")
        .version("0.1.0")
        .author("Mark Vien <iluvshinonomenano@waifu.club")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input file(s)")
                .default_value("-")
        )
        .arg(
            Arg::with_name("line")
                .short("l")    
                .long("line")
                .help("Count lines")
        )
        .arg(
            Arg::with_name("word")
                .short("w")    
                .long("word")
                .help("Count words"),
        )
        .arg(
            Arg::with_name("byte")
                .short("c")    
                .long("byte")
                .help("Count bytes")
        )
        .arg(
            Arg::with_name("char")
                .short("m")
                .long("char")
                .help("Count characters")
        )
        .get_matches();

    let mut lines = matches.is_present("line");
    let mut words = matches.is_present("word");
    let mut bytes = matches.is_present("byte");
    let chars = matches.is_present("char");

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        file: matches.values_of_lossy("files").unwrap(),
        lines,
        words,
        bytes,
        chars,
    })
}

fn open(filename: &str) -> R<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn count(mut file: impl BufRead) -> R<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear()
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

pub fn run(config: Config) -> R<()> {
    for filename in &config.file {
        match open(&filename) {
            Err(err) => eprintln!("Cannot open file: {}", err),
            Ok(file) => {
                if let Ok(info) = count(file) {
                    println!(
                        "{:>8}{:>8}{:>8} {}",
                        info.num_lines,
                        info.num_words,
                        info.num_bytes,
                        filename
                    );
                }
            },
        }
    }

    Ok(())
}
