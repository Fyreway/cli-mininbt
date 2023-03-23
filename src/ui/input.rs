use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::nbt::tag::{
    id::TagID,
    payload::TagPayload,
    traversal::{set, traverse, TagTraversal},
};

use super::{EditMode, UI};

pub enum Status {
    Ok,
    Quit,
    Change,
    Write,
}

fn to_id(value: &str) -> Option<TagID> {
    match value {
        "Byte" => Some(TagID::Byte),
        "Short" => Some(TagID::Short),
        "Int" => Some(TagID::Int),
        "Long" => Some(TagID::Long),
        "Float" => Some(TagID::Float),
        "Double" => Some(TagID::Double),
        "String" => Some(TagID::String),
        _ => None,
    }
}

impl UI<'_> {
    fn update_focused_tag(&mut self) {
        self.focused_payload = Some(
            traverse(&self.get_full_trav(), self.tag)
                .unwrap()
                .get_payload(),
        );
        self.focused_id = Some(self.focused_payload.as_ref().unwrap().into());
    }

    pub fn move_focus(&mut self, forward: bool) {
        let res = traverse(&self.selected_tag, self.tag).unwrap();
        let selected_payload = res.get_payload();
        match &self.focused_trav {
            TagTraversal::Compound(name) => {
                let subtags = selected_payload.as_compound().unwrap();
                let idx = subtags
                    .iter()
                    // Remove Tag_END
                    .filter(|t| t.id != TagID::End)
                    .position(|t| &t.name == name)
                    .unwrap();
                self.focused_trav = TagTraversal::Compound(
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
                self.focused_trav = TagTraversal::Array(if forward {
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
                        self.focused_trav = TagTraversal::Compound(
                            subtags[if forward { 0 } else { subtags.len() - 2 }]
                                .name
                                .clone(),
                        );
                    }
                }
                TagPayload::List(_, payloads) => {
                    if !payloads.is_empty() {
                        self.focused_trav = TagTraversal::Array(if forward {
                            0
                        } else {
                            (payloads.len() - 1).try_into().unwrap()
                        });
                    }
                }
                _ => unreachable!(),
            },
        }

        self.update_focused_tag();
    }

    pub fn get_events(&mut self) -> crossterm::Result<Status> {
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match &mut self.edit_mode {
                EditMode::None => match code {
                    KeyCode::Char('q') => return Ok(Status::Quit),
                    KeyCode::Enter => {
                        if traverse(&self.get_full_trav(), self.tag)
                            .unwrap()
                            .is_container()
                        {
                            self.selected_tag.push(self.focused_trav.clone());
                            self.focused_trav = TagTraversal::None;
                            self.move_focus(true);
                        }
                    }
                    KeyCode::Esc => {
                        if let Some(trav) = self.selected_tag.pop() {
                            self.focused_trav = trav;
                            self.update_focused_tag();
                        } else {
                            return Ok(Status::Quit);
                        }
                    }
                    KeyCode::Tab | KeyCode::Down => self.move_focus(true),
                    KeyCode::BackTab | KeyCode::Up => self.move_focus(false),
                    KeyCode::Char('c') => return Ok(Status::Change),
                    KeyCode::Char('w') => return Ok(Status::Write),
                    _ => (),
                },

                EditMode::Value(text, idx) => match code {
                    KeyCode::Char(ch) => {
                        text.insert(*idx, ch);
                        *idx += 1;
                    }
                    KeyCode::Backspace => {
                        *idx = idx.saturating_sub(1);
                        text.remove(*idx);
                    }
                    KeyCode::Left => *idx = idx.saturating_sub(1),
                    KeyCode::Right => *idx += 1,
                    KeyCode::Enter => {
                        if let Some(payload) = self.focused_id.unwrap().parse(text) {
                            self.focused_payload = Some(payload.clone());
                            let mut tag = self.tag.clone();
                            set(&self.get_full_trav(), &mut tag, payload);
                            *self.tag = tag;
                            self.edit_mode = EditMode::None;
                        }
                    }
                    _ => (),
                },
                EditMode::Type(text, idx) => match code {
                    KeyCode::Char(ch) => {
                        text.insert(*idx, ch);
                        *idx += 1;
                    }
                    KeyCode::Backspace => {
                        *idx = idx.saturating_sub(1);
                        text.remove(*idx);
                    }
                    KeyCode::Left => *idx = idx.saturating_sub(1),
                    KeyCode::Right => *idx += 1,
                    KeyCode::Enter => {
                        if let Some(id) = to_id(text) {
                            self.focused_id = Some(id);
                            self.focused_payload = None;
                            self.edit_mode = EditMode::Value(String::new(), 0);
                        }
                    }
                    _ => (),
                },
            }
        }

        Ok(Status::Ok)
    }
}
