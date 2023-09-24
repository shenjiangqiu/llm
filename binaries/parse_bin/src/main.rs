use std::{fs::File, io::BufReader, path::PathBuf};

use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    input_file: PathBuf,
}

fn main() {
    let Cli { input_file } = Cli::parse();
    let value: rust_utils::InputRecord =
        bincode::deserialize_from(BufReader::new(File::open(input_file).unwrap())).unwrap();
    for (index, (input_token, records)) in value.records.into_iter().enumerate() {
        println!("{}-{}", index, input_token);
        records.print_all_shapes();
    }
}
