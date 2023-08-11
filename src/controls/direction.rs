//! Direction concept representation in struct

//in mod.rs

/// Representation of a direction that is used by any [`crate::rendering::entity::Entity`]
///
/// Directions can be used to process movements of entities.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Direction {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}
