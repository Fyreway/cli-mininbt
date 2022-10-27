use std::{fmt::Display, io::Stdout};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{PrintStyledContent, StyledContent},
    terminal,
};

pub struct Window {
    y: u16,
    x: u16,
    w: u16,
    h: u16,
}

impl Window {
    pub fn new(y: u16, x: u16, w: u16, h: u16) -> Result<Self, &'static str> {
        let size = terminal::size().unwrap();
        if y + w > size.0 || x + h > size.1 {
            Err("Invalid dimensions")
        } else {
            Ok(Self {
                y,
                x,
                w: if w == 0 { size.0 } else { w },
                h: if h == 0 { size.1 } else { h },
            })
        }
    }

    pub fn mv(&mut self, stdout: &mut Stdout, y: u16, x: u16) -> crossterm::Result<&mut Self> {
        queue!(stdout, MoveTo(y + self.y, x + self.x))?;
        Ok(self)
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
        )?;
        Ok(self)
    }

    pub fn write<T: Display>(
        &mut self,
        stdout: &mut Stdout,
        s: StyledContent<T>,
    ) -> crossterm::Result<&mut Self> {
        queue!(stdout, PrintStyledContent(s))?;
        Ok(self)
    }
}
