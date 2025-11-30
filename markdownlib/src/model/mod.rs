//! Data model module aggregating all core types.

pub mod block;
pub mod document;
pub mod heading;
pub mod inline;
pub mod paragraph;

pub use block::Block;
pub use document::Document;
pub use heading::{Heading, HeadingLevelError};
pub use inline::Inline;
pub use paragraph::Paragraph;
