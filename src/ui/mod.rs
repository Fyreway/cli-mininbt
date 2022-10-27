use std::io::{self, Stdout};

use crossterm::{
    execute,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};

use crate::{
    nbt::tag::{traversal::TagTraversal, Tag},
    util::Unwrap,
};

use self::{input::Status, window::Window};

mod input;
mod render;
mod window;

pub struct UI<'a> {
    stdout: Stdout,
    tag: &'a Tag,
    breadcrumbs_win: Window,
    tag_win: Window,
    bottom_win: Window,
    selected_tag: Vec<TagTraversal>,
    focused_tag: TagTraversal,
}

impl UI<'_> {
    pub fn new(tag: &Tag) -> crossterm::Result<UI> {
        let size = terminal::size().unwrap();
        let mut stdout = io::stdout();
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen)?;
        Ok(UI {
            stdout,
            tag,
            breadcrumbs_win: Window::new(0, 0, 0, 1).unwrap_or_err(),
            tag_win: Window::new(0, 1, 0, size.1 - 2).unwrap_or_err(),
            bottom_win: Window::new(0, size.1 - 1, 0, 1).unwrap_or_err(),
            selected_tag: vec![],
            focused_tag: TagTraversal::None,
        })
    }

    fn deinit(&mut self) -> crossterm::Result<()> {
        disable_raw_mode()?;
        execute!(self.stdout, LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn mainloop(&mut self) -> crossterm::Result<()> {
        'main: loop {
            self.render()?;

            #[allow(clippy::single_match)]
            match self.get_events()? {
                Status::Quit => break 'main,
                _ => (),
            }
        }
        self.deinit()?;
        Ok(())
    }
}
