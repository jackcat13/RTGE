use std::{
    io::{stdout, Write},
    panic,
};

use crate::controls::direction::Direction;
use crossterm::{
    cursor::MoveTo,
    style::Print,
    terminal::{self, size},
    QueueableCommand,
};
use rayon::prelude::{IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use super::{position::Position, sprite::Sprite};

#[derive(Clone, PartialEq, Debug)]
pub struct Entity {
    pub name: String,
    pub sprite: Sprite,
    pub position: Position,
    pub direction: Direction,
    pub speed: u16,
    pub animation_name: Option<String>,
}

#[allow(dead_code)]
pub fn print_sprites(entities: &mut [Entity]) {
    let mut stdout = stdout();
    stdout
        .queue(terminal::Clear(terminal::ClearType::All))
        .expect("Failed to clean terminal");
    entities.par_iter_mut().for_each(|entity| {
        print_sprite(
            &mut entity.sprite,
            entity.position.x,
            entity.position.y,
            &entity.animation_name,
        )
    });
    stdout.flush().expect("Failed to flush terminal prints");
}

fn print_sprite(sprite: &mut Sprite, x: u16, y: u16, animation_name: &Option<String>) {
    match animation_name {
        None => print_frame(&sprite.pixels, x, y),
        Some(name) => print_animated(sprite, x, y, &name.clone()),
    };
}

fn print_frame(pixels: &[Vec<char>], original_x: u16, original_y: u16) {
    let mut stdout = stdout();
    let mut x = original_x;
    let mut y = original_y;
    pixels.iter().for_each(|line| {
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

fn print_animated(sprite: &mut Sprite, x: u16, y: u16, animation_name: &String) {
    match sprite.animations.clone() {
        None => panic!("No animation found in sprite whereas an animation_name is configured"),
        Some(animations) => animations
            .iter()
            .filter(|animation| animation.name.eq(animation_name))
            .for_each(|animation| {
                let frames_number = animation.frames.clone().len();
                let frame = animation
                    .frames
                    .get(resolve_frame_number(sprite, frames_number))
                    .expect("Failed to resolve frame of animated sprite");
                print_frame(&frame.pixels, x, y);
            }),
    }
}

fn resolve_frame_number(sprite: &mut Sprite, animations_count: usize) -> usize {
    let mut result = 0;
    if let Some(number) = sprite.animation_index {
        result = number;
        if number < animations_count - 1 {
            sprite.animation_index = Some(number + 1);
        } else {
            sprite.animation_index = Some(0);
        }
    };
    result
}

#[allow(dead_code)]
pub fn print_sprites_centered_on(entity_centered: &mut Entity, other_entities: &mut Vec<Entity>) {
    let mut stdout = stdout();
    let (term_size_x, term_size_y) = size().expect("Failed to get terminal size");
    let middle_x = term_size_x / 2;
    let middle_y = term_size_y / 2;
    stdout
        .queue(terminal::Clear(terminal::ClearType::All))
        .expect("Failed to clean terminal");

    print_sprite(
        &mut entity_centered.sprite,
        middle_x,
        middle_y,
        &entity_centered.animation_name,
    );
    other_entities.par_iter_mut().for_each(|entity| {
        // Avoid panic and therefore print if coordinates are negative
        let old_hook = panic::take_hook();
        panic::set_hook(Box::new(|_info| {}));
        if let Ok(result) = std::panic::catch_unwind(|| -> (u16, u16) {
            (
                (middle_x + entity.position.x - entity_centered.position.x),
                (middle_y + entity.position.y - entity_centered.position.y),
            )
        }) {
            let (relative_x, relative_y) = result;
            print_sprite(
                &mut entity.sprite,
                relative_x,
                relative_y,
                &entity.animation_name,
            );
        }
        panic::set_hook(old_hook);
    });
    stdout
        .queue(MoveTo(term_size_x, term_size_y))
        .expect("Failed to move to last position")
        .queue(Print(" "))
        .expect("Failed to print to last position");
    stdout.flush().expect("Failed to flush terminal prints");
}

#[allow(dead_code)]
pub fn move_entities(entities: Vec<Entity>) -> Vec<Entity> {
    entities.into_par_iter().map(move_entity).collect()
}

#[allow(dead_code)]
pub fn move_entity(mut entity: Entity) -> Entity {
    if entity.direction.up {
        entity.position.y -= entity.speed;
    }
    if entity.direction.down {
        entity.position.y += entity.speed;
    }
    if entity.direction.left {
        entity.position.x -= entity.speed;
    }
    if entity.direction.right {
        entity.position.x += entity.speed;
    }
    entity
}
