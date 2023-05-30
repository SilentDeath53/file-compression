use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

fn main() -> io::Result<()> {
     let input_file = "input.txt";

    let output_file = "compressed.zip";
    let file = File::create(output_file)?;
    let encoder = GzEncoder::new(file, Compression::default());

    let mut input = File::open(input_file)?;
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer)?;

    let mut encoder = GzEncoder::new(file, Compression::default());
    encoder.write_all(&buffer)?;

    encoder.finish()?;

    println!("File compressed successfully.");

    Ok(())
}
