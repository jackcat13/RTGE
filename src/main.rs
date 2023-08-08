use std::{io, time::Duration};

use crossterm::{
    event::{Event, EventStream, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use futures::{FutureExt, StreamExt};
use futures_timer::Delay;
use rendering::{
    entity::{print_sprites_centered_on, Entity},
    position::Position,
    sprite::load_sprite,
};
use tokio::select;

mod rendering;

const TERM_SIZE_X: u16 = 20_000;
const TERM_SIZE_Y: u16 = 20_000;

//Main only useful to do some manual tests in the library
#[tokio::main]
async fn main() -> io::Result<()> {
    let _ = crossterm::terminal::SetSize(TERM_SIZE_X, TERM_SIZE_Y);
    enable_raw_mode().expect("Failed enabling raw mode");
    game().await;
    disable_raw_mode()
}

async fn game() {
    let mut bob = Entity {
        name: "bob".to_string(),
        sprite: load_sprite("./manualTests/bob.json".to_string()),
        position: Position {
            x: TERM_SIZE_X / 2,
            y: TERM_SIZE_Y / 2,
        },
    };
    let enemies = vec![Entity {
        name: "enemy".to_string(),
        sprite: load_sprite("./manualTests/enemy.json".to_string()),
        position: Position {
            x: (TERM_SIZE_X / 2) + 20,
            y: (TERM_SIZE_Y / 2) + 20,
        },
    }];
    let mut reader = EventStream::new();
    loop {
        print_sprites_centered_on(&bob, &enemies);

        let delay = Delay::new(Duration::from_millis(1_000)).fuse();
        let event = reader.next().fuse();
        select! {
            _ = delay => { println!(".\r"); },
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        if event == Event::Key(KeyCode::Char('z').into()) {
                            bob.position.y -= 1;
                        }
                        if event == Event::Key(KeyCode::Char('s').into()) {
                            bob.position.y += 1;
                        }
                        if event == Event::Key(KeyCode::Char('q').into()) {
                            bob.position.x -= 1;
                        }
                        if event == Event::Key(KeyCode::Char('d').into()) {
                            bob.position.x += 1;
                        }
                        if event == Event::Key(KeyCode::Esc.into()) {
                            break;
                        }
                    }
                    Some(Err(_)) => {},
                    None => {},
                }
            }
        };
    }
}
