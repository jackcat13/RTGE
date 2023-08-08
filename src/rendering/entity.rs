use std::{
    io::{stdout, Write},
    panic,
};

use crossterm::{
    cursor::MoveTo,
    style::Print,
    terminal::{self, size},
    QueueableCommand,
};

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

#[allow(dead_code)]
pub fn print_sprites_centered_on(entity_centered: &Entity, other_entities: &[Entity]) {
    let mut stdout = stdout();
    let (term_size_x, term_size_y) = size().expect("Failed to get terminal size");
    let middle_x = term_size_x / 2;
    let middle_y = term_size_y / 2;
    stdout
        .queue(terminal::Clear(terminal::ClearType::All))
        .expect("Failed to clean terminal");
    print_sprite(&entity_centered.sprite, middle_x, middle_y);
    other_entities.iter().for_each(|entity| {
        let old_hook = panic::take_hook();
        panic::set_hook(Box::new(|_info| {}));
        let maybe_error = std::panic::catch_unwind(|| {
            print_sprite(
                &entity.sprite,
                middle_x + entity.position.x - entity_centered.position.x,
                middle_y + entity.position.y - entity_centered.position.y,
            )
        });
        if maybe_error.is_err() {}
        panic::set_hook(old_hook);
    });
    stdout
        .queue(MoveTo(term_size_x, term_size_y))
        .expect("Failed to move to last position")
        .queue(Print(" "))
        .expect("Failed to print to last position");
    stdout.flush().expect("Failed to flush terminal prints");
}
