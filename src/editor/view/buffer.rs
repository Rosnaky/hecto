use std::{fs::read_to_string, io::Error};
use super::Location;
use super::line::Line;



#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
}

impl Buffer {
    pub fn load(file_name: &str) -> Result<Self, Error> {
        let file_contents = read_to_string(file_name)?;
        let mut lines = Vec::new();
        for line in file_contents.lines() {
            lines.push(Line::from(line));
        }
        Ok(Self { lines })
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
    pub fn height(&self) -> usize {
        self.lines.len()
    }

    pub fn insert_char(&mut self, c: char, at: Location) {
        if at.line_index > self.lines.len() {
            return;
        }
        if at.line_index == self.lines.len() {
            self.lines.push(Line::from(&c.to_string()));
        }
        else if let Some(line) = self.lines.get_mut(at.line_index) {
            line.insert_char(at.grapheme_index, c);
        }
    }

    pub fn delete(&mut self, at: Location) {
        if let Some(line) = self.lines.get_mut(at.line_index) {
            line.delete(at.grapheme_index);
        }
    }
}