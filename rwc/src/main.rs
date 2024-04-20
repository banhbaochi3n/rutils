mod lib;

fn main() {
    if let Err(err) = rwc::get_args().and_then(rwc::run) {
        eprintln!("LOL");
        std::process::exit(1);
    }
}
