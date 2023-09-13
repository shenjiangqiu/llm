use std::{fs::File, io::BufReader};

fn main() {
    println!("Hello, world!");

    let value: rust_utils::InputRecord =
        bincode::deserialize_from(BufReader::new(File::open("input_record.bin").unwrap())).unwrap();
    for (index, (input_token, records)) in value.records.into_iter().enumerate() {
        println!("{}-{}", index, input_token);
        records.print_all_shapes();
    }
}
