//! Frame of sprite concept representation in struct

//in mod.rs

use serde::{Deserialize, Serialize};

/// Representation of a frame concept. May represents a static [`Sprite`] or one frame of animation of a [`Sprite`]. If the sprite is colored, Ansi values (Vec of u8) must be used (See 256 colors - [cheat sheet](https://www.ditig.com/256-colors-cheat-sheet) for more info.).
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Frame {
    pub pixels: Vec<Vec<char>>,
    pub colors: Option<Vec<Vec<u8>>>,
}
