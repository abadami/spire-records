use spire_record_interpreter::read_test_file;

fn main() {
    let run = read_test_file().expect("TODO: panic message");

    println!("{}", run.players.get(0).unwrap().character);
}
