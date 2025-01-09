extern crate colored;
extern crate flate2;

use colored::Colorize;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant; // to show how long the compression took

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: <source> <destination>");
        return;
    }

    // Open the file to compress
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());

    // Create the output file
    let output = File::create(args().nth(2).unwrap()).unwrap();

    // Create the GzEncoder
    let mut encoder = GzEncoder::new(output, Compression::default());

    // Start the timer
    let start = Instant::now();

    // Compress the file
    copy(&mut input, &mut encoder).unwrap();

    let output = encoder.finish().unwrap();

    // Print the length of the source and destination files
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );

    // The destination file is the output file
    println!("Destination len: {:?}", output.metadata().unwrap().len());

    // Print how long the compression took
    println!(
        "Compression took: {}",
        format!("{:?}", start.elapsed()).green()
    );
}
