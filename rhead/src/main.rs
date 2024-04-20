mod lib;

fn main() {
    if let Err(e) = rhead::get_args().and_then(rhead::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
