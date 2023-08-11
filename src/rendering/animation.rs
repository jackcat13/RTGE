use serde::{Deserialize, Serialize};

use super::frame::Frame;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Animation {
    pub name: String,
    pub frames: Vec<Frame>,
}
