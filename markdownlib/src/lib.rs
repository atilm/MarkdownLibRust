//! Core data structures representing a Markdown document.
//!
//! This module currently provides a lightweight in-memory representation
//! of a Markdown document as a sequence of block elements (headings and
//! paragraphs). It does not yet perform parsing; that can be added later
//! by transforming raw text lines into these structures.
//!
//! # Example
//! ```
//! use markdownlib::{Heading, Paragraph, Block, Document};
//!
//! let heading = Heading::new(1, "Title").unwrap();
//! let para = Paragraph::new("This is a paragraph of text.");
//!
//! let mut doc = Document::default();
//! doc.push(Block::Heading(heading));
//! doc.push(Block::Paragraph(para));
//!
//! assert_eq!(doc.blocks().len(), 2);
//! ```

/// A Markdown heading (ATX style) with a level from 1 to 6.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Heading {
    /// The heading level (1..=6).
    pub level: u8,
    /// The raw text content of the heading (without leading `#` characters).
    pub text: String,
}

impl Heading {
    /// Creates a new `Heading`, validating that the level is in the range 1..=6.
    pub fn new<L: Into<u8>, S: Into<String>>(level: L, text: S) -> Result<Self, HeadingLevelError> {
        let lvl = level.into();
        if (1..=6).contains(&lvl) {
            Ok(Heading {
                level: lvl,
                text: text.into(),
            })
        } else {
            Err(HeadingLevelError(lvl))
        }
    }
}

/// Error returned when attempting to construct a heading with an invalid level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeadingLevelError(pub u8);

impl core::fmt::Display for HeadingLevelError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "invalid heading level: {} (expected 1..=6)", self.0)
    }
}

impl std::error::Error for HeadingLevelError {}

/// Inline elements that can appear inside paragraph text.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Inline {
    /// Plain textual content.
    Text(String),
    /// A Markdown link: `[text](url "optional title")`.
    Link {
        text: Vec<Inline>,
        url: String,
        title: Option<String>,
    },
    /// A Markdown image: `![alt](url "optional title")`.
    Image {
        alt: Vec<Inline>,
        url: String,
        title: Option<String>,
    },
}

impl Inline {
    /// Convenience constructor for plain text.
    pub fn text<S: Into<String>>(s: S) -> Self {
        Inline::Text(s.into())
    }
}

/// A Markdown paragraph: one or more inline elements.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Paragraph {
    /// Ordered inline elements.
    pub inlines: Vec<Inline>,
}

impl Paragraph {
    /// Creates a paragraph treating the whole input as a single text inline.
    pub fn new<S: Into<String>>(text: S) -> Self {
        Paragraph {
            inlines: vec![Inline::text(text)],
        }
    }
}

/// A block element in a Markdown document.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Block {
    /// A heading (`#`, `##`, etc.).
    Heading(Heading),
    /// A paragraph of text.
    Paragraph(Paragraph),
}

/// An entire Markdown document represented as an ordered sequence of blocks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Document {
    blocks: Vec<Block>,
}

impl Document {
    /// Creates an empty document.
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }
    /// Returns a slice of the blocks.
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }
    /// Adds a block to the end of the document.
    pub fn push(&mut self, block: Block) {
        self.blocks.push(block);
    }
    /// Returns true if the document has no blocks.
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }
    /// Number of blocks in the document.
    pub fn len(&self) -> usize {
        self.blocks.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading_level_validation() {
        assert!(Heading::new(1, "Ok").is_ok());
        assert!(Heading::new(6, "Ok").is_ok());
        assert!(Heading::new(0, "Bad").is_err());
        assert!(Heading::new(7, "Bad").is_err());
    }

    #[test]
    fn build_document() {
        let h = Heading::new(2, "Section").unwrap();
        let p = Paragraph::new(
            "Some paragraph with a [link](https://example.com) and an ![alt](img.png \"Title\") image.",
        );
        let mut doc = Document::new();
        doc.push(Block::Heading(h));
        doc.push(Block::Paragraph(p));
        assert_eq!(doc.len(), 2);
        assert!(!doc.is_empty());
    }
}
