use crate::capsule::run;

mod capsule;
mod errors;
mod namespaces;
mod utils;

fn main() {
    match run() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Failed to run capsule: {}", e);
            std::process::exit(1);
        }
    }
}
