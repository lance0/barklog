/// Configuration for bark
pub struct Config {
    /// Maximum number of log lines to keep in the ring buffer
    pub max_lines: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_lines: 10_000,
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        let max_lines = std::env::var("BARK_MAX_LINES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10_000);

        Self { max_lines }
    }
}
