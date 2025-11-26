//! Data model module aggregating all core types.

pub mod heading;
pub mod inline;
pub mod paragraph;
pub mod block;
pub mod document;

pub use heading::{Heading, HeadingLevelError};
pub use inline::{Inline, parse_inlines};
pub use paragraph::Paragraph;
pub use block::Block;
pub use document::Document;