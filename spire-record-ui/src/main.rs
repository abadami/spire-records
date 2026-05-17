use std::path::Path;
use spire_record_interpreter::{read_runs_from_directory};
fn main() {
    let runs = read_runs_from_directory(Path::new("spire-record-interpreter/test_files")).expect("TODO: panic message");

    for run in runs {
        println!("{}", run.start_time)
    }
    //println!("Start Time: {}", run.start_time);
    //println!("{}", run.players.get(0).unwrap().character);
}
