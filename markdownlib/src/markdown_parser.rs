use std::iter::Peekable;
use std::str::Chars;

use crate::{Block, Document, Heading};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Markdown parsing error at line {line}, column {column}: {message}")]
pub struct MarkdownParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

enum ParseState {
    Start,
    Heading,
    // Paragraph,
    // LinkText,
    // LinkUrl,
}

fn parse(input: &str) -> Result<Document, MarkdownParseError> {
    let mut state = ParseState::Start;

    if input.is_empty() {
        return Ok(Document::new());
    }

    let mut doc = Document::new();

    // Parse heading from input
    let mut characters = input.chars().peekable();
    let mut line_number = 1;

    while let Some(c) = characters.peek() {
        if *c == '\n' {
            line_number += 1;
            characters.next();
            continue;
        }

        match state {
            ParseState::Start => {
                if *c == '#' {
                    state = ParseState::Heading;
                }
            }
            ParseState::Heading => {
                let heading = parse_heading(&mut characters, line_number)?;
                doc.push(Block::Heading(heading));
            }
        }
    }

    Ok(doc)
}

fn parse_heading(
    characters: &mut Peekable<Chars<'_>>,
    line_number: usize,
) -> Result<Heading, MarkdownParseError> {
    let mut level = 0;
    while let Some('#') = characters.next() {
        level += 1;
    }

    let mut title = String::new();
    while let Some(c) = characters.peek() {
        if *c == '\n' {
            break;
        }

        let c = characters.next().unwrap();
        title.push(c);
    }

    Heading::new(level, title.trim()).map_err(|e| MarkdownParseError {
        message: format!("Invalid heading level: {}", e.0),
        line: line_number,
        column: 1,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_document() {
        let input = "";
        let doc = parse(input).expect("Failed to parse document");

        assert_eq!(doc.len(), 0);
    }

    #[test]
    fn parse_first_level_heading() {
        let input = "# Heading 1";
        let doc = parse(input).expect("Failed to parse document");

        assert_eq!(doc.len(), 1);
        match &doc.get(0) {
            Block::Heading(heading) => {
                assert_eq!(heading.level, 1);
                assert_eq!(heading.text, "Heading 1");
            }
            _ => panic!("Expected a Heading block"),
        }
    }

    #[test]
    fn parse_second_level_heading() {
        let input = "## Heading 2";
        let doc = parse(input).expect("Failed to parse document");

        assert_eq!(doc.len(), 1);
        match &doc.get(0) {
            Block::Heading(heading) => {
                assert_eq!(heading.level, 2);
                assert_eq!(heading.text, "Heading 2");
            }
            _ => panic!("Expected a Heading block"),
        }
    }

    #[test]
    fn parse_two_subsequent_headings() {
        let input = "### Heading 3\n#### Heading 4";
        let doc = parse(input).expect("Failed to parse document");

        assert_eq!(doc.len(), 2);
        match &doc.get(0) {
            Block::Heading(heading) => {
                assert_eq!(heading.level, 3);
                assert_eq!(heading.text, "Heading 3");
            }
            _ => panic!("Expected a Heading block"),
        }
        match &doc.get(1) {
            Block::Heading(heading) => {
                assert_eq!(heading.level, 4);
                assert_eq!(heading.text, "Heading 4");
            }
            _ => panic!("Expected a Heading block"),
        }
    }

    #[test]
    fn parse_invalid_heading_level() {
        let input = "####### Invalid Heading";
        let result = parse(input);

        match result {
            Err(MarkdownParseError {
                message,
                line,
                column,
            }) => {
                assert_eq!(line, 1);
                assert_eq!(column, 1);
                assert!(message.contains("Invalid heading level: 7"));
            }
            _ => panic!("Expected a MarkdownParseError"),
        }
    }

    #[test]
    fn parse_invalid_heading_level_in_third_line() {
        let input = "# Valid Heading\n## Another Valid Heading\n####### Invalid Heading";
        let result = parse(input);

        match result {
            Err(MarkdownParseError {
                message,
                line,
                column,
            }) => {
                assert_eq!(line, 3);
                assert_eq!(column, 1);
                assert!(message.contains("Invalid heading level: 7"));
            }
            _ => panic!("Expected a MarkdownParseError"),
        }
    }
}
