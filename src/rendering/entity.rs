use std::io::{stdout, Write};

use crossterm::{cursor::MoveTo, style::Print, terminal, QueueableCommand};

use super::{position::Position, sprite::Sprite};

pub struct Entity {
    pub name: String,
    pub sprite: Sprite,
    pub position: Position,
}

#[allow(dead_code)]
pub fn print_sprites(entities: &[Entity]) {
    let mut stdout = stdout();
    stdout
        .queue(terminal::Clear(terminal::ClearType::All))
        .expect("Failed to clean terminal");
    entities
        .iter()
        .for_each(|entity| print_sprite(&entity.sprite, entity.position.x, entity.position.y));
    stdout.flush().expect("Failed to flush terminal prints");
}

fn print_sprite(sprite: &Sprite, original_x: u16, original_y: u16) {
    let mut stdout = stdout();
    let mut x = original_x;
    let mut y = original_y;
    sprite.pixels.iter().for_each(|line| {
        line.iter().for_each(|pixel| {
            stdout
                .queue(MoveTo(x, y))
                .expect("Failed to move position of cursor")
                .queue(Print(pixel))
                .expect("Failed to print pixel of sprite");
            x += 1;
        });
        x = original_x;
        y += 1;
    });
}
