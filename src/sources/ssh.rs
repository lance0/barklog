use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;

use super::{LogEvent, LogSource};
use crate::app::LogLine;
use crate::config::{DEFAULT_CHANNEL_BUFFER, DEFAULT_TAIL_LINES};
use async_trait::async_trait;

/// Validate SSH hostname to prevent command injection.
/// Rejects hostnames starting with '-' (option injection) and
/// hostnames with shell metacharacters.
pub fn validate_ssh_host(host: &str) -> Result<(), String> {
    if host.is_empty() {
        return Err("SSH hostname cannot be empty".to_string());
    }

    // Reject hostnames starting with '-' to prevent option injection
    // e.g., -oProxyCommand=... would be interpreted as an SSH option
    if host.starts_with('-') {
        return Err("Invalid SSH hostname: cannot start with '-'".to_string());
    }

    // Allow: alphanumeric, '.', '-', '_', '@' (for user@host), ':' (for port)
    // This is a conservative allowlist to prevent shell injection
    let valid = host.chars().all(|c| {
        c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' || c == '@' || c == ':'
    });

    if !valid {
        return Err(format!(
            "Invalid SSH hostname '{}': contains disallowed characters",
            host
        ));
    }

    Ok(())
}

/// Validate remote path to prevent option injection.
pub fn validate_remote_path(path: &str) -> Result<(), String> {
    if path.is_empty() {
        return Err("Remote path cannot be empty".to_string());
    }

    // Reject paths starting with '-' to prevent option injection to tail
    if path.starts_with('-') {
        return Err("Invalid remote path: cannot start with '-'".to_string());
    }

    Ok(())
}

/// SSH remote file log source
pub struct SshSource {
    /// SSH host (user@host or just host)
    host: String,
    /// Remote file path
    path: String,
    /// SSH host key checking mode
    host_key_checking: String,
}

impl SshSource {
    #[allow(dead_code)]
    pub fn new(host: String, path: String) -> Self {
        Self::with_host_key_checking(host, path, "yes".to_string())
    }

    pub fn with_host_key_checking(host: String, path: String, host_key_checking: String) -> Self {
        Self {
            host,
            path,
            host_key_checking,
        }
    }
}

#[async_trait]
impl LogSource for SshSource {
    async fn stream(&self) -> mpsc::Receiver<LogEvent> {
        let (tx, rx) = mpsc::channel(DEFAULT_CHANNEL_BUFFER);

        let host = self.host.clone();
        let path = self.path.clone();
        let host_key_checking = self.host_key_checking.clone();

        tokio::spawn(async move {
            // Use ssh to run tail -F on the remote host
            let mut cmd = Command::new("ssh");
            cmd.arg("-o")
                .arg("BatchMode=yes") // Disable password prompts
                .arg("-o")
                .arg(format!("StrictHostKeyChecking={}", host_key_checking))
                .arg("--") // Prevent option injection from hostname
                .arg(&host)
                .arg("tail")
                .arg("-F")
                .arg("-n")
                .arg(DEFAULT_TAIL_LINES)
                .arg("--") // Prevent option injection from path
                .arg(&path)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());

            let mut child = match cmd.spawn() {
                Ok(child) => child,
                Err(e) => {
                    let _ = tx
                        .send(LogEvent::Error(format!(
                            "Failed to connect to '{}' for '{}': {}. Check SSH key authentication.",
                            host, path, e
                        )))
                        .await;
                    return;
                }
            };

            let stdout = match child.stdout.take() {
                Some(stdout) => stdout,
                None => {
                    let _ = tx
                        .send(LogEvent::Error("Failed to get stdout".to_string()))
                        .await;
                    return;
                }
            };

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
                        let _ = tx.send(LogEvent::EndOfStream).await;
                        break;
                    }
                    Err(e) => {
                        let _ = tx.send(LogEvent::Error(e.to_string())).await;
                        break;
                    }
                }
            }

            let _ = child.kill().await;
        });

        rx
    }

    fn name(&self) -> String {
        format!("ssh:{}:{}", self.host, self.path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_ssh_host_valid() {
        assert!(validate_ssh_host("example.com").is_ok());
        assert!(validate_ssh_host("user@example.com").is_ok());
        assert!(validate_ssh_host("192.168.1.1").is_ok());
        assert!(validate_ssh_host("host:22").is_ok());
        assert!(validate_ssh_host("user@host.domain.com").is_ok());
        assert!(validate_ssh_host("my-server").is_ok());
        assert!(validate_ssh_host("my_server").is_ok());
    }

    #[test]
    fn test_validate_ssh_host_rejects_dash_prefix() {
        // This is the key security test - reject option injection
        assert!(validate_ssh_host("-oProxyCommand=evil").is_err());
        assert!(validate_ssh_host("-v").is_err());
        assert!(validate_ssh_host("--help").is_err());
    }

    #[test]
    fn test_validate_ssh_host_rejects_empty() {
        assert!(validate_ssh_host("").is_err());
    }

    #[test]
    fn test_validate_ssh_host_rejects_shell_metacharacters() {
        assert!(validate_ssh_host("host;rm -rf /").is_err());
        assert!(validate_ssh_host("host$(evil)").is_err());
        assert!(validate_ssh_host("host`evil`").is_err());
        assert!(validate_ssh_host("host|cat /etc/passwd").is_err());
        assert!(validate_ssh_host("host&background").is_err());
        assert!(validate_ssh_host("host>file").is_err());
        assert!(validate_ssh_host("host<file").is_err());
    }

    #[test]
    fn test_validate_remote_path_valid() {
        assert!(validate_remote_path("/var/log/syslog").is_ok());
        assert!(validate_remote_path("/home/user/app.log").is_ok());
        assert!(validate_remote_path("relative/path.log").is_ok());
    }

    #[test]
    fn test_validate_remote_path_rejects_dash_prefix() {
        // Prevent option injection to tail command
        assert!(validate_remote_path("-n100").is_err());
        assert!(validate_remote_path("--help").is_err());
    }

    #[test]
    fn test_validate_remote_path_rejects_empty() {
        assert!(validate_remote_path("").is_err());
    }
}
