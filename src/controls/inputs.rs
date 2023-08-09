use std::time::Duration;

use crossterm::event::{
    Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers,
};
use futures::{select, FutureExt, StreamExt};
use futures_timer::Delay;

pub async fn process_inputs(
    mut inputs_rules: impl FnMut(Event) -> Result<(), String>,
) -> Result<(), String> {
    let mut reader = EventStream::new();
    let mut delay = Delay::new(Duration::from_millis(60)).fuse();
    let mut event = reader.next().fuse();
    select! {
        _ = delay => { print!(""); },
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

pub fn process_key_press_event(direction: char) -> Event {
    Event::Key(KeyEvent {
        code: KeyCode::Char(direction),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    })
}
