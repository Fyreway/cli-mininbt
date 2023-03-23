use super::tag::{id::TagID, payload::TagPayload, Tag};

fn encode_payload(payload: &TagPayload) -> Vec<u8> {
    let mut bytes = vec![];
    match payload {
        TagPayload::End => (),
        TagPayload::Byte(n) => bytes.extend_from_slice(&n.to_be_bytes()),
        TagPayload::Short(n) => bytes.extend_from_slice(&n.to_be_bytes()),
        TagPayload::Int(n) => bytes.extend_from_slice(&n.to_be_bytes()),
        TagPayload::Long(n) => bytes.extend_from_slice(&n.to_be_bytes()),
        TagPayload::Float(n) => bytes.extend_from_slice(&n.to_be_bytes()),
        TagPayload::Double(n) => bytes.extend_from_slice(&n.to_be_bytes()),
        TagPayload::ByteArray(v) | TagPayload::IntArray(v) | TagPayload::LongArray(v) => {
            bytes.extend_from_slice(&(v.len() as u32).to_be_bytes());
            for b in v {
                bytes.append(&mut encode_payload(b));
            }
        }
        TagPayload::String(s) => {
            bytes.extend_from_slice(&(s.len() as u16).to_be_bytes());
            bytes.extend_from_slice(s.as_bytes());
        }
        TagPayload::List(id, v) => {
            bytes.push(*id as u8);
            bytes.extend_from_slice(&(v.len() as i32).to_be_bytes());
            for e in v {
                bytes.append(&mut encode_payload(e));
            }
        }
        TagPayload::Compound(tags) => {
            for tag in tags {
                bytes.append(&mut encode_tag(tag));
            }
        }
    }

    bytes
}

pub fn encode_tag(input: &Tag) -> Vec<u8> {
    let mut tag = input.clone();
    let mut bytes = vec![];

    // Tag ID
    tag.id = (&tag.payload).into();
    bytes.push(tag.id as u8);

    // Handle tag name
    if tag.id != TagID::End {
        bytes.extend_from_slice(&(tag.name.len() as u16).to_be_bytes());
        bytes.extend_from_slice(tag.name.as_bytes());
    }

    // Payload
    bytes.append(&mut encode_payload(&tag.payload));

    bytes
}
