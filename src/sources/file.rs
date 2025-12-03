use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;

use super::{LogEvent, LogSource};
use crate::app::LogLine;
use crate::config::DEFAULT_CHANNEL_BUFFER;

/// A log source that reads from a file using tail -F
pub struct FileSource {
    path: PathBuf,
}

impl FileSource {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

#[async_trait::async_trait]
impl LogSource for FileSource {
    async fn stream(&self) -> mpsc::Receiver<LogEvent> {
        let (tx, rx) = mpsc::channel(DEFAULT_CHANNEL_BUFFER);
        let path = self.path.clone();

        tokio::spawn(async move {
            let result = Command::new("tail")
                .arg("-F")
                .arg(&path)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn();

            match result {
                Ok(mut child) => {
                    if let Some(stdout) = child.stdout.take() {
                        let reader = BufReader::new(stdout);
                        let mut lines = reader.lines();

                        loop {
                            match lines.next_line().await {
                                Ok(Some(line)) => {
                                    if tx.send(LogEvent::Line(LogLine::new(line))).await.is_err() {
                                        break;
                                    }
                                }
                                Ok(None) => {
                                    // EOF - process exited
                                    break;
                                }
                                Err(e) => {
                                    // Report read error (e.g., invalid UTF-8) and continue
                                    let _ = tx
                                        .send(LogEvent::Error(format!("Read error: {}", e)))
                                        .await;
                                    // Continue reading - don't abort on single bad line
                                }
                            }
                        }
                    }

                    // Wait for process to exit
                    match child.wait().await {
                        Ok(status) if !status.success() => {
                            let _ = tx
                                .send(LogEvent::Error(format!(
                                    "tail exited with status: {}",
                                    status
                                )))
                                .await;
                        }
                        Err(e) => {
                            let _ = tx
                                .send(LogEvent::Error(format!("Error waiting for tail: {}", e)))
                                .await;
                        }
                        _ => {}
                    }

                    let _ = tx.send(LogEvent::EndOfStream).await;
                }
                Err(e) => {
                    let _ = tx
                        .send(LogEvent::Error(format!("Failed to spawn tail: {}", e)))
                        .await;
                    let _ = tx.send(LogEvent::EndOfStream).await;
                }
            }
        });

        rx
    }

    fn name(&self) -> String {
        self.path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| self.path.display().to_string())
    }
}
