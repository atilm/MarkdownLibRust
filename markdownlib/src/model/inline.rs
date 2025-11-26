//! Inline elements (text, links, images) and a naive parser.

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Inline {
    Text(String),
    Link {
        text: Vec<Inline>,
        url: String,
        title: Option<String>,
    },
    Image {
        alt: Vec<Inline>,
        url: String,
        title: Option<String>,
    },
}

impl Inline {
    pub fn text<S: Into<String>>(s: S) -> Self {
        Inline::Text(s.into())
    }
}

pub fn parse_inlines(input: &str) -> Vec<Inline> {
    let mut out = Vec::new();
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'!' || bytes[i] == b'[' {
            let is_image = bytes[i] == b'!';
            let start_bracket = if is_image { i + 1 } else { i };
            if start_bracket < bytes.len() && bytes.get(start_bracket) == Some(&b'[') {
                if let Some(closing) = input[start_bracket + 1..].find(']') {
                    let closing_idx = start_bracket + 1 + closing;
                    if closing_idx + 1 < bytes.len() && bytes.get(closing_idx + 1) == Some(&b'(') {
                        if let Some(paren_close_rel) = input[closing_idx + 2..].find(')') {
                            let paren_close = closing_idx + 2 + paren_close_rel;
                            let alt_or_text = &input[start_bracket + 1..closing_idx];
                            let inside = &input[closing_idx + 2..paren_close];
                            let (url, title) = if let Some(space_pos) = inside.find(' ') {
                                let (u, rest) = inside.split_at(space_pos);
                                let rest = rest.trim_start();
                                let title = if rest.starts_with('"')
                                    && rest.ends_with('"')
                                    && rest.len() >= 2
                                {
                                    Some(rest[1..rest.len() - 1].to_string())
                                } else {
                                    None
                                };
                                (u.to_string(), title)
                            } else {
                                (inside.to_string(), None)
                            };
                            let text_inlines = vec![Inline::Text(alt_or_text.to_string())];
                            if is_image {
                                out.push(Inline::Image {
                                    alt: text_inlines,
                                    url,
                                    title,
                                });
                            } else {
                                out.push(Inline::Link {
                                    text: text_inlines,
                                    url,
                                    title,
                                });
                            }
                            i = paren_close + 1;
                            continue;
                        }
                    }
                }
            }
        }
        let next_special = input[i + 1..]
            .find(|c| c == '!' || c == '[')
            .map(|p| i + 1 + p)
            .unwrap_or(bytes.len());
        out.push(Inline::Text(input[i..next_special].to_string()));
        i = next_special;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_basic() {
        let v = parse_inlines("A [link](https://ex.com \"Title\") and ![img](image.png)");
        assert!(v.iter().any(|i| matches!(i, Inline::Link { .. })));
        assert!(v.iter().any(|i| matches!(i, Inline::Image { .. })));
    }
}
