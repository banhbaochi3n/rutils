#![allow(warnings, unused, dead_code)]
use clap::{Arg, App};
use walkdir::{WalkDir, DirEntry};
use std::error::Error;
use regex::Regex;

type R<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    File,
    Dir,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> R<Config> {
    let matches = App::new("rfind")
        .version("0.1.0")
        .author("Mark Vien <iluvshinonome@waifu.club>")
        .about("Rust find")
        .arg(
            Arg::with_name("paths")
                .value_name("PATH")
                .help("Path to search for stuff")
                .default_value(".")
                .multiple(true),
        )
        .arg(
            Arg::with_name("names")
                .value_name("NAME")    
                .short("n")
                .long("name")
                .help("Name of entry to search")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("types")
                .value_name("TYPE")    
                .short("t")
                .long("type")
                .help("Type of entry")
                .takes_value(true)
                .multiple(true)
                .possible_values(&["f", "d", "l"]),
        )
        .get_matches();

    let names = matches
        .values_of_lossy("names")
        .map(|vals| {
            vals.into_iter()
                .map(|name| {
                    Regex::new(&name)
                        .map_err(|_| format!("Invalid --name \"{}\"", name))
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .unwrap_or_default();

    let entry_types = matches
        .values_of_lossy("types")
        .map(|val| {
            val.iter()
                .map(|t| match t.as_str() {
                    "f" => EntryType::File,
                    "d" => EntryType::Dir,
                    "l" => EntryType::Link,
                    _ => unreachable!("Invalid type"),
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(Config {
        paths: matches.values_of_lossy("paths").unwrap(),
        names,
        entry_types,
    })
}

pub fn run(config: Config) -> R<()> {
    let type_filter = |entry: &DirEntry| {
        config.entry_types.is_empty() || config.entry_types.iter().any(|entry_type| {
            match entry_type {
                EntryType::File => entry.file_type().is_file(),
                EntryType::Dir => entry.file_type().is_dir(),
                EntryType::Link => entry.file_type().is_symlink(),
            }
        })     
    };

    let name_filter = |entry: &DirEntry| {
        config.names.is_empty() || config.names.iter().any(|name| {
            name.is_match(&entry.file_name().to_string_lossy())
        })
    };

    for path in config.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprint!("{}", e);
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();

        println!("{}", entries.join("\n"));
    }
    Ok(())
}
