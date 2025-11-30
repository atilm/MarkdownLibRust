//! Document: collection of block elements.

use crate::model::block::Block;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Document {
    blocks: Vec<Block>,
}

impl Document {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }
    pub fn push(&mut self, block: Block) {
        self.blocks.push(block);
    }
    pub fn get(& self, index: usize) -> &Block {
        &self.blocks[index]
    }
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }
    pub fn len(&self) -> usize {
        self.blocks.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::block::Block;
    use crate::model::heading::Heading;
    use crate::model::paragraph::Paragraph;
    #[test]
    fn build_document() {
        let mut doc = Document::new();
        doc.push(Block::Heading(Heading::new(2, "Section").unwrap()));
        doc.push(Block::Paragraph(Paragraph::parse("A [link](u)")));
        assert_eq!(doc.len(), 2);
        assert!(!doc.is_empty());
    }
}
