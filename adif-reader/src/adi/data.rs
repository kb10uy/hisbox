/// If `text` is long enough to fetch string that has `length`, return `Some`.
pub fn get_field_value(text: &str, length: usize) -> Option<&str> {
    (text.len() >= length).then(|| &text[..length])
}
