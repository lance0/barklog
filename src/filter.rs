//! Log filtering with substring and regex support.
//!
//! Provides `ActiveFilter` for real-time log filtering with
//! case-insensitive substring matching or regex patterns.

use regex::Regex;

/// A range representing a match within a line
#[derive(Clone, Copy, Debug)]
pub struct MatchRange {
    pub start: usize,
    pub end: usize,
}

/// A filter that can be applied to log lines
#[derive(Clone)]
pub struct ActiveFilter {
    /// The pattern string
    pub pattern: String,
    /// Whether to treat the pattern as a regex
    pub is_regex: bool,
    /// Compiled regex (if is_regex is true and pattern is valid)
    compiled: Option<Regex>,
    /// Lowercase pattern for case-insensitive substring matching
    pattern_lower: String,
}

impl ActiveFilter {
    pub fn new(pattern: String, is_regex: bool) -> Self {
        let compiled = if is_regex {
            Regex::new(&pattern).ok()
        } else {
            None
        };
        let pattern_lower = pattern.to_lowercase();

        Self {
            pattern,
            is_regex,
            compiled,
            pattern_lower,
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
            line.to_lowercase().contains(&self.pattern_lower)
        }
    }

    /// Find all match ranges in a line
    pub fn find_matches(&self, line: &str) -> Vec<MatchRange> {
        let mut matches = Vec::new();

        if self.is_regex {
            if let Some(ref regex) = self.compiled {
                for m in regex.find_iter(line) {
                    matches.push(MatchRange {
                        start: m.start(),
                        end: m.end(),
                    });
                }
            } else {
                // Invalid regex, fall back to substring
                self.find_substring_matches(line, &mut matches);
            }
        } else {
            self.find_substring_matches(line, &mut matches);
        }

        matches
    }

    /// Find all case-insensitive substring matches
    fn find_substring_matches(&self, line: &str, matches: &mut Vec<MatchRange>) {
        if self.pattern_lower.is_empty() {
            return;
        }

        let line_lower = line.to_lowercase();
        let mut start = 0;

        while let Some(pos) = line_lower[start..].find(&self.pattern_lower) {
            let match_start = start + pos;
            let match_end = match_start + self.pattern.len();
            matches.push(MatchRange {
                start: match_start,
                end: match_end,
            });
            start = match_end;
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

#[cfg(test)]
mod tests {
    use super::*;

    // ActiveFilter::matches() tests

    #[test]
    fn test_substring_match_case_insensitive() {
        let filter = ActiveFilter::new("error".to_string(), false);
        assert!(filter.matches("ERROR: something failed"));
        assert!(filter.matches("Error: something failed"));
        assert!(filter.matches("an error occurred"));
    }

    #[test]
    fn test_substring_no_match() {
        let filter = ActiveFilter::new("error".to_string(), false);
        assert!(!filter.matches("warning: something happened"));
        assert!(!filter.matches("INFO: all good"));
    }

    #[test]
    fn test_regex_match_valid() {
        let filter = ActiveFilter::new(r"ERROR|WARN".to_string(), true);
        assert!(filter.matches("ERROR: something failed"));
        assert!(filter.matches("WARN: something happened"));
        assert!(!filter.matches("INFO: all good"));
    }

    #[test]
    fn test_regex_match_with_pattern() {
        let filter = ActiveFilter::new(r"\d{3}-\d{4}".to_string(), true);
        assert!(filter.matches("Phone: 555-1234"));
        assert!(!filter.matches("Phone: 5551234"));
    }

    #[test]
    fn test_regex_invalid_falls_back_to_substring() {
        // Invalid regex (unclosed bracket)
        let filter = ActiveFilter::new("[invalid".to_string(), true);
        // Should fall back to substring match
        assert!(filter.matches("this has [invalid in it"));
        assert!(!filter.matches("this does not"));
    }

    // ActiveFilter::find_matches() tests

    #[test]
    fn test_find_matches_substring_single() {
        let filter = ActiveFilter::new("error".to_string(), false);
        let matches = filter.find_matches("an ERROR occurred");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].start, 3);
        assert_eq!(matches[0].end, 8);
    }

    #[test]
    fn test_find_matches_substring_multiple() {
        let filter = ActiveFilter::new("test".to_string(), false);
        let matches = filter.find_matches("test one TEST two test");
        assert_eq!(matches.len(), 3);
    }

    #[test]
    fn test_find_matches_regex() {
        let filter = ActiveFilter::new(r"\d+".to_string(), true);
        let matches = filter.find_matches("abc 123 def 456");
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].start, 4);
        assert_eq!(matches[0].end, 7);
        assert_eq!(matches[1].start, 12);
        assert_eq!(matches[1].end, 15);
    }

    #[test]
    fn test_find_matches_empty_pattern() {
        let filter = ActiveFilter::new("".to_string(), false);
        let matches = filter.find_matches("some text");
        assert!(matches.is_empty());
    }

    #[test]
    fn test_find_matches_no_match() {
        let filter = ActiveFilter::new("xyz".to_string(), false);
        let matches = filter.find_matches("abc def");
        assert!(matches.is_empty());
    }

    #[test]
    fn test_find_matches_invalid_regex_fallback() {
        let filter = ActiveFilter::new("[bad".to_string(), true);
        let matches = filter.find_matches("has [bad regex");
        assert_eq!(matches.len(), 1);
    }
}
