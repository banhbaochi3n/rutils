mod lib;

fn main() {
    if let Err(err) = runiq::get_args().and_then(runiq::run) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
