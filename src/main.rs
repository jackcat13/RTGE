use std::io::{self};

use controls::{
    direction::Direction,
    inputs::{process_inputs, process_key_press_event},
};
use crossterm::event::{Event, KeyCode};
use rendering::{
    entity::{print_sprites_centered_on, Entity},
    position::Position,
    sprite::load_sprite,
};

use crate::rendering::entity::{move_entities, move_entity};

mod controls;
mod rendering;

const TERM_SIZE_X: u16 = 20_000;
const TERM_SIZE_Y: u16 = 20_000;
const UP: char = 'z';
const DOWN: char = 's';
const LEFT: char = 'q';
const RIGHT: char = 'd';

//Main only useful to do some manual tests in the library
#[tokio::main]
async fn main() -> io::Result<()> {
    let _ = crossterm::terminal::SetSize(TERM_SIZE_X, TERM_SIZE_Y);
    game().await
}

async fn game() -> io::Result<()> {
    let mut camel = Entity {
        name: "bob".to_string(),
        sprite: load_sprite("./manualTests/animated_camel.json".to_string()),
        position: Position {
            x: TERM_SIZE_X / 2,
            y: TERM_SIZE_Y / 2,
        },
        direction: Direction {
            up: false,
            down: false,
            left: false,
            right: false,
        },
        speed: 2,
        animation_name: Option::Some("walking".to_string()),
    };
    let mut enemies = vec![Entity {
        name: "enemy".to_string(),
        sprite: load_sprite("./manualTests/enemy.json".to_string()),
        position: Position {
            x: (TERM_SIZE_X / 2) + 20,
            y: (TERM_SIZE_Y / 2) + 20,
        },
        direction: Direction {
            up: false,
            down: false,
            left: false,
            right: false,
        },
        speed: 1,
        animation_name: Option::None,
    }];

    loop {
        print_sprites_centered_on(&mut camel, &mut enemies);

        let inputs_rules = |event: Event| -> Result<(), String> {
            camel.direction.up = event == process_key_press_event(UP);
            camel.direction.down = event == process_key_press_event(DOWN);
            camel.direction.left = event == process_key_press_event(LEFT);
            camel.direction.right = event == process_key_press_event(RIGHT);
            if event == Event::Key(KeyCode::Esc.into()) {
                return Err("Escape event".to_string());
            }
            Ok(())
        };

        match process_inputs(inputs_rules).await {
            Ok(_) => {}
            Err(_) => {
                break;
            }
        }

        camel = move_entity(camel);
        enemies = move_entities(enemies);
    }
    Ok(())
}
