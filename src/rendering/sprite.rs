//! Provides sprites representation and loading helpers.

//in mod.rs

use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::from_str;

use super::animation::Animation;

/// Sprite concept representation in struct. Represents the sprite (animated or not) of an [`Entity`]. If the sprite is colored, Ansi values (Vec of u8) must be used (See 256 colors - [cheat sheet](https://www.ditig.com/256-colors-cheat-sheet) for more info.).
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Sprite {
    pub name: String,
    pub pixels: Vec<Vec<char>>,
    pub animations: Option<Vec<Animation>>,
    pub animation_index: Option<usize>,
    pub colors: Option<Vec<Vec<u8>>>,
}

/// Sprite loading helper to create the sprite 2D collection based on a JSON file. It can be static sprite or animated sprite.
///
/// # Examples
///
/// ## Load a sprite from json file
///
/// ```ignore
/// let sprite = load_sprite("./src/rendering/tests/simple_sprite.json".to_string());
/// ```
#[allow(dead_code)]
pub fn load_sprite(file_path: String) -> Sprite {
    let file_raw =
        fs::read_to_string(file_path.clone()).expect("Should have been able to read the file");
    let mut sprite: Sprite = from_str(&file_raw).unwrap_or_else(|error| {
        panic!(
            "Failed to load sprite from file {} with following error : {}",
            file_path, error
        )
    });
    sprite.animation_index = Some(0);
    sprite
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_sprite_should_create_sprite_struct() {
        let sprite = load_sprite("./src/rendering/tests/simple_sprite.json".to_string());
        assert_eq!(sprite.name, "Simple sprite");
        assert_eq!(
            sprite.pixels,
            vec![
                [
                    '╮', '╭', '┻', '┻', '╮', '╭', '┻', '┻', '╮', '╭', '▕', '╮', '╲', ' ', ' '
                ],
                [
                    '▕', '╯', '┃', '╭', '╮', '┃', '┃', '╭', '╮', '┃', '╰', '▕', '╯', '╭', '▏'
                ],
                [
                    '▕', '╭', '┻', '┻', '┻', '┛', '┗', '┻', '┻', '┛', ' ', '╰', '▏', ' ', ' '
                ],
                [
                    '▕', '╰', '━', '━', '━', '┓', '┈', '┈', '┈', '╭', '╮', '▕', '╭', '╮', '▏'
                ],
                [
                    '▕', '╭', '╮', '╰', '┳', '┳', '┳', '┳', '╯', '╰', '╯', '▕', '╰', '╯', '▏'
                ],
                [
                    '▕', '╰', '╯', '┈', '┗', '┛', '┗', '┛', '┈', '╭', '╮', '▕', '╮', '┈', '▏'
                ]
            ]
        );
    }

    #[test]
    #[should_panic]
    fn load_sprite_should_panic_when_wrong_file() {
        load_sprite("doesNotExist.js".to_string());
    }

    #[test]
    #[should_panic]
    fn load_sprite_should_panic_when_wrong_json_in_file() {
        load_sprite("./src/rendering/tests/wrong_json.json".to_string());
    }
}
