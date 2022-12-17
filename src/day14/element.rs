use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub enum Element {
    EMPTY,
    ROCK,
    SAND,
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::EMPTY => write!(f, "."),
            Element::ROCK => write!(f, "#"),
            Element::SAND => write!(f, "O"),
        }
    }
}
