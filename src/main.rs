extern crate byteorder;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use byteorder::{BigEndian, WriteBytesExt};

fn main() {
    // Get the CLI arguments
    // TODO: Figure out a less ugly way to do this
    let args: Vec<String> = env::args().collect();

    // TODO: read_file
    // TODO: decision logic on read or write
    write_file(&args[1], &args[2]);
}

fn write_file(device: &String, store: &String) {

    // Open our files for reading/writing
    let mut out_file = File::create(device).expect(
        "Output file could not be created");
    let mut in_file = File::open(store).expect("Input file not found");

    // Read the input file
    let mut data = String::new();
    in_file.read_to_string(&mut data).expect("Input file could not be read");

    // Write our data to the device
    let _ = out_file.write_u64::<BigEndian>(data.len() as u64);
    let _ = out_file.write_all(data.as_bytes()).expect("Could not write to output file");
}
