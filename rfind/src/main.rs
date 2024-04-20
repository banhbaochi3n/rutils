#![allow(warnings, unused)]
mod lib;

fn main() {
    if let Err(e) = rfind::get_args().and_then(rfind::run) {
        eprintln!("lol");
        std::process::exit(1);
    }
}
