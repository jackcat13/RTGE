//! Provides helpers for inputs processing

//in mod.rs

use std::time::Duration;

use crossterm::event::{
    Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers,
};
use futures::{select, FutureExt, StreamExt};
use futures_timer::Delay;

/// Helper function to process any Stream event in a simplified way. Events are processed based on an input closure and returns Ok if the input processing didn't fail.
///
/// # Examples
///
/// ## Inputs processing inside a loop (break on keyboard escape)
///
/// ```ignore
/// let inputs_rules = |event: Event| -> Result<(), String> {
///    if event == Event::Key(KeyCode::Esc.into()) {
///        return Err("Escape event".to_string());
///    }
///    Ok(())
/// };
///
/// match process_inputs(inputs_rules).await {
///    Ok(_) => {}
///    Err(_) => {
///       break;
///    }
/// }
/// ```
pub async fn process_inputs(
    mut inputs_rules: impl FnMut(Event) -> Result<(), String>,
) -> Result<(), String> {
    let mut reader = EventStream::new();
    let mut delay = Delay::new(Duration::from_millis(60)).fuse();
    let mut event = reader.next().fuse();
    select! {
        _ = delay => {  },
        maybe_event = event => {
            match maybe_event {
                Some(Ok(event)) => {
                    return inputs_rules(event)
                }
                Some(Err(_)) => { return Err("Error while processing inputs".to_string()) },
                None => { return Ok(()) },
            }
        }
    };
    Ok(())
}

/// Helper function to create an [`crossterm::event::Event`] instance reprensenting a keyboard input. Useful helper to be used for the `input_rules` input param of [`process_inputs`] function.
///
/// # Examples
///
/// ## `p` character event representation
///
/// ```ignore
/// let p_event = process_key_press_event('p');
/// ```
pub fn process_key_press_event(key_character: char) -> Event {
    Event::Key(KeyEvent {
        code: KeyCode::Char(key_character),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    })
}
