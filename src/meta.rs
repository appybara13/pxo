use serde_json::{Map, Value};

use crate::error::PxoError;

/// Contains the information from .pxo json data
#[derive(Clone, Debug)]
pub struct Meta {
    /// The width of all of the images in pixels
    pub width: u32,
    /// The height of all of the images in pixels
    pub height: u32,
    /// The fps to use in conjuction with [Frame] duration
    pub fps: f32,
    /// The .pxo frames, in order. Each [Frame] should contain a number of
    /// [Cel]s equal to the number of layers
    pub frames: Vec<Frame>,
    /// The .pxo layers, with the first [Layer] being the lowest one
    pub layers: Vec<Layer>,
    /// Any tags set in the .pxo, mostly used to mark animations
    pub tags: Vec<Tag>,
}

/// A tag from a Pixelorama
#[derive(Clone, Debug)]
pub struct Tag {
    /// The name of the animation
    pub name: String,
    /// The first frame of the animation
    pub from: usize,
    /// The last frame of the animation, inclusive
    pub to: usize,
}

/// A layer from Pixelorama. This actually just contains the layer name and
/// visibility as the [Cel]s  for each layer are stored in [Frame]s
#[derive(Clone, Debug)]
pub struct Layer {
    /// The name of the layer
    pub name: String,
    /// The visibility of the layer
    pub visible: bool,
}

/// A frame from Pixelorama
#[derive(Clone, Debug)]
pub struct Frame {
    /// The duration of the frame in seconds
    pub duration: f32,
    /// The [Cel]s for this frame, in same order as the layers in [Meta]
    pub cels: Vec<Cel>,
}

/// A cel exists for every combination of Pixelorama layer and frame
#[derive(Clone, Copy, Debug)]
pub struct Cel {
    /// Cel opacity can be set per cel in Pixelorama
    pub opacity: f32,
    /// The index of the corresponding image in a [Pxo]
    pub image_index: usize,
}

pub(crate) fn parse_meta(json: String) -> Result<Meta, PxoError> {
    let json: Value = serde_json::from_str(json.as_str())?;
    let json = json.expect_object()?;

    let width = json.expect("size_x")?.expect_u64()? as u32;
    let height = json.expect("size_y")?.expect_u64()? as u32;

    let fps = json.expect("fps")?.expect_f64()? as f32;

    let mut meta = Meta {
        width,
        height,
        fps,
        frames: Vec::new(),
        layers: Vec::new(),
        tags: Vec::new(),
    };

    let frames = json.expect("frames")?.expect_array()?;

    let mut image_index = 0;
    for (i, frame) in frames.iter().enumerate() {
        let frame = frame.expect_object()?;

        let duration = frame.expect("duration")?.expect_f64()? as f32;

        meta.frames.push(Frame {
            cels: Vec::new(),
            duration,
        });

        let cels = frame.expect("cels")?.expect_array()?;

        for cel in cels {
            let opacity = cel.expect_object()?.expect("opacity")?.expect_f64()? as f32;
            meta.frames[i].cels.push(Cel {
                image_index,
                opacity,
            });
            image_index += 1;
        }
    }

    let layers = json.expect("layers")?.expect_array()?;

    for layer in layers {
        let layer = layer.expect_object()?;

        let name = layer.expect("name")?.expect_string()?;
        let visible = layer.expect("visible")?.expect_bool()?;

        meta.layers.push(Layer { name, visible })
    }

    let tags = json.expect("tags")?.expect_array()?;

    for tag in tags {
        let tag = tag.expect_object()?;

        let from = tag.expect("from")?.expect_u64()? as usize;
        let to = tag.expect("to")?.expect_u64()? as usize;
        let name = tag.expect("name")?.expect_string()?;

        meta.tags.push(Tag { name, from, to })
    }

    Ok(meta)
}

trait MapExt {
    fn expect(&self, key: &str) -> Result<&Value, PxoError>;
}

impl MapExt for Map<String, Value> {
    fn expect(&self, key: &str) -> Result<&Value, PxoError> {
        self.get(key).ok_or(PxoError::UnexpectedJson)
    }
}

trait ValueExt {
    fn expect_object(&self) -> Result<&Map<String, Value>, PxoError>;
    fn expect_array(&self) -> Result<&Vec<Value>, PxoError>;
    fn expect_u64(&self) -> Result<u64, PxoError>;
    fn expect_f64(&self) -> Result<f64, PxoError>;
    fn expect_string(&self) -> Result<String, PxoError>;
    fn expect_bool(&self) -> Result<bool, PxoError>;
}

impl ValueExt for Value {
    fn expect_object(&self) -> Result<&Map<String, Value>, PxoError> {
        self.as_object().ok_or(PxoError::UnexpectedJson)
    }
    fn expect_array(&self) -> Result<&Vec<Value>, PxoError> {
        self.as_array().ok_or(PxoError::UnexpectedJson)
    }
    fn expect_u64(&self) -> Result<u64, PxoError> {
        self.as_u64().ok_or(PxoError::UnexpectedJson)
    }
    fn expect_f64(&self) -> Result<f64, PxoError> {
        self.as_f64().ok_or(PxoError::UnexpectedJson)
    }
    fn expect_string(&self) -> Result<String, PxoError> {
        Ok(self.as_str().ok_or(PxoError::UnexpectedJson)?.to_owned())
    }
    fn expect_bool(&self) -> Result<bool, PxoError> {
        self.as_bool().ok_or(PxoError::UnexpectedJson)
    }
}
