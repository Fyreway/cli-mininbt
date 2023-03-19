use std::io::Write;

use crossterm::{
    queue,
    style::Stylize,
    terminal::{Clear, ClearType},
};

use crate::nbt::tag::{id::TagID, payload::TagPayload, traversal::TagTraversal, Tag};

use super::UI;

impl UI<'_> {
    fn render_array_type(
        &mut self,
        tag_id: TagID,
        payloads: &[TagPayload],
    ) -> crossterm::Result<()> {
        for (i, item) in payloads.iter().enumerate() {
            self.tree_win
                .mvwrite(
                    &mut self.stdout,
                    0,
                    (i + 1).try_into().unwrap(),
                    "- ".grey(),
                )?
                .write(&mut self.stdout, {
                    let formatted = format!("{i}").red();
                    if if let TagTraversal::Array(idx) = self.focused_tag.clone() {
                        idx == i.try_into().unwrap()
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
    fn render_array(&mut self, tag: &Tag) -> crossterm::Result<()> {
        self.tree_win.mvwrite(
            &mut self.stdout,
            0,
            0,
            format!("\"{}\"", tag.name).as_str().green(),
        )?;
        self.render_array_type(
            (&tag.payload).into(),
            match &tag.payload {
                TagPayload::List(_, p)
                | TagPayload::ByteArray(p)
                | TagPayload::IntArray(p)
                | TagPayload::LongArray(p) => p,
                _ => unreachable!(),
            },
        )?;
        Ok(())
    }

    fn render_compound(&mut self, tag: &Tag) -> crossterm::Result<()> {
        self.tree_win.mvwrite(
            &mut self.stdout,
            0,
            0,
            format!("\"{}\"", tag.name).as_str().green(),
        )?;
        for (i, subtag) in tag
            .payload
            .as_compound()
            .unwrap()
            .iter()
            .filter(|t| t.tag_id != TagID::End)
            .enumerate()
        {
            self.tree_win
                .mvwrite(
                    &mut self.stdout,
                    0,
                    (i + 1).try_into().unwrap(),
                    "- ".grey(),
                )?
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
        let tag = res.get_tag();
        match tag.tag_id {
            TagID::Compound => self.render_compound(tag)?,
            TagID::ByteArray | TagID::List | TagID::IntArray | TagID::LongArray => {
                self.render_array(tag)?;
            }
            _ => (),
        }
        self.breadcrumbs_win.mv(&mut self.stdout, 0, 0)?;
        for tr in &self.selected_tag {
            self.breadcrumbs_win
                .write(&mut self.stdout, tr.to_string().grey())?
                .write(&mut self.stdout, " > ".dark_grey())?;
        }
        self.bottom_win
            .mvwrite(&mut self.stdout, 0, 0, "[q]uit".stylize())?;
        self.stdout.flush()?;
        Ok(())
    }
}
