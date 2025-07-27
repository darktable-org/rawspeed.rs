use rawspeed_metadata_camerasxml_parser::camerasxml_parser;
use std::env;
use std::fs;

#[expect(clippy::print_stdout, clippy::use_debug)]
fn main() -> Result<(), Box<dyn core::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let file_path = match args.get(2) {
        Some(p) => p,
        None => "/home/lebedevri/rawspeed/data/cameras.xml",
    };

    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let repr = camerasxml_parser::parse_str(&contents)?;
    println!("{repr:#?}");
    Ok(())
}
