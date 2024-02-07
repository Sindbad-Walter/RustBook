use {
    chapter_12::Config,
    std::{env, process},
};
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument : {err}");
        process::exit(1);
    });

    if let Err(e) = chapter_12::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
