use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Sprite {
    pub name: String,
    pub pixels: Vec<Vec<char>>,
}

#[allow(dead_code)]
pub fn load_sprite(file_path: String) -> Sprite {
    let file_raw =
        fs::read_to_string(file_path.clone()).expect("Should have been able to read the file");
    let sprite: Sprite = from_str(&file_raw)
        .unwrap_or_else(|_| panic!("Failed to load sprite from file {}", file_path));
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
