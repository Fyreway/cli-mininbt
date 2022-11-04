use crossterm::event::{self, Event, KeyCode};

use crate::nbt::tag::{
    id::TagID,
    payload::TagPayload,
    traversal::{TagTraversal, TraversedTag},
};

use super::UI;

pub enum Status {
    Ok,
    Quit,
}

impl UI<'_> {
    fn move_focus(&mut self, forward: bool) {
        let res = TagTraversal::traverse(&self.selected_tag, self.tag).unwrap();
        let selected_payload = &res.get_tag().payload;
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
                    subtags[if forward {
                        if subtags.len() <= idx + 2 {
                            0
                        } else {
                            idx + 1
                        }
                    } else if idx == 0 {
                        subtags.len() - 2
                    } else {
                        idx - 1
                    }]
                    .name
                    .clone(),
                );
            }
            TagTraversal::Array(idx) => {
                let payloads = selected_payload.as_list().unwrap().1;
                self.focused_tag = TagTraversal::Array(if forward {
                    if payloads.len() <= (idx + 1).try_into().unwrap() {
                        0
                    } else {
                        idx + 1
                    }
                } else if *idx == 0 {
                    payloads.len() as i32 - 1
                } else {
                    *idx - 1
                });
            }
            TagTraversal::None => match selected_payload {
                TagPayload::Compound(subtags) => {
                    // Account for Tag_END at the back
                    if !subtags.len() > 1 {
                        self.focused_tag = TagTraversal::Compound(
                            subtags[if forward { 0 } else { subtags.len() - 2 }]
                                .name
                                .clone(),
                        );
                    }
                }
                TagPayload::List(_, payloads) => {
                    if !payloads.is_empty() {
                        self.focused_tag = TagTraversal::Array(if forward {
                            0
                        } else {
                            (payloads.len() - 1).try_into().unwrap()
                        });
                    }
                }
                _ => unreachable!(),
            },
        }
    }

    pub fn get_events(&mut self) -> crossterm::Result<Status> {
        #[allow(clippy::single_match)]
        match event::read()? {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => return Ok(Status::Quit),
                KeyCode::Enter => {
                    self.selected_tag.push(self.focused_tag.clone());
                    match TagTraversal::traverse(&self.selected_tag, self.tag).unwrap() {
                        TraversedTag::Tag(_) => self.focused_tag = TagTraversal::None,
                        TraversedTag::Contained(_) => {
                            self.selected_tag.pop();
                        }
                    }
                }
                KeyCode::Esc => {
                    if let Some(tag) = self.selected_tag.pop() {
                        self.focused_tag = tag;
                    }
                }
                KeyCode::Tab => self.move_focus(true),
                KeyCode::BackTab => self.move_focus(false),
                _ => (),
            },
            _ => (),
        }

        Ok(Status::Ok)
    }
}
