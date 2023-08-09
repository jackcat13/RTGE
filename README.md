# RTGE : Rust Terminal Game Engine

## What is it ?

The purpose of this project is to provide a library in order to do graphical rendering in a terminal with Rust language 🦀.

But why ? Because it's a fun achievement and it allows to developp fun games bazingly fast within terminals.

## Getting started

Add library to your cargo project :

```
cargo add rtge
```

### Print a "sprite" in terminal

```rust
fn main() {
    let mut bob = Entity {
        name: "bob".to_string(),
        sprite: load_sprite("./manualTests/bob.json".to_string()),
        position: Position { x: 100, y: 100 },
        direction: Direction {
            up: false,
            down: false,
            left: false,
            right: false,
        },
        speed: 2,
    };
    let entities = vec![bob];
    print_sprites(&entities);
}
```

The "sprite" definition can be found here : https://github.com/jackcat13/RTGE/blob/main/manualTests/bob.json

You need to first define an entity to represent the sprite to print. Then, the `print_sprites` method is responsible to print the entities at the proper places.

Note : To use the other features, plase take a look at the documentation (in progress) and to [examples](https://github.com/jackcat13/RTGE_examples).
