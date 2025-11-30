use std::iter::Peekable;
use std::str::Chars;

use crate::{Block, Document, Heading, Inline, Paragraph};
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
    Paragraph,
    ParagraphEndCandidate,
    // LinkText,
    // LinkUrl,
}

fn parse(input: &str) -> Result<Document, MarkdownParseError> {
    let mut state = ParseState::Start;

    let mut doc = Document::new();

    // Parse heading from input
    let mut characters = input.chars().peekable();
    let mut line_number = 1;

    let mut paragraph_text = String::new();
    let mut paragraph_end_candidate_text = String::new();

    while let Some(next_c) = characters.peek() {
        if *next_c == '\n' {
            line_number += 1;
        }

        match state {
            ParseState::Start => {
                state = ParseState::Paragraph;
            }
            ParseState::Heading => {
                let heading = parse_heading(&mut characters, line_number)?;
                line_number += 1;
                doc.push(Block::Heading(heading));

                state = ParseState::Paragraph;
            }
            ParseState::Paragraph => {
                if *next_c == '#' {
                    state = ParseState::Heading;
                    continue;
                }

                if let Some(c) = characters.next() {
                    if c == '\n' {
                        paragraph_end_candidate_text.push(c);
                        state = ParseState::ParagraphEndCandidate;
                        continue;
                    }

                    paragraph_text.push(c);
                }
            }
            ParseState::ParagraphEndCandidate => {
                if let Some(c) = characters.next() {
                    // If we encounter another newline, we finalize the paragraph
                    if c == '\n' {
                        doc.push(Block::Paragraph(Paragraph {
                            inlines: vec![Inline::Text(paragraph_text.to_string())],
                        }));
                        paragraph_text.clear();
                        paragraph_end_candidate_text.clear();
                        state = ParseState::Paragraph;
                        continue;
                    }
                    
                    // Whitespace after newline, continue accumulating
                    if c.is_whitespace() {
                        paragraph_end_candidate_text.push(c);
                        continue;
                    }

                    // This is just another line in the same paragraph, possibly after some spaces
                    paragraph_text.push_str(&paragraph_end_candidate_text);
                    paragraph_text.push(c);
                    paragraph_end_candidate_text.clear();
                    state = ParseState::Paragraph;
                }
            }
        }
    }

    if paragraph_text.len() > 0 {
        doc.push(Block::Paragraph(Paragraph {
            inlines: vec![Inline::Text(paragraph_text.to_string())],
        }));
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
            characters.next();
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
    use crate::{Inline, Paragraph};

    use super::*;

    fn assert_heading(block: &Block, expected_level: u8, expected_text: &str) {
        match block {
            Block::Heading(heading) => {
                assert_eq!(heading.level, expected_level);
                assert_eq!(heading.text, expected_text);
            }
            _ => panic!("Expected a Heading block"),
        }
    }

    fn assert_parsing_error(
        result: Result<Document, MarkdownParseError>,
        expected_line: usize,
        expected_column: usize,
        expected_message_substring: &str,
    ) {
        match result {
            Err(MarkdownParseError {
                message,
                line,
                column,
            }) => {
                assert_eq!(line, expected_line);
                assert_eq!(column, expected_column);
                assert!(message.contains(expected_message_substring));
            }
            _ => panic!("Expected a MarkdownParseError"),
        }
    }

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
        assert_heading(&doc.get(0), 1, "Heading 1");
    }

    #[test]
    fn parse_second_level_heading() {
        let input = "## Heading 2";
        let doc = parse(input).expect("Failed to parse document");

        assert_eq!(doc.len(), 1);
        assert_heading(&doc.get(0), 2, "Heading 2");
    }

    #[test]
    fn parse_two_subsequent_headings() {
        let input = "### Heading 3\n#### Heading 4";
        let doc = parse(input).expect("Failed to parse document");

        assert_eq!(doc.len(), 2);
        assert_heading(&doc.get(0), 3, "Heading 3");
        assert_heading(&doc.get(1), 4, "Heading 4");
    }

    #[test]
    fn parse_invalid_heading_level() {
        let input = "####### Invalid Heading";
        let result = parse(input);

        assert_parsing_error(result, 1, 1, "Invalid heading level: 7");
    }

    #[test]
    fn parse_invalid_heading_level_in_third_line() {
        let input = "# Valid Heading\n## Another Valid Heading\n####### Invalid Heading";
        let result = parse(input);

        assert_parsing_error(result, 3, 1, "Invalid heading level: 7");
    }

    #[test]
    fn parse_heading_with_leading_and_trailing_spaces() {
        let input = "#    Heading with spaces    ";
        let doc = parse(input).expect("Failed to parse document");
        assert_eq!(doc.len(), 1);
        assert_heading(&doc.get(0), 1, "Heading with spaces");
    }

    fn assert_paragraph(block: &Block, expected_text: &[&str]) {
        let expected_paragraph: Paragraph = Paragraph {
            inlines: expected_text
                .iter()
                .map(|&s| Inline::Text(s.to_string()))
                .collect(),
        };

        match block {
            Block::Paragraph(p) => {
                assert_eq!(*p, expected_paragraph)
            }
            _ => panic!("Expected a Paragraph block"),
        }
    }

    #[test]
    fn parse_paragraph_after_heading() {
        let input = "# Heading 1\nThis is a paragraph.";
        let doc = parse(input).expect("Failed to parse document");

        assert_eq!(doc.len(), 2);
        assert_heading(&doc.get(0), 1, "Heading 1");
        assert_paragraph(&doc.get(1), ["This is a paragraph."].as_ref());
    }

    #[test]
    fn parse_paragraph_at_beginning_of_document() {
        let input = "This is a paragraph at the beginning.";
        let doc = parse(input).expect("Failed to parse document");

        assert_eq!(doc.len(), 1);
        assert_paragraph(
            &doc.get(0),
            ["This is a paragraph at the beginning."].as_ref(),
        );
    }

    #[test]
    fn parse_multiple_paragraphs() {
        let input = "First paragraph.\n\nSecond paragraph.\n\nThird paragraph.";
        let doc = parse(input).expect("Failed to parse document");

        assert_paragraph(&doc.get(0), ["First paragraph."].as_ref());
        assert_paragraph(&doc.get(1), ["Second paragraph."].as_ref());
        assert_paragraph(&doc.get(2), ["Third paragraph."].as_ref());
        assert_eq!(doc.len(), 3);
    }
}
