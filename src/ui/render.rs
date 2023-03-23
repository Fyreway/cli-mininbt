use std::io::Write;

use crossterm::{
    queue,
    style::{StyledContent, Stylize},
    terminal::{Clear, ClearType},
};

use crate::nbt::tag::{id::TagID, payload::TagPayload, traversal::traverse};

use super::{EditMode, UI};

fn get_value_display(id: TagID, payload: &TagPayload) -> StyledContent<String> {
    let formatted = format!("{payload}");
    match id {
        TagID::Byte | TagID::Short | TagID::Int | TagID::Long => formatted.magenta(),
        TagID::Float | TagID::Double => formatted.dark_magenta(),
        TagID::ByteArray | TagID::List | TagID::IntArray | TagID::LongArray => {
            formatted.dark_green()
        }
        TagID::String => formatted.cyan(),
        TagID::Compound => formatted.dark_red(),
        TagID::End => unreachable!(),
    }
}

impl UI<'_> {
    fn render_array_type(&mut self, id: TagID, payloads: &[TagPayload]) -> crossterm::Result<()> {
        for (i, payload) in payloads.iter().enumerate() {
            self.tree_win
                .mvwrite(&mut self.stdout, 0, i.try_into().unwrap(), "- ".grey())?
                .write(&mut self.stdout, {
                    let formatted = format!("{i}").red();
                    if self
                        .focused_trav
                        .clone()
                        .as_array()
                        .map_or_else(|| false, |&idx| idx == i as i32)
                    {
                        formatted.on_dark_blue()
                    } else {
                        formatted
                    }
                })?
                .write(&mut self.stdout, ": ".grey())?
                .write(&mut self.stdout, get_value_display(id, payload))?;
        }
        Ok(())
    }
    fn render_array(&mut self, payload: &TagPayload) -> crossterm::Result<()> {
        self.render_array_type(
            payload.into(),
            match payload {
                TagPayload::List(_, p)
                | TagPayload::ByteArray(p)
                | TagPayload::IntArray(p)
                | TagPayload::LongArray(p) => p,
                _ => unreachable!(),
            },
        )?;
        Ok(())
    }

    fn render_compound(&mut self, payload: &TagPayload) -> crossterm::Result<()> {
        for (i, subtag) in payload
            .as_compound()
            .unwrap()
            .iter()
            .filter(|t| t.id != TagID::End)
            .enumerate()
        {
            self.tree_win
                .mvwrite(&mut self.stdout, 0, i.try_into().unwrap(), "- ".grey())?
                .write(&mut self.stdout, {
                    let formatted = format!("\"{}\"", subtag.name).red();
                    if if let Some(name) = self.focused_trav.as_compound() {
                        name == &subtag.name
                    } else {
                        false
                    } {
                        formatted.on_dark_blue()
                    } else {
                        formatted
                    }
                })?
                .write(&mut self.stdout, ": ".grey())?
                .write(
                    &mut self.stdout,
                    get_value_display((&subtag.payload).into(), &subtag.payload),
                )?;
        }
        Ok(())
    }

    fn render_statusbar(&mut self) -> crossterm::Result<()> {
        self.bottom_win.home(&mut self.stdout)?.write(
            &mut self.stdout,
            format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
                .bold()
                .blue(),
        )?;
        Ok(())
    }

    fn render_edit(&mut self) -> crossterm::Result<()> {
        self.edit_win
            .home(&mut self.stdout)?
            .write(&mut self.stdout, "Type: ".bold().yellow())?
            .write(
                &mut self.stdout,
                if let EditMode::Type(input, _) = &self.edit_mode {
                    input.as_str().stylize()
                } else {
                    match self.focused_id.unwrap() {
                        TagID::Byte => "Byte".magenta(),
                        TagID::Short => "Short".magenta(),
                        TagID::Int => "Int".magenta(),
                        TagID::Long => "Long".magenta(),
                        TagID::Float => "Float".dark_magenta(),
                        TagID::Double => "Double".dark_magenta(),
                        TagID::ByteArray => "ByteArray".dark_green(),
                        TagID::List => "List".dark_green(),
                        TagID::String => "String".cyan(),
                        TagID::Compound => "Compound".dark_red(),
                        TagID::IntArray => "IntArray".dark_green(),
                        TagID::LongArray => "LongArray".dark_green(),
                        TagID::End => unreachable!(),
                    }
                },
            )?
            .nextline(&mut self.stdout)?
            .write(&mut self.stdout, "Value: ".bold().yellow())?
            .write(
                &mut self.stdout,
                if let EditMode::Value(input, _) = &self.edit_mode {
                    input.to_string().stylize()
                } else {
                    get_value_display(
                        self.focused_id.unwrap(),
                        self.focused_payload.as_ref().unwrap(),
                    )
                },
            )?;
        Ok(())
    }

    pub fn render(&mut self) -> crossterm::Result<()> {
        queue!(self.stdout, Clear(ClearType::All))?;
        let payload = traverse(&self.selected_tag, self.tag)
            .unwrap()
            .get_payload();
        match Into::<TagID>::into(&payload) {
            TagID::Compound => self.render_compound(&payload)?,
            _ => self.render_array(&payload)?,
        }
        self.breadcrumbs_win.mv(&mut self.stdout, 0, 0)?;
        for tr in &self.selected_tag {
            self.breadcrumbs_win
                .write(&mut self.stdout, {
                    let s = tr.to_string();
                    if tr.as_array().is_some() {
                        s.dark_green()
                    } else {
                        s.dark_red()
                    }
                })?
                .write(&mut self.stdout, " > ".dark_grey())?;
        }
        self.render_statusbar()?;
        self.render_edit()?;
        self.stdout.flush()?;
        Ok(())
    }
}
