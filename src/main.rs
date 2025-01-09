use colored::Colorize;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::{self, copy};
use std::time::Instant;

fn main() -> io::Result<()> {
    // Collect the command line arguments
    let args: Vec<String> = args().collect();

    // Check if the user provided the input and output file paths
    if args.len() < 3 {
        eprintln!(
            "Usage: {} <input file> <output file>",
            args.get(0).unwrap_or(&"compress".to_string())
        );
        std::process::exit(1);
    }

    // Get the input and output file paths
    let input_file_path = &args[1];
    let output_file_path = &args[2];

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
