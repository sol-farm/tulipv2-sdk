//! Functions for converting strings to the "tag format", and taking a tag formatted value
//! and converting it back into a string. Assumes all data is standard UTF8 encoded

pub fn tag(value: &str) -> Option<[u8; 32]> {
    if value.len().gt(&32) {
        return None;
    }
    let mut _tag = [0_u8; 32];
    for (idx, b) in value.as_bytes().iter().enumerate() {
        _tag[idx] = *b;
    }
    Some(_tag)
}

/// converts a tag byte array into a tag string
/// 0 values are ignored and not parsed into a string
pub fn tag_to_str(data: &[u8; 32]) -> String {
    let mut value = String::with_capacity(32);
    data.iter().for_each(|b| {
        let b = *b;
        if b != 0 {
            value.push(b as char);
        }
    });
    value
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tag() {
        let tag_msg = "tulip".to_string();
        let _want_tag_msg = String::from("tulip\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}");
        let mut too_big_tag = String::with_capacity(100);
        for _ in 0..100 {
            too_big_tag.push('a');
        }
        assert!(tag(&too_big_tag).is_none());
        let tag_bytes = tag(&tag_msg).unwrap();
        let got_tag_msg = tag_to_str(&tag_bytes);
        assert_eq!(got_tag_msg, tag_msg);
    }
}
