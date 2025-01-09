extern crate colored;
extern crate flate2;

use colored::Colorize;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::{self, copy};
use std::time::Instant;

fn main() -> io::Result<()> {
    // Check if the user provided the input and output file paths
    if args().len() < 3 {
        eprintln!(
            "Usage: {} <input file> <output file>",
            args().nth(0).unwrap()
        );
        std::process::exit(1);
    }

    // Get the input and output file paths
    let input_file_path = args().nth(1).expect("Missing input file");
    let output_file_path = args().nth(2).expect("Missing output file");

    // Open the input file
    let mut input_file = File::open(&input_file_path)?;

    // Create the output file
    let output_file = File::create(&output_file_path)?;

    // Create a GzEncoder
    let mut encoder = GzEncoder::new(output_file, Compression::best());

    // Start the timer
    let start = Instant::now();

    // Compress the file
    copy(&mut input_file, &mut encoder)?;

    // Finish the compression
    let output_file = encoder.finish()?;

    // Print the length of the source and destination files
    println!("Source len: {:?}", input_file.metadata()?.len());

    // The destination file is the output file
    println!("Destination len: {:?}", output_file.metadata()?.len());

    // Print how long the compression took
    println!(
        "Compression took: {}",
        format!("{:?}", start.elapsed()).green()
    );

    Ok(())
}
