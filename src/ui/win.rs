use std::{fmt::Display, io::Stdout};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{PrintStyledContent, StyledContent},
    terminal,
};

#[derive(Debug)]
pub enum WindowError {
    CrosstermError(crossterm::ErrorKind),
    InvalidDimensions,
}

impl ToString for WindowError {
    fn to_string(&self) -> String {
        match self {
            Self::CrosstermError(e) => format!("Crossterm error: {e}"),
            Self::InvalidDimensions => "Invalid dimensions".to_string(),
        }
    }
}

pub struct Window {
    y: u16,
    x: u16,
    // w: u16,
    // h: u16,
}

impl Window {
    pub fn new(y: u16, x: u16, w: u16, h: u16) -> Result<Self, WindowError> {
        let size = terminal::size().map_err(WindowError::CrosstermError)?;
        if y + w > size.0 || x + h > size.1 {
            Err(WindowError::InvalidDimensions)
        } else {
            Ok(Self {
                y,
                x,
                // w: if w == 0 { size.0 } else { w },
                // h: if h == 0 { size.1 } else { h },
            })
        }
    }

    pub fn home(&mut self, stdout: &mut Stdout) -> crossterm::Result<&mut Self> {
        queue!(stdout, MoveTo(self.y, self.x)).map(|_| self)
    }

    pub fn mv(&mut self, stdout: &mut Stdout, y: u16, x: u16) -> crossterm::Result<&mut Self> {
        queue!(stdout, MoveTo(y + self.y, x + self.x)).map(|_| self)
    }

    pub fn mvwrite<T: Display>(
        &mut self,
        stdout: &mut Stdout,
        y: u16,
        x: u16,
        s: StyledContent<T>,
    ) -> crossterm::Result<&mut Self> {
        queue!(
            stdout,
            MoveTo(y + self.y, x + self.x),
            PrintStyledContent(s)
        )
        .map(|_| self)
    }

    pub fn write<T: Display>(
        &mut self,
        stdout: &mut Stdout,
        s: StyledContent<T>,
    ) -> crossterm::Result<&mut Self> {
        queue!(stdout, PrintStyledContent(s)).map(|_| self)
    }
}
