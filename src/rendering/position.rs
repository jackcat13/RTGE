//! Position concept representation in struct to represent [`Entity`] position.

//in mod.rs

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}
