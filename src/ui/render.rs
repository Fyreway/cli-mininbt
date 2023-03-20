use std::io::Write;

use crossterm::{
    queue,
    style::Stylize,
    terminal::{Clear, ClearType},
};

use crate::nbt::tag::{id::TagID, payload::TagPayload, traversal::TagTraversal};

use super::UI;

impl UI<'_> {
    fn render_array_type(
        &mut self,
        tag_id: TagID,
        payloads: &[TagPayload],
    ) -> crossterm::Result<()> {
        for (i, item) in payloads.iter().enumerate() {
            self.tree_win
                .mvwrite(&mut self.stdout, 0, i.try_into().unwrap(), "- ".grey())?
                .write(&mut self.stdout, {
                    let formatted = format!("{i}").red();
                    if self
                        .focused_tag
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
                .write(&mut self.stdout, {
                    let formatted = format!("{item}");
                    match tag_id {
                        TagID::Byte | TagID::Short | TagID::Int | TagID::Long => {
                            formatted.magenta()
                        }
                        TagID::Float | TagID::Double => formatted.dark_magenta(),
                        TagID::ByteArray | TagID::List | TagID::IntArray | TagID::LongArray => {
                            formatted.dark_green()
                        }
                        TagID::String => formatted.cyan(),
                        TagID::Compound => formatted.dark_red(),
                        TagID::End => unreachable!(),
                    }
                })?;
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
            .filter(|t| t.tag_id != TagID::End)
            .enumerate()
        {
            self.tree_win
                .mvwrite(&mut self.stdout, 0, i.try_into().unwrap(), "- ".grey())?
                .write(&mut self.stdout, {
                    let formatted = format!("\"{}\"", subtag.name).red();
                    if if let Some(name) = self.focused_tag.as_compound() {
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
                .write(&mut self.stdout, {
                    let formatted = format!("{}", subtag.payload);
                    match subtag.tag_id {
                        TagID::Byte | TagID::Short | TagID::Int | TagID::Long => {
                            formatted.magenta()
                        }
                        TagID::Float | TagID::Double => formatted.dark_magenta(),
                        TagID::ByteArray | TagID::List | TagID::IntArray | TagID::LongArray => {
                            formatted.dark_green()
                        }
                        TagID::String => formatted.cyan(),
                        TagID::Compound => formatted.dark_red(),
                        TagID::End => unreachable!(),
                    }
                })?;
        }
        Ok(())
    }

    pub fn render(&mut self) -> crossterm::Result<()> {
        queue!(self.stdout, Clear(ClearType::All))?;
        let selected_tag = self.selected_tag.clone();
        let res = TagTraversal::traverse(&selected_tag, self.tag).unwrap();
        let payload = res.get_payload();
        match Into::<TagID>::into(payload) {
            TagID::Compound => self.render_compound(payload)?,
            _ => self.render_array(payload)?,
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
        self.bottom_win
            .mvwrite(&mut self.stdout, 0, 0, "[q]uit".stylize())?;
        self.stdout.flush()?;
        Ok(())
    }
}
