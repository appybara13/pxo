# Pxo

Utilities for loading [Pixelorama](https://github.com/Orama-Interactive/Pixelorama) files. Only supports recent
Pixelorama versions.

By default, a file will be read into a `Pxo` file, and each `Cel` (one exists for every frame and layer)
is stored as a separate image. This method of loading is recommended only when separate layers are needed,
such as if different, swappable items of clothing are stored on different layers.

The `sprite` feature, enabled by default, allows loading a file into a more useable `Sprite`. Layers are
merged so that there is a single image per frame. The rest of the `Sprite` describe how to animate it.

The `pack` feature allows packing loaded files. The images of a `Sprite` are packed into a single spritesheet
image and a `PackedSprite` is used to hold the sprite data, with each frame represented by a `PackedFrame`. The
image is returned separately from the `PackedSprite` because it is also possible to pack multiple files into the
same image.

# Basic Usage

```rust
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
