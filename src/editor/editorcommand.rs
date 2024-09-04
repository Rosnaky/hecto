use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use super::terminal::Size;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    PageDown,
    PageUp,
    Home,
    End,
}

pub enum EditorCommand {
    Move(Direction),
    Resize(Size),
    Quit,
    Insert(char)
}

#[allow(clippy::as_conversions)]
impl TryFrom<Event> for EditorCommand {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                (KeyCode::Char(character), KeyModifiers::NONE | KeyModifiers::SHIFT) => Ok(Self::Insert(character)),
                (KeyCode::Up, KeyModifiers::NONE) => Ok(Self::Move(Direction::Up)),
                (KeyCode::Down, KeyModifiers::NONE) => Ok(Self::Move(Direction::Down)),
                (KeyCode::Left, KeyModifiers::NONE) => Ok(Self::Move(Direction::Left)),
                (KeyCode::Right, KeyModifiers::NONE) => Ok(Self::Move(Direction::Right)),
                (KeyCode::PageDown, KeyModifiers::NONE) => Ok(Self::Move(Direction::PageDown)),
                (KeyCode::PageUp, KeyModifiers::NONE) => Ok(Self::Move(Direction::PageUp)),
                (KeyCode::Home, KeyModifiers::NONE) => Ok(Self::Move(Direction::Home)),
                (KeyCode::End, KeyModifiers::NONE) => Ok(Self::Move(Direction::End)),
                _ => Err(format!("Unrecognized key: {:?}", code)),
            },
            Event::Resize(width_u16, height_u16) => Ok(Self::Resize(Size {
                height: height_u16 as usize,
                width: width_u16 as usize,
            })),
            _ => Err(format!("Unrecognized event: {:?}", event)),
        }
    }
}