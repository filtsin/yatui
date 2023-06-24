use std::borrow::Cow;

/// Simple wrapper around `String` with **cached** size in terminal way
pub struct RawText {
    content: Cow<'static, str>,
    size: RawTextSize,
}

pub struct RawTextSize {
    pub column: usize,
    pub lines: usize,
    pub graphemes: usize,
}

impl RawText {}
