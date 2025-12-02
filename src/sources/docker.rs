use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;

use super::{LogEvent, LogSource};
use crate::app::LogLine;
use crate::config::{DEFAULT_CHANNEL_BUFFER, DEFAULT_TAIL_LINES};

/// A log source that reads from a Docker container using docker logs -f
pub struct DockerSource {
    container: String,
}

impl DockerSource {
    pub fn new(container: String) -> Self {
        Self { container }
    }
}

#[async_trait::async_trait]
impl LogSource for DockerSource {
    async fn stream(&self) -> mpsc::Receiver<LogEvent> {
        let (tx, rx) = mpsc::channel(DEFAULT_CHANNEL_BUFFER);
        let container = self.container.clone();

        tokio::spawn(async move {
            let result = Command::new("docker")
                .arg("logs")
                .arg("-f")
                .arg("--tail")
                .arg(DEFAULT_TAIL_LINES)
                .arg(&container)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn();

            match result {
                Ok(mut child) => {
                    // Docker logs outputs to both stdout and stderr
                    // For simplicity, we'll merge them by reading stdout first
                    if let Some(stdout) = child.stdout.take() {
                        let reader = BufReader::new(stdout);
                        let mut lines = reader.lines();

                        while let Ok(Some(line)) = lines.next_line().await {
                            if tx.send(LogEvent::Line(LogLine::new(line))).await.is_err() {
                                break;
                            }
                        }
                    }

                    match child.wait().await {
                        Ok(status) if !status.success() => {
                            let _ = tx
                                .send(LogEvent::Error(format!(
                                    "docker logs exited with status: {}",
                                    status
                                )))
                                .await;
                        }
                        Err(e) => {
                            let _ = tx
                                .send(LogEvent::Error(format!(
                                    "Error waiting for docker logs: {}",
                                    e
                                )))
                                .await;
                        }
                        _ => {}
                    }

                    let _ = tx.send(LogEvent::EndOfStream).await;
                }
                Err(e) => {
                    let _ = tx
                        .send(LogEvent::Error(format!(
                            "Failed to start docker logs for '{}': {}. Is Docker installed and running?",
                            container, e
                        )))
                        .await;
                    let _ = tx.send(LogEvent::EndOfStream).await;
                }
            }
        });

        rx
    }

    fn name(&self) -> String {
        self.container.clone()
    }
}
