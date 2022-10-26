use std::{
    fmt::{self, Write},
    io::{self, Stdout},
};

use crossterm::{
    execute,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
// most amazing crate I've ever used
use enum_as_inner::EnumAsInner;

use crate::{
    nbt::tag::{Tag, TagID},
    util::Unwrap,
};

use self::{input::Status, window::Window};

mod input;
mod render;
mod window;

#[derive(Clone, EnumAsInner)]
enum TagTraversal {
    Compound(String),
    Array(i32),
    None,
}

impl fmt::Display for TagTraversal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compound(name) => f.write_fmt(format_args!("{name}")),
            Self::Array(idx) => f.write_fmt(format_args!("[{idx}]")),
            Self::None => f.write_char('_'),
        }
    }
}

/// Traverses a path indicated by a vector of TagTraversal.
fn traverse<'a>(path: &'a Vec<TagTraversal>, root: &'a Tag) -> Result<&'a Tag, &str> {
    let mut tag = root;
    let mut payload = &tag.payload;
    for traversal in path {
        match traversal {
            TagTraversal::Compound(name) => {
                let subtags = payload.as_compound().unwrap();
                tag = subtags
                    .iter()
                    .find(|t| &t.name == name)
                    .ok_or("Invalid name")?;
                payload = &tag.payload;
            }
            TagTraversal::Array(idx) => {
                let (tag_id, payloads) = payload.as_list().unwrap();
                payload = payloads.get(*idx as usize).ok_or("Invalid index")?;
                if tag_id != &TagID::Compound || tag_id != &TagID::List {
                    return Err("Cannot traverse into non-container tag");
                }
            }
            _ => unreachable!(),
        }
    }

    Ok(tag)
}

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
