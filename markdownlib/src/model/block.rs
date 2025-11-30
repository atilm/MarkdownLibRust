//! Block-level elements.

use crate::model::heading::Heading;
use crate::model::paragraph::Paragraph;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Block {
    Heading(Heading),
    Paragraph(Paragraph),
}
