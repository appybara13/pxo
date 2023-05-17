use std::fs::File;

use image::{Pixel, RgbaImage};

use crate::{error::PxoError, meta::Tag, Pxo};

/// A simpler representation of a .pxo file, with layers merged down
pub struct Sprite {
    /// The width of the sprite in pixels
    pub width: u32,
    /// The height of the sprite in pixels
    pub height: u32,
    /// The fps to use in conjuction with frame duration
    pub fps: u32,
    /// Any tags set in the .pxo, mostly used to mark animations
    pub tags: Vec<Tag>,
    /// The duration of each frame in seconds, the size should be the same for images
    pub durations: Vec<f32>,
    /// An image for each frame
    pub images: Vec<RgbaImage>,
}

/// Controls the way in which [Pxo] layers are merged down into a [Sprite]
#[derive(Clone, Copy)]
pub struct SpriteOptions {
    /// Include all layers, including ones that are invisible in Pixelorama `default = false`
    pub ignore_layer_visibility: bool,
    /// Ignore cel opacity as set in Pixelorama `default = false`
    pub ignore_cel_opacity: bool,
}

impl Default for SpriteOptions {
    fn default() -> Self {
        SpriteOptions {
            ignore_layer_visibility: false,
            ignore_cel_opacity: false,
        }
    }
}

impl Sprite {
    /// Load a [Sprite] from a file
    pub fn load(file: File, options: SpriteOptions) -> Result<Sprite, PxoError> {
        let pxo = Pxo::load(file)?;

        Sprite::from(pxo, options)
    }

    /// Convert a [Pxo] to a [Sprite]
    pub fn from(pxo: Pxo, options: SpriteOptions) -> Result<Sprite, PxoError> {
        let mut durations = Vec::new();
        let mut images = Vec::new();

        for (i, frame) in (&pxo.meta.frames).into_iter().enumerate() {
            images.push(merge_frame(i, &pxo, options)?);
            durations.push(frame.duration);
        }

        let sprite = Sprite {
            width: pxo.meta.width,
            height: pxo.meta.height,
            fps: pxo.meta.height,
            tags: pxo.meta.tags,
            durations,
            images,
        };

        Ok(sprite)
    }
}

fn merge_frame(
    frame_index: usize,
    pxo: &Pxo,
    options: SpriteOptions,
) -> Result<RgbaImage, PxoError> {
    let frame = &pxo.meta.frames[frame_index];

    let mut image: RgbaImage = RgbaImage::from_raw(
        pxo.meta.width,
        pxo.meta.height,
        vec![0u8; (pxo.meta.width * pxo.meta.height * 4) as usize],
    )
    .ok_or(PxoError::SpriteConversion)?;

    let mut visible_layers = (&pxo.meta.layers)
        .into_iter()
        .enumerate()
        .filter_map(|(i, l)| if l.visible { Some(i) } else { None });

    for (layer, cel) in (&frame.cels).into_iter().enumerate() {
        let use_cel = options.ignore_layer_visibility || visible_layers.any(|l| l == layer);

        let cel_alpha_multiplier = if options.ignore_cel_opacity {
            1f32
        } else {
            cel.opacity
        };

        if use_cel {
            image
                .pixels_mut()
                .zip(pxo.images[cel.image_index].pixels())
                .for_each(|(target, source)| {
                    target.blend(
                        &source.map_with_alpha(|c| c, |a| (cel_alpha_multiplier * a as f32) as u8),
                    )
                });
        }
    }

    Ok(image)
}
