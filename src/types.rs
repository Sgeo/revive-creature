use std::io::prelude::*;



use flate2::write::ZlibEncoder;

#[derive(Debug, Clone)]
pub enum Engine {
    C3,
    DS
}

#[derive(Debug, Clone)]
pub struct PrayChunk {
    pub r#type: [u8; 4],
    pub name: [u8; 128],
    pub data: Vec<u8>
}

impl PrayChunk {
    pub fn serialize(&self) -> Result<Vec<u8>, failure::Error> {
        let mut output: Vec<u8> = Vec::new();
        output.write_all(&self.r#type)?;
        output.write_all(&self.name)?;
        let uncompressed_size = self.data.len() as u32;
        let mut encoder = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(&self.data)?;
        let encoded = encoder.finish()?;
        let compressed_size = encoded.len() as u32;
        output.write_all(&compressed_size.to_le_bytes())?;
        output.write_all(&uncompressed_size.to_le_bytes())?;
        output.write_all(&1u32.to_le_bytes())?;
        output.write_all(&encoded)?;
        Ok(output)
    }
}