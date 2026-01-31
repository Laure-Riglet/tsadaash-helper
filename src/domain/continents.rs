use strum::IntoEnumIterator;
use strum_macros::{EnumIter, Display};

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, Display)]
pub enum Continents {
    Africa,
    America,
    Antarctica,
    Asia,
    Atlantic,
    Australia,
    Europe,
    Indian,
    Pacific,
}

impl Continents {
    pub fn vec() -> Vec<Continents> {
        Self::iter().collect()
    }
}
