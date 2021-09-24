use std::io::prelude::*;



use flate2::write::ZlibEncoder;
use flate2::bufread::ZlibDecoder;

mod types;
mod parsers;

use nom::IResult;
use nom::Finish;
use types::CreaturesArchive;

use anyhow::Context;
use std::convert::TryInto;



// Thanks to ligfx
// for the below, and for extensive research and documentation into Creatures file formats
const LIFE_FACULTY_HEADER: &'static [u8] = b"\x0b\x00\x00\x00LifeFaculty\x04\x00\x00\x00OBST";

// Format of LifeFaculty (derived from LifeFaculty.cpp)
// Bounds listed are [inclusive, exclusive)
// 0x00 - 0x04: Agent reference?
// 0x04 - 0x08: Asleep locus
// 0x08 - 0x0C: Death trigger locus, should probably clear this
// 0x0C - 0x28: 7 floats for aging loci
// 0x28 - 0x2C: Sex
// 0x2C - 0x30: Age
// 0x30 - 0x34: Variant
// 0x34 - 0x38: Age in ticks
// 0x38 - 0x3A: State. 0 = zombie, 1 = alert, 2 = asleep, 3 = dreaming, 4 = unconscious, 5 = DEAD


fn revive_crea(crea: &mut CreaturesArchive) -> Result<(), anyhow::Error> {
    let header_location = memchr::memmem::find(&crea.data, LIFE_FACULTY_HEADER).context("Unable to locate LifeFaculty")?;
    let start = header_location + LIFE_FACULTY_HEADER.len();
    let data = &mut crea.data[start..];
    let state = u16::from_le_bytes(data[0x38..0x3A].try_into().unwrap());
    if state != 5 {
        eprintln!("This creature is not dead! State: {}", state);
    } else {
        data[0x38..0x3A].copy_from_slice(&1u16.to_le_bytes());
    }
    let death_trigger = f32::from_le_bytes(data[0x08..0x0C].try_into().unwrap());
    if death_trigger != 0f32 {
        eprintln!("Clearing death trigger locus!");
        data[0x08..0x0C].copy_from_slice(&0f32.to_le_bytes());
    }

    Ok(())

}




fn main() -> Result<(), anyhow::Error> {

    let mut pray_data: Vec<u8> = Vec::new();
    std::io::stdin().read_to_end(&mut pray_data)?;

    let (_remainder, chunks) = parsers::parse_pray(&pray_data).finish().map_err(|e| { nom::error::Error {input: format!("{:#?}", e.input), code: e.code}})?;

    print!("PRAY");
    for mut chunk in chunks {
        //let out = chunk.serialize()?;
        if &chunk.r#type == b"CREA" {
            let (_remainder, mut crea): (&[u8], CreaturesArchive) = parsers::parse_creaturesarchive(&chunk.data).finish().map_err(|e| { nom::error::Error {input: format!("{:#?}", e.input), code: e.code}})?;
            revive_crea(&mut crea)?;
            chunk.data = crea.serialize()?;
        }
        let out = chunk.serialize()?;
        std::io::stdout().write_all(&out)?;
    }

    Ok(())
}
