use std::io::{self, Stdout};

use crossterm::{
    cursor, execute,
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

    // path to current tag (EXCLUDING name in container)
    selected_tag: Vec<TagTraversal>,

    // name in current container
    focused_tag: TagTraversal,
}

impl UI<'_> {
    pub fn new(tag: &Tag) -> crossterm::Result<UI> {
        let size = terminal::size()?;
        Ok(UI {
            stdout: io::stdout(),
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

    pub fn mainloop(&mut self) -> crossterm::Result<()> {
        enable_raw_mode()?;
        execute!(self.stdout, EnterAlternateScreen, cursor::Hide)?;

        self.move_focus(true);

        'main: loop {
            self.render()?;

            #[allow(clippy::single_match)]
            match self.get_events()? {
                Status::Quit => break 'main,
                Status::Ok => (),
            }
        }

        disable_raw_mode()?;
        execute!(self.stdout, LeaveAlternateScreen, cursor::Show)?;
        Ok(())
    }
}
