//! Block-level elements.

use crate::model::heading::Heading;
use crate::model::paragraph::Paragraph;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Block {
    Heading(Heading),
    Paragraph(Paragraph),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::heading::Heading;
    use crate::model::paragraph::Paragraph;
    #[test]
    fn construct_blocks() {
        let h = Heading::new(1, "Title").unwrap();
        let p = Paragraph::new("Text");
        let _bh = Block::Heading(h);
        let _bp = Block::Paragraph(p);
    }
}
