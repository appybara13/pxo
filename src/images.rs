use image::{ImageBuffer, RgbaImage};

use crate::{error::PxoError, meta::Meta, read_ext::ReadExt};

pub(crate) fn load_images<R: ReadExt>(
    meta: &Meta,
    reader: &mut R,
) -> Result<Vec<ImageBuffer<image::Rgba<u8>, Vec<u8>>>, PxoError> {
    let mut images: Vec<RgbaImage> = Vec::new();

    for frame in &meta.frames {
        for _cel in &frame.cels {
            let mut data = vec![0u8; (4 * meta.width * meta.height) as usize];
            reader.read_exact(data.as_mut_slice())?;
            images.push(
                ImageBuffer::from_raw(meta.width, meta.height, data).ok_or(PxoError::ReadImage)?,
            );
        }
    }
    Ok(images)
}
