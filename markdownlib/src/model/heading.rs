//! Heading data type.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeadingLevelError(pub u8);

/// A Markdown heading (ATX style) with a level from 1 to 6.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Heading {
    pub level: u8,
    pub text: String,
}

impl Heading {
    pub fn new<L: Into<u8>, S: Into<String>>(level: L, text: S) -> Result<Self, HeadingLevelError> {
        let lvl = level.into();
        if (1..=6).contains(&lvl) {
            Ok(Heading {
                level: lvl,
                text: text.into(),
            })
        } else {
            Err(HeadingLevelError(lvl))
        }
    }
}


impl core::fmt::Display for HeadingLevelError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "invalid heading level: {} (expected 1..=6)", self.0)
    }
}

impl std::error::Error for HeadingLevelError {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validate_levels() {
        assert!(Heading::new(1, "Ok").is_ok());
        assert!(Heading::new(6, "Ok").is_ok());
        assert!(Heading::new(0, "Bad").is_err());
        assert!(Heading::new(7, "Bad").is_err());
    }
}
