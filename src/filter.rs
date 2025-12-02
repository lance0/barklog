use regex::Regex;

/// A filter that can be applied to log lines
#[derive(Clone)]
pub struct ActiveFilter {
    /// The pattern string
    pub pattern: String,
    /// Whether to treat the pattern as a regex
    pub is_regex: bool,
    /// Compiled regex (if is_regex is true and pattern is valid)
    compiled: Option<Regex>,
}

impl ActiveFilter {
    pub fn new(pattern: String, is_regex: bool) -> Self {
        let compiled = if is_regex {
            Regex::new(&pattern).ok()
        } else {
            None
        };

        Self {
            pattern,
            is_regex,
            compiled,
        }
    }

    /// Check if a line matches this filter
    pub fn matches(&self, line: &str) -> bool {
        if self.is_regex {
            if let Some(ref regex) = self.compiled {
                regex.is_match(line)
            } else {
                // Invalid regex, treat as substring match
                line.contains(&self.pattern)
            }
        } else {
            // Case-insensitive substring match
            line.to_lowercase().contains(&self.pattern.to_lowercase())
        }
    }
}

/// A saved filter with a name
#[derive(Clone)]
pub struct SavedFilter {
    pub name: String,
    pub pattern: String,
    pub is_regex: bool,
}
