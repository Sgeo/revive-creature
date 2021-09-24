use std::io::prelude::*;
use std::convert::TryInto;

use nom::IResult;
use nom::bytes::complete::{tag, take};
use nom::number::complete::le_u32;
use nom::sequence::preceded;
use nom::multi::many0;

use crate::types::PrayChunk;

pub const PRAY_HEADER: &'static [u8] = b"PRAY";
pub const CREATURESARCHIVE_HEADER: &'static [u8] = b"Creatures Evolution Engine - Archived information file. zLib 1.13 compressed.\x1a\x04";

use flate2::write::ZlibEncoder;
use flate2::bufread::ZlibDecoder;

fn parse_praychunk(input: &[u8]) -> nom::IResult<&[u8], PrayChunk> {
    let (input, r#type): (&[u8], &[u8]) = take(4usize)(input)?;
    let (input, name): (&[u8], &[u8]) = take(128usize)(input)?;
    let (input, compressed_size): (&[u8], u32) = le_u32(input)?;
    let (input, uncompressed_size): (&[u8], u32) = le_u32(input)?;
    let (input, flags): (&[u8], u32) = le_u32(input)?;
    let (input, maybe_compressed): (&[u8], &[u8]) = take(compressed_size)(input)?;
    let mut data = Vec::with_capacity(uncompressed_size as usize);
    if flags & 0x1 == 1 {
        let mut decoder = ZlibDecoder::new(maybe_compressed);
        decoder.read_to_end(&mut data).expect("Unable to compress data!"); // TODO: Replace with using the result
    } else {
        data.write_all(maybe_compressed).expect("Unable to write data!"); // TODO: Replace with using the Result
    }
    Ok(
        (input, PrayChunk {
            r#type: r#type.try_into().unwrap(),
            name: name.try_into().unwrap(),
            data: data
        })
    )
}

pub fn parse_pray<'a>(input: &'a [u8]) -> nom::IResult<&'a [u8], Vec<PrayChunk>> {
    preceded(tag(PRAY_HEADER), many0(parse_praychunk))(input)
}