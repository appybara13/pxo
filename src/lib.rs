#![deny(missing_docs, unsafe_code)]
/*!
Utilities for loading [Pixelorama](https://github.com/Orama-Interactive/Pixelorama) files. Only supports recent
Pixelorama versions.

By default, a file will be read into a [Pxo] file, and each [Cel] (one exists for every frame and layer)
is stored as a separate image. This method of loading is recommended only when separate layers are needed,
such as if different, swappable items of clothing are stored on different layers.

The `sprite` feature, enabled by default, allows loading a file into a more useable [Sprite]. Layers are
merged so that there is a single image per frame. The rest of the [Sprite] describe how to animate it.

The `pack` feature allows packing loaded files. The images of a [Sprite] are packed into a single spritesheet
image and a [PackedSprite] is used to hold the sprite data, with each frame represented by a [PackedFrame]. The
image is returned separately from the [PackedSprite] because it is also possible to pack multiple files into the
same image.

# Basic Usage

```
# use std::fs::File;
let file = File::open("path/to/your/sprite.pxo")?;

// Load a .pxo and convert it into a sprite
let pxo = pxo::Pxo::load(file)?;
let sprite = pxo::Sprite::from(pxo, SpriteOptions::Default())?;

// Alternatively, a .pxo can be directly loaded as a sprite
let sprite = pxo::Sprite::load(file, SpriteOptions::Default())?;

// Packing a single sprite
// An error will be returned if the sprite cannot be packed into a 2048x2048 image
let (packed, image) = pxo::PackedSprite::pack_sprite(sprite, 2048, 2048)?;

// Packing two sprites, loaded from two different files, into the same image
sprite_a = pxo::Sprite::load(file_a, SpriteOptions::Default())?;
sprite_b = pxo::Sprite::load(file_b, SpriteOptions::Default())?;

let (packed, image) = pxo::PackedSprite::pack_sprites(&[sprite_a, sprite_b], 2048, 2048)?;
```
*/

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use decompress::decompress;
use image::RgbaImage;
use images::load_images;
use meta::parse_meta;
use read_ext::ReadExt;

mod decompress;
mod error;
mod images;
mod meta;
mod read_ext;

#[cfg(feature = "sprite")]
mod sprite;

#[cfg(feature = "pack")]
mod pack;
#[cfg(feature = "pack")]
pub use pack::PackedFrame;
#[cfg(feature = "pack")]
pub use pack::PackedSprite;

impl ReadExt for BufReader<File> {}
impl ReadExt for BufReader<&[u8]> {}

#[cfg(feature = "sprite")]
pub use sprite::Sprite;
#[cfg(feature = "sprite")]
pub use sprite::SpriteOptions;

pub use error::PxoError;
pub use meta::Cel;
pub use meta::Frame;
pub use meta::Layer;
pub use meta::Meta;
pub use meta::Tag;

/// The closest representation of a .pxo file
#[derive(Clone, Debug)]
pub struct Pxo {
    /// Information from the .pxo json metadata
    pub meta: Meta,
    /// All of the images in the .pxo, one for every [Cel]
    pub images: Vec<RgbaImage>,
}

impl Pxo {
    /// Loads a [Pxo] from a file
    pub fn load(file: File) -> Result<Pxo, PxoError> {
        let mut reader = BufReader::new(file);
        let decompressed = decompress(&mut reader)?;
        let mut reader = BufReader::new(decompressed.as_slice());

        let mut json = String::new();
        reader.read_line(&mut json)?;

        let meta = parse_meta(json)?;
        let images = load_images(&meta, &mut reader)?;

        Ok(Pxo { meta, images })
    }
}
