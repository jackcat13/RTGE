use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Frame {
    pub pixels: Vec<Vec<char>>,
}
