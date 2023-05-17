use std::io::Seek;

use crate::{error::PxoError, read_ext::ReadExt};

const GODOT_MAGIC_COMPRESED: &str = "GCPF";
const GODOT_ZLTF_COMPRESSION_MODE: u32 = 2;

pub(crate) fn decompress<R: ReadExt + Seek>(source: &mut R) -> Result<Vec<u8>, PxoError> {
    check_head(source)?;

    let block_size = source.read_u32()?;
    if block_size == 0 {
        return Err(PxoError::ZeroBlockSize);
    }
    let total_size = source.read_u32()?;
    let block_count = (total_size / block_size + 1) as usize;

    let mut block_sizes = vec![0usize; block_count];
    for block in 0..block_count {
        block_sizes[block] = source.read_u32()? as usize;
    }

    let mut decompressed = Vec::new();

    for block in 0..block_count {
        let mut compressed_block = vec![0u8; block_sizes[block]];
        source.read_exact(&mut compressed_block)?;
        let mut decompressed_block = zstd::decode_all(compressed_block.as_slice())?;
        decompressed.append(&mut decompressed_block);
    }

    Ok(decompressed)
}

fn check_head<R: ReadExt>(reader: &mut R) -> Result<(), PxoError> {
    let magic = reader.read_string(4)?;
    if magic != GODOT_MAGIC_COMPRESED {
        return Err(PxoError::UnexpectedMagic(
            GODOT_MAGIC_COMPRESED.to_string(),
            magic,
        ));
    }

    let compression_mode = reader.read_u32()?;
    if compression_mode != GODOT_ZLTF_COMPRESSION_MODE {
        return Err(PxoError::UnexpectedCompressionMode(compression_mode));
    }

    Ok(())
}
