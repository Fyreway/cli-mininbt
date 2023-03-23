use std::{
    fs,
    io::{self, Stdout, Write},
    path::PathBuf,
};

use crossterm::{
    cursor, execute,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use enum_as_inner::EnumAsInner;
use flate2::{write::GzEncoder, Compression};

use crate::{
    nbt::{
        encode::encode_tag,
        tag::{id::TagID, payload::TagPayload, traversal::TagTraversal, Tag},
    },
    util::UnwrapOrDisplayErr,
};

use self::{input::Status, win::Window};

mod input;
mod render;
mod win;

#[derive(Clone, EnumAsInner)]
pub enum EditMode {
    None,
    Type(String, usize),
    Value(String, usize),
}

pub struct UI<'a> {
    filename: PathBuf,
    stdout: Stdout,
    tag: &'a mut Tag,
    breadcrumbs_win: Window,
    tree_win: Window,
    edit_win: Window,
    bottom_win: Window,

    // path to current tag (EXCLUDING name in container)
    selected_tag: Vec<TagTraversal>,

    // name in current container
    focused_trav: TagTraversal,

    focused_payload: Option<TagPayload>,
    focused_id: Option<TagID>,

    edit_mode: EditMode,
}

impl UI<'_> {
    pub fn new(filename: PathBuf, tag: &mut Tag) -> crossterm::Result<UI> {
        let size = terminal::size()?;
        Ok(UI {
            filename,
            stdout: io::stdout(),
            tag,
            breadcrumbs_win: Window::new(0, 0, 0, 1).unwrap(),
            tree_win: Window::new(0, 1, size.0 / 2, size.1 - 2).unwrap(),
            edit_win: Window::new(size.0 / 2 + 1, 1, size.0 / 2 - 1, size.1 - 2).unwrap(),
            bottom_win: Window::new(0, size.1 - 1, 0, 1).unwrap(),
            selected_tag: vec![],
            focused_trav: TagTraversal::None,
            focused_payload: None,
            focused_id: None,
            edit_mode: EditMode::None,
        })
    }

    fn get_full_trav(&self) -> Vec<TagTraversal> {
        let mut trav = self.selected_tag.clone();
        trav.push(self.focused_trav.clone());
        trav
    }

    fn write(&self) {
        let mut gz = GzEncoder::new(vec![], Compression::default());
        gz.write_all(&encode_tag(self.tag))
            .unwrap_or_err("Could not read tag");

        fs::write(
            &self.filename,
            gz.finish().unwrap_or_err("Could not unzip file"),
        )
        .unwrap_or_err("Could not write to file");
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
                Status::Change => self.edit_mode = EditMode::Type(String::new(), 0),
                Status::Write => self.write(),
            }
        }

        disable_raw_mode()?;
        execute!(self.stdout, LeaveAlternateScreen, cursor::Show)?;
        Ok(())
    }
}
