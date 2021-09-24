use std::io::prelude::*;



use flate2::write::ZlibEncoder;
use flate2::bufread::ZlibDecoder;

mod types;
mod parsers;

use nom::IResult;
use nom::Finish;




fn main() -> Result<(), anyhow::Error> {

    let mut pray_data: Vec<u8> = Vec::new();
    std::io::stdin().read_to_end(&mut pray_data)?;

    let (_remainder, chunks) = parsers::parse_pray(&pray_data).finish().map_err(|e| { nom::error::Error {input: format!("{:#?}", e.input), code: e.code}})?;

    print!("PRAY");
    for chunk in chunks {
        let out = chunk.serialize()?;
        std::io::stdout().write_all(&out)?;
    }

    Ok(())
}
