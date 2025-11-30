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
//! let mut doc = Document::new();
//! doc.push(Block::Heading(heading));
//! assert_eq!(doc.len(), 1);
//! ```

pub mod model;
pub mod markdown_parser;
pub use model::{Heading, HeadingLevelError, Inline, Paragraph, Block, Document};
