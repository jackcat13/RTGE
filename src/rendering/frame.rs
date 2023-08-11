//! Frame of sprite concept representation in struct

//in mod.rs

use serde::{Deserialize, Serialize};

/// Representation of a frame concept. May represents a static [`Sprite`] or one frame of animation of a [`Sprite`].
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Frame {
    pub pixels: Vec<Vec<char>>,
}
