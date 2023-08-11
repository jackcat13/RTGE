//! Init functions automatically called during init of programs to setup terminal setup so that it's possible to play games in terminals.

//in mod.rs

use static_init::{constructor, destructor};
use std::io::stdout;

use crossterm::terminal::enable_raw_mode;
use crossterm::{event::PopKeyboardEnhancementFlags, execute, terminal::disable_raw_mode};

#[constructor(0)]
extern "C" fn init() {
    let mut stdout = stdout();
    execute!(stdout, PopKeyboardEnhancementFlags)
        .expect("Could not active pop keyboard enhancement");
    match enable_raw_mode() {
        Ok(_) => {}
        Err(_) => println!("Could not activate raw mode"),
    }
}

#[destructor(0)]
extern "C" fn destroy() {
    match disable_raw_mode() {
        Ok(_) => {}
        Err(_) => println!("Could not disable raw mode"),
    }
}
