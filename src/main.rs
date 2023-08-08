use std::{thread::sleep, time};

use rendering::{
    entity::{print_sprites, Entity},
    position::Position,
    sprite::load_sprite,
};

mod rendering;

//Main only useful to do some manual tests in the library
fn main() {
    let mut entities = vec![
        Entity {
            name: "bob".to_string(),
            sprite: load_sprite("./manualTests/simple_sprite.json".to_string()),
            position: Position { x: 50, y: 0 },
        },
        Entity {
            name: "bob2".to_string(),
            sprite: load_sprite("./manualTests/simple_sprite.json".to_string()),
            position: Position { x: 100, y: 10 },
        },
    ];
    loop {
        print_sprites(&entities);

        entities.get_mut(0).unwrap().position.x += 1;

        sleep(time::Duration::from_millis(10));
    }
}
