use super::terminal::Size;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Home,
    End,
}

pub enum Command {
    Move(Direction),
    Insert(char),
    Remove,
    Resize(Size),
    Quit,
}

impl TryFrom<KeyCode> for Direction {
    type Error = String;

    // Match specific directionally related key codes from crossterm,
    // to a custom direction enum
    fn try_from(code: KeyCode) -> Result<Self, Self::Error> {
        match code {
            KeyCode::Up => Ok(Self::Up),
            KeyCode::Down => Ok(Self::Down),
            KeyCode::Left => Ok(Self::Left),
            KeyCode::Right => Ok(Self::Right),
            KeyCode::PageUp => Ok(Self::PageUp),
            KeyCode::PageDown => Ok(Self::PageDown),
            KeyCode::Home => Ok(Self::Home),
            KeyCode::End => Ok(Self::End),
            _ => Err(format!("Direction not supported!")),
        }
    }
}

impl TryFrom<Event> for Command {
    type Error = String;

    // Matches event from crossterm to a custom command enum
    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                modifiers,
                ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                (
                    KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::PageDown
                    | KeyCode::PageUp
                    | KeyCode::Home
                    | KeyCode::End,
                    _,
                ) => match Direction::try_from(code) {
                    Ok(direction) => Ok(Self::Move(direction)),
                    Err(err) => Err(format!("{err}")),
                },
                (KeyCode::Char(c), _) => Ok(Self::Insert(c)),
                (KeyCode::Backspace, _) => Ok(Self::Remove),
                _ => Err(format!("Code not supported!")),
            },
            Event::Resize(width_u16, height_u16) => {
                // Cast u16 to usize, and ignore clippy warnings
                #[allow(clippy::as_conversions)]
                let width_usize: usize = width_u16 as usize;
                #[allow(clippy::as_conversions)]
                let height_usize: usize = height_u16 as usize;

                Ok(Self::Resize(Size {
                    width: width_usize,
                    height: height_usize,
                }))
            }
            _ => Err(format!("Event not supported!")),
        }
    }
}
