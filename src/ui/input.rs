use crossterm::event::{self, Event, KeyCode};

use crate::{
    nbt::tag::{TagID, TagPayload},
    util::Unwrap,
};

use super::{traverse, TagTraversal, UI};

pub enum Status {
    Ok,
    Quit,
}

impl UI<'_> {
    pub fn get_events(&mut self) -> crossterm::Result<Status> {
        match event::read()? {
            Event::Key(key) => match key.code {
                KeyCode::Char(ch) => match ch {
                    'q' => return Ok(Status::Quit),
                    _ => (),
                },
                KeyCode::Enter => {
                    self.selected_tag.push(self.focused_tag.clone());
                    self.focused_tag = TagTraversal::None;
                }
                KeyCode::Tab => {
                    let selected_payload = &traverse(&self.selected_tag, self.tag)
                        .unwrap_or_err()
                        .payload;
                    match &self.focused_tag {
                        TagTraversal::Compound(name) => {
                            let subtags = selected_payload.as_compound().unwrap();
                            let idx = subtags
                                .iter()
                                // Remove Tag_END
                                .filter(|t| t.tag_id != TagID::End)
                                .position(|t| &t.name == name)
                                .unwrap();
                            self.focused_tag = TagTraversal::Compound(
                                subtags[if subtags.len() <= (idx + 2).try_into().unwrap() {
                                    0
                                } else {
                                    idx + 1
                                }]
                                .name
                                .clone(),
                            )
                        }
                        TagTraversal::Array(idx) => {
                            let payloads = selected_payload.as_list().unwrap().1;
                            self.focused_tag = TagTraversal::Array(
                                if payloads.len() <= (idx + 1).try_into().unwrap() {
                                    0
                                } else {
                                    idx + 1
                                },
                            );
                        }
                        TagTraversal::None => match selected_payload {
                            TagPayload::Compound(subtags) => {
                                // Account for Tag_END at the back
                                if !subtags.len() > 1 {
                                    self.focused_tag =
                                        TagTraversal::Compound(subtags[0].name.clone())
                                }
                            }
                            TagPayload::List(_, payloads) => {
                                if !payloads.is_empty() {
                                    self.focused_tag = TagTraversal::Array(0);
                                }
                            }
                            _ => unreachable!(),
                        },
                    }
                }
                _ => (),
            },
            _ => (),
        }

        Ok(Status::Ok)
    }
}
