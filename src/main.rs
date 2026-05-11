use crate::capsule::run;

mod capsule;
mod namespaces;
mod errors;
mod utils;

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to run capsule: {}", e);
        }
    }
}

