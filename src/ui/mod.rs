use std::io::{self, Stdout};

use crossterm::{
    execute,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};

use crate::{
    nbt::tag::{traversal::TagTraversal, Tag},
    util::UnwrapOrStrErr,
};

use self::{input::Status, win::Window};

mod input;
mod render;
mod win;

pub struct UI<'a> {
    stdout: Stdout,
    tag: &'a Tag,
    breadcrumbs_win: Window,
    tree_win: Window,
    bottom_win: Window,
    selected_tag: Vec<TagTraversal>,
    focused_tag: TagTraversal,
}

impl UI<'_> {
    pub fn new(tag: &Tag) -> crossterm::Result<UI> {
        let size = terminal::size()?;
        let mut stdout = io::stdout();
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen)?;
        Ok(UI {
            stdout,
            tag,
            breadcrumbs_win: Window::new(0, 0, 0, 1)
                .unwrap_or_err("Could not create breadcrumbs window"),
            tree_win: Window::new(0, 1, size.0 / 2, size.1 - 2)
                .unwrap_or_err("Could not create tree window"),
            bottom_win: Window::new(0, size.1 - 1, 0, 1)
                .unwrap_or_err("Could not create bottom window"),
            selected_tag: vec![],
            focused_tag: TagTraversal::None,
        })
    }

    fn deinit(&mut self) -> crossterm::Result<()> {
        disable_raw_mode()?;
        execute!(self.stdout, LeaveAlternateScreen)
    }

    pub fn mainloop(&mut self) -> crossterm::Result<()> {
        'main: loop {
            self.render()?;

            #[allow(clippy::single_match)]
            match self.get_events()? {
                Status::Quit => break 'main,
                Status::Ok => (),
            }
        }
        self.deinit()
    }
}
