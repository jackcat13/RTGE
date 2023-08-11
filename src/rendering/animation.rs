//! Animation of sprites concept representation in struct

//in mod.rs

use serde::{Deserialize, Serialize};

use super::frame::Frame;

/// Representation of an animation that is used by any [`crate::rendering::sprite::Sprite`] optionally.
///
/// Animations can be used to represent multiple [`crate::rendering::frame::Frame`] in a certain order to create an animation.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Animation {
    pub name: String,
    pub frames: Vec<Frame>,
}
