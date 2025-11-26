//! Paragraph structure built from inline elements.

use crate::model::inline::{Inline, parse_inlines};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Paragraph {
	pub inlines: Vec<Inline>,
}

impl Paragraph {
	/// Creates a paragraph treating entire input as plain text.
	pub fn new<S: Into<String>>(text: S) -> Self { Paragraph { inlines: vec![Inline::text(text)] } }
	/// Parses a raw markdown string into inline elements (naive).
	pub fn parse<S: AsRef<str>>(raw: S) -> Self { Paragraph { inlines: parse_inlines(raw.as_ref()) } }
	/// Concatenate visible text.
	pub fn visible_text(&self) -> String {
		fn accumulate(buf: &mut String, inline: &Inline) {
			match inline {
				Inline::Text(t) => buf.push_str(t),
				Inline::Link { text, .. } => for i in text { accumulate(buf, i); },
				Inline::Image { alt, .. } => for i in alt { accumulate(buf, i); },
			}
		}
		let mut out = String::new();
		for i in &self.inlines { accumulate(&mut out, i); }
		out
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn parse_paragraph() {
		let p = Paragraph::parse("Hello [world](https://ex.com) ![img](i.png)");
		assert!(p.inlines.iter().any(|i| matches!(i, Inline::Link { .. })));
		assert!(p.inlines.iter().any(|i| matches!(i, Inline::Image { .. })));
		assert!(p.visible_text().contains("world"));
	}
}
