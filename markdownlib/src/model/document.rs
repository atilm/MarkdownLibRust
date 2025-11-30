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
