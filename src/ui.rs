use std::io::{self, Stdout};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::nbt::tag::Tag;

enum Status {
    Ok,
    Quit,
}

pub struct UI<'a> {
    stdout: Stdout,
    tag: &'a Tag,
}

impl UI<'_> {
    pub fn new<'a>(tag: &'a Tag) -> crossterm::Result<UI<'a>> {
        let mut stdout = io::stdout();
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen)?;
        Ok(UI { stdout, tag })
    }

    fn deinit(&mut self) -> crossterm::Result<()> {
        disable_raw_mode()?;
        execute!(self.stdout, LeaveAlternateScreen)?;
        Ok(())
    }

    fn get_events(&mut self) -> crossterm::Result<Status> {
        match event::read()? {
            Event::Key(key) => match key.code {
                KeyCode::Char(ch) => match ch {
                    'q' => return Ok(Status::Quit),
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }

        Ok(Status::Ok)
    }

    pub fn mainloop(&mut self) -> crossterm::Result<()> {
        'main: loop {
            match self.get_events()? {
                Status::Quit => break 'main,
                _ => (),
            }
        }
        self.deinit()?;
        Ok(())
    }
}
