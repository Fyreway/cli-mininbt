use crate::nbt::tag::{id::TagID, Tag};

#[test]
fn parse_compound_tag() {
    // Should be an empty TAG_Compound named "Foo"
    let bytes = vec![0x0a, 0x00, 0x03, 0x46, 0x6f, 0x6f, 0x00];
    let tag = Tag::new(&bytes).unwrap();
    assert_eq!(tag.name, "Foo");
    let subtags = tag.payload.as_compound().unwrap();
    assert_eq!(subtags.len(), 1);
}

#[test]
fn parse_list_tag() {
    // Should be a TAG_List named "Bar", with type String, and contents ["Spam", "and", "Eggs"]
    let bytes = vec![
        0x09, 0x00, 0x03, 0x42, 0x61, 0x72, 0x08, 0x00, 0x00, 0x00, 0x03, 0x00, 0x04, 0x53, 0x70,
        0x61, 0x6d, 0x00, 0x03, 0x61, 0x6e, 0x64, 0x00, 0x04, 0x45, 0x67, 0x67, 0x73,
    ];
    let tag = Tag::new(&bytes).unwrap();
    assert_eq!(tag.name, "Bar");
    let (tag_id, payloads) = tag.payload.as_list().unwrap();
    assert_eq!(tag_id, &TagID::String);
    assert_eq!(payloads.len(), 3);
    assert_eq!(payloads[0].as_string().unwrap(), "Spam");
    assert_eq!(payloads[1].as_string().unwrap(), "and");
    assert_eq!(payloads[2].as_string().unwrap(), "Eggs");
}
