#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Category {
    Language = 0,
    Keyboard = 1,
    Timezone = 2,
    LocaleFormat = 3,
}

impl Category {
    pub(super) fn from_i32(value: i32) -> Self {
        match value {
            0 => Category::Language,
            1 => Category::Keyboard,
            3 => Category::LocaleFormat,
            _ => Category::Timezone,
        }
    }

    pub(super) fn next(self) -> Self {
        match self {
            Category::Language => Category::Keyboard,
            Category::Keyboard => Category::Timezone,
            Category::Timezone => Category::LocaleFormat,
            Category::LocaleFormat => Category::LocaleFormat,
        }
    }
}
