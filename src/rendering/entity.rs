//! It provides helpers to display and move entities in a simple way as well as game entities representation.

//in mod.rs

use std::{
    io::{stdout, Write},
    panic,
};

use crate::controls::direction::Direction;
use crossterm::{
    cursor::MoveTo,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, size},
    QueueableCommand,
};
use rayon::prelude::{IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use super::{position::Position, sprite::Sprite};

/// Representation of an entity in games that is used to be displayed in terminal. It contains attributes required to process games calculations (like position, speed, ...)
#[derive(Clone, PartialEq, Debug)]
pub struct Entity {
    pub name: String,
    pub sprite: Sprite,
    pub position: Position,
    pub direction: Direction,
    pub speed: u16,
    pub animation_name: Option<String>,
}

/// Helper function to display a collection of entities in the terminal. It relies on the entities sprites and positions to process printing.
///
/// # Examples
///
/// ## Display an entity
///
/// ```ignore
///let mut camel = Entity {
///    name: "bob".to_string(),
///    sprite: load_sprite("./manualTests/animated_camel.json".to_string()),
///    position: Position {
///        x: TERM_SIZE_X / 2,
///        y: TERM_SIZE_Y / 2,
///    },
///    direction: Direction {
///        up: false,
///        down: false,
///        left: false,
///        right: false,
///    },
///    speed: 2,
///    animation_name: Option::Some("walking".to_string()),
/// };
///
/// let entities = vec![camel];
/// print_sprites(&mut entities);
/// ```
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
        None => print_frame(&sprite.pixels, x, y, &sprite.colors),
        Some(name) => print_animated(sprite, x, y, &name.clone()),
    };
}

fn print_frame(
    pixels: &[Vec<char>],
    original_x: u16,
    original_y: u16,
    colors: &Option<Vec<Vec<u8>>>,
) {
    let mut stdout = stdout();
    let mut x = original_x;
    let mut y = original_y;
    let flat_colors;
    let mut colors_iter;
    if let Some(colors_clone) = colors.clone() {
        flat_colors = colors_clone
            .into_par_iter()
            .flat_map(|color| color)
            .collect::<Vec<u8>>();
        colors_iter = Some(flat_colors.iter());
    } else {
        colors_iter = None
    };
    pixels.iter().for_each(|line| {
        line.iter().for_each(|pixel| {
            if let Some(colors_iter) = colors_iter.as_mut() {
                stdout
                    .queue(SetForegroundColor(Color::AnsiValue(
                        *colors_iter.next().unwrap(),
                    )))
                    .expect("Failed to setup pixel color");
            }
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
    stdout.queue(ResetColor).expect("Failed to reset color");
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
                print_frame(&frame.pixels, x, y, &frame.colors);
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

/// Similar function than [`print_sprites`] except that it takes in addition an entity on which the camera will be centered.
///
/// # Examples
///
/// ## Display two entities with camera centered on an entity
///
/// ```ignore
/// let mut camel = Entity {
///    name: "camel".to_string(),
///    sprite: load_sprite("./manualTests/animated_camel.json".to_string()),
///    position: Position {
///        x: TERM_SIZE_X / 2,
///        y: TERM_SIZE_Y / 2,
///    },
///    direction: Direction {
///        up: false,
///        down: false,
///        left: false,
///        right: false,
///    },
///    speed: 2,
///    animation_name: Option::Some("walking".to_string()),
/// };
///
/// print_sprites_centered_on(&mut camel, &mut enemies);
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

/// Move an [`Entity`] instance based on its direction and speed.
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

/// Call [`move_entity`] in parallel on a collection of [`Entity`]
#[allow(dead_code)]
pub fn move_entities(entities: Vec<Entity>) -> Vec<Entity> {
    entities.into_par_iter().map(move_entity).collect()
}
