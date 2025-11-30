//! Crate root: re-exports the Markdown data model.
//!
//! Provides simple types for representing a Markdown document in memory.
//! Parsing and rendering are intentionally naive and minimal.
//!
//! # Example
//! ```
//! use markdownlib::{Heading, Paragraph, Block, Document};
//!
//! let heading = Heading::new(1, "Title").unwrap();
//! let para = Paragraph::parse("A [link](https://example.com) and ![img](img.png)");
//! let mut doc = Document::new();
//! doc.push(Block::Heading(heading));
//! doc.push(Block::Paragraph(para));
//! assert_eq!(doc.len(), 2);
//! ```

pub mod model;
pub mod markdown_parser;
pub use model::{Heading, HeadingLevelError, Inline, Paragraph, Block, Document};
