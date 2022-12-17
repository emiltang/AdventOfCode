use std::fmt::{Display, Formatter};

use crate::day14::element::Element;

pub struct Row(pub Vec<Element>);

impl Display for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in self.0.iter() {
            write!(f, "{}", i)?
        }
        Ok(())
    }
}
