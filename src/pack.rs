use std::collections::BTreeMap;

use image::RgbaImage;
use rectangle_pack::{
    contains_smallest_box, pack_rects, volume_heuristic, GroupedRectsToPlace, RectToInsert,
    TargetBin,
};

use crate::{error::PxoError, meta::Tag, sprite::Sprite};

/// Corresponds to a frame in Pixelorama, describing the position of the image data of a [PackedSprite]
#[derive(Clone, Copy, Debug)]
pub struct PackedFrame {
    /// The duration of the frame
    pub duration: f32,
    /// The x position of the frame image inside the spritesheet
    pub x_offset: u32,
    /// The y position of the frame image inside the spritesheet
    pub y_offset: u32,
}

/// A [Sprite] that has been packed, so stores no image data
#[derive(Clone, Debug)]
pub struct PackedSprite {
    /// The width of the sprite in pixels
    pub width: u32,
    /// The height of the sprite in pixels
    pub height: u32,
    /// The fps to use in conjuction with [PackedFrame] duration
    pub fps: u32,
    /// Any tags set in the .pxo, mostly used to mark animations
    pub tags: Vec<Tag>,
    /// Stores the position in the spritesheet of each frame
    pub frames: Vec<PackedFrame>,
}

impl PackedSprite {
    ///  Pack a [Sprite] into a [PackedSprite] and an image
    pub fn pack_sprite(
        sprite: Sprite,
        max_width: u32,
        max_height: u32,
    ) -> Result<(PackedSprite, RgbaImage), PxoError> {
        let (sprites, image) = Self::pack_sprites(vec![sprite], max_width, max_height)?;

        Ok((sprites[0].clone(), image))
    }

    /// Pack a group of [Sprite]s into a group of [PackedSprite]s and an image
    pub fn pack_sprites(
        sprites: Vec<Sprite>,
        max_width: u32,
        max_height: u32,
    ) -> Result<(Vec<PackedSprite>, RgbaImage), PxoError> {
        let mut to_place: GroupedRectsToPlace<(usize, usize), usize> = GroupedRectsToPlace::new();

        for (s, sprite) in (&sprites).into_iter().enumerate() {
            for frame in 0..sprite.images.len() {
                to_place.push_rect(
                    (s, frame),
                    None,
                    RectToInsert::new(sprite.width, sprite.height, 1),
                );
            }
        }

        let mut target_bins = BTreeMap::new();
        target_bins.insert(0usize, TargetBin::new(max_width, max_height, 1));

        let placements = pack_rects(
            &to_place,
            &mut target_bins,
            &volume_heuristic,
            &contains_smallest_box,
        )?;

        let mut width = 0;
        let mut height = 0;
        for (_, (_, location)) in placements.packed_locations() {
            if width < location.x() + location.width() {
                width = location.x() + location.width()
            }
            if height < location.y() + location.height() {
                height = location.y() + location.height()
            }
        }

        let mut image: RgbaImage =
            RgbaImage::from_raw(width, height, vec![0u8; (width * height * 4) as usize])
                .ok_or(PxoError::SpriteConversion)?;

        let mut new_sprites = Vec::new();

        for s in 0..sprites.len() {
            new_sprites.push(PackedSprite {
                width: sprites[s].width,
                height: sprites[s].height,
                fps: sprites[s].fps,
                tags: Vec::new(),
                frames: Vec::new(),
            })
        }

        for ((s, frame), (_, location)) in placements.packed_locations() {
            new_sprites[*s].frames.push(PackedFrame {
                duration: sprites[*s].durations[*frame],
                x_offset: location.x(),
                y_offset: location.y(),
            });

            for x in 0..sprites[*s].width {
                for y in 0..sprites[*s].height {
                    let target = image.get_pixel_mut(location.x() + x, location.y() + y);
                    let source = sprites[*s].images[*frame].get_pixel(x, y);

                    *target = *source;
                }
            }
        }

        for (s, sprite) in (&mut new_sprites).into_iter().enumerate() {
            for tag in &sprites[s].tags {
                sprite.tags.push(tag.clone());
            }
        }

        Ok((new_sprites, image))
    }
}
