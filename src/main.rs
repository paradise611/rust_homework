fn main() {
    if let Err(err) = rust_homework::cli::run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}
