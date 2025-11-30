//! Paragraph structure built from inline elements.

use crate::model::inline::{Inline};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Paragraph {
	pub inlines: Vec<Inline>,
}

impl Paragraph {
	/// Creates a paragraph treating entire input as plain text.
	pub fn new<S: Into<String>>(text: S) -> Self { Paragraph { inlines: vec![Inline::text(text)] } }
	/// Concatenate visible text.
	pub fn visible_text(&self) -> String {
		fn accumulate(buf: &mut String, inline: &Inline) {
			match inline {
				Inline::Text(t) => buf.push_str(t),
				Inline::Link { text, .. } => buf.push_str(text),
				Inline::Image { alt, .. } => buf.push_str(alt),
			}
		}
		let mut out = String::new();
		for i in &self.inlines { accumulate(&mut out, i); }
		out
	}
}
