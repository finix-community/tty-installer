/// Returns the single character an installer screen should accept as typed input, or `None` when the event does not carry printable text
pub fn printable_char(input: &str) -> Option<char> {
    let mut chars = input.chars();
    let c = chars.next()?;
    if chars.next().is_some() {
        return None;
    }
    let code = c as u32;
    if c.is_control() || (0xE000..=0xF8FF).contains(&code) {
        return None;
    }
    Some(c)
}
