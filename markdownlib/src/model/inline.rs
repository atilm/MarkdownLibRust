//! Inline elements (text, links, images) and a naive parser.

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Inline {
    Text(String),
    Link {
        text: String,
        url: String,
        title: Option<String>,
    },
    Image {
        alt: String,
        url: String,
        title: Option<String>,
    },
}

impl Inline {
    pub fn text<S: Into<String>>(s: S) -> Self {
        Inline::Text(s.into())
    }
}
