use std::io::prelude::*;



use flate2::write::ZlibEncoder;
use flate2::bufread::ZlibDecoder;

mod types;
mod parsers;




fn main() -> Result<(), failure::Error> {
    println!("Hello, world!");

    parsers::parse_pray(&[])?;

    Ok(())
}
