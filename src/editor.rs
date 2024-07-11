use crossterm::event::{read, Event::{self}, KeyCode::{self}, KeyEvent, KeyEventKind, KeyModifiers};
mod terminal;
mod view;
use terminal::{Size, Position, Terminal};
use view::View;
use std::{cmp::min, env, io::Error};

#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View
}

impl Editor {
    
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.handle_args();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn handle_args(&mut self) {
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            self.view.load(file_name);
        }
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(event);
        }
        Ok(())
    }
    fn move_point(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location {mut x, mut y} = self.location;
        let Size {height, width} = Terminal::size()?;
        match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                if y < height.saturating_sub(1) {
                    y = min(height.saturating_sub(1), y.saturating_add(1));
                }
            }
            KeyCode::Left => {    
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            _ => (),
        }
        self.location = Location {x, y};
        Ok(())
    }
    fn evaluate_event(&mut self, event: Event) {
        match event {
            Event::Key(KeyEvent {code, modifiers, kind: KeyEventKind::Press, ..})
         => match (code, modifiers)
            {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                }
                (
                    KeyCode::Up | 
                    KeyCode::Down | 
                    KeyCode::Left | 
                    KeyCode::Right | 
                    KeyCode::PageDown | 
                    KeyCode::PageUp | 
                    KeyCode::Home | 
                    KeyCode::End, 
                    KeyModifiers::NONE) => {
                        self.move_point(code).unwrap();
                    }
                _ => (),
            },
            Event::Resize(width_u16, height_u16) => {
                #[allow(clippy::as_conversions)]
                let height = height_u16 as usize;
                #[allow(clippy::as_conversions)]
                let width = width_u16 as usize;
                self.view.resize(Size {height: height, width: width});
            }
            _ => ()
        }
    }
    fn refresh_screen(&mut self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            self.view.render()?;
            Terminal::move_caret_to(Position {col: self.location.x, row: self.location.y})?;
        }
        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }
}