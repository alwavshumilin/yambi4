use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;

use crate::phpmailer::MailError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmtpSecurity {
    None,
    StartTls,
    Ssl,
}

#[derive(Debug)]
pub struct SmtpConnection {
    pub host: String,
    pub port: u16,
    pub timeout_seconds: u64,
    pub security: SmtpSecurity,
    pub stream: Option<TcpStream>,
    pub server_hello: String,
}

impl Clone for SmtpConnection {
    fn clone(&self) -> Self {
        Self {
            host: self.host.clone(),
            port: self.port,
            timeout_seconds: self.timeout_seconds,
            security: self.security.clone(),
            stream: None,
            server_hello: self.server_hello.clone(),
        }
    }
}

impl SmtpConnection {
    pub fn new(host: String, port: u16) -> Self {
        Self {
            host: host.clone(),
            port,
            timeout_seconds: 300,
            security: SmtpSecurity::None,
            stream: None,
            server_hello: String::new(),
        }
    }

    pub fn connect(&mut self) -> Result<(), MailError> {
        let address = format!("{}:{}", self.host.clone(), self.port);
        let stream = TcpStream::connect(address.clone())
            .map_err(|err| MailError::transport(format!("Failed to connect to {address}: {err}")))?;

        stream
            .set_read_timeout(Some(Duration::from_secs(self.timeout_seconds)))
            .map_err(|err| MailError::transport(format!("Failed to set read timeout: {err}")))?;

        stream
            .set_write_timeout(Some(Duration::from_secs(self.timeout_seconds)))
            .map_err(|err| MailError::transport(format!("Failed to set write timeout: {err}")))?;

        self.stream = Some(stream);
        self.server_hello = self.read_response()?;

        if !self.server_hello.starts_with("220") {
            return Err(MailError::protocol(format!(
                "Unexpected SMTP greeting: {}",
                self.server_hello.clone()
            )));
        }

        Ok(())
    }

    pub fn send_command(&mut self, command: String) -> Result<String, MailError> {
        let stream = self
            .stream
            .as_mut()
            .ok_or_else(|| MailError::transport("SMTP stream is not connected".to_string()))?;

        let payload = format!("{}\r\n", command.clone());
        stream
            .write_all(payload.as_bytes())
            .map_err(|err| MailError::transport(format!("Write command failed ({command}): {err}")))?;

        stream
            .flush()
            .map_err(|err| MailError::transport(format!("Flush command failed ({command}): {err}")))?;

        self.read_response()
    }

    pub fn read_response(&mut self) -> Result<String, MailError> {
        let stream = self
            .stream
            .as_mut()
            .ok_or_else(|| MailError::transport("SMTP stream is not connected".to_string()))?;

        let mut reader = BufReader::new(stream);
        let mut response = String::new();
        let mut line = String::new();

        loop {
            line.clear();
            let bytes = reader
                .read_line(&mut line)
                .map_err(|err| MailError::transport(format!("Read response failed: {err}")))?;

            if bytes == 0 {
                break;
            }

            response.push_str(line.clone().as_str());

            let is_last_line = line.as_bytes().get(3).copied() == Some(b' ');
            if line.len() >= 4 && is_last_line {
                break;
            }
        }

        Ok(response.trim_end().to_string())
    }

    pub fn quit(&mut self) -> Result<(), MailError> {
        if self.stream.is_none() {
            return Ok(());
        }

        let _ = self.send_command("QUIT".to_string());
        self.stream = None;

        Ok(())
    }
}
