use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;

use crate::phpmailer::MailError;

#[derive(Debug, Clone)]
pub struct Pop3 {
    pub host: String,
    pub port: u16,
    pub timeout_seconds: u64,
    pub connected: bool,
}

impl Pop3 {
    pub fn new(host: String, port: u16) -> Self {
        Self {
            host: host.clone(),
            port,
            timeout_seconds: 30,
            connected: false,
        }
    }

    pub fn authorise(
        &mut self,
        user_name: String,
        password: String,
        debug: bool,
    ) -> Result<(), MailError> {
        let address = format!("{}:{}", self.host.clone(), self.port);
        let mut stream = TcpStream::connect(address.clone())
            .map_err(|err| MailError::transport(format!("POP3 connect failed for {address}: {err}")))?;

        stream
            .set_read_timeout(Some(Duration::from_secs(self.timeout_seconds)))
            .map_err(|err| MailError::transport(format!("POP3 read timeout failed: {err}")))?;

        stream
            .set_write_timeout(Some(Duration::from_secs(self.timeout_seconds)))
            .map_err(|err| MailError::transport(format!("POP3 write timeout failed: {err}")))?;

        let greeting = Self::read_line(&mut stream)?;
        if debug {
            println!("POP3 greeting: {}", greeting.clone());
        }
        if !greeting.starts_with("+OK") {
            return Err(MailError::protocol(format!(
                "Invalid POP3 greeting: {}",
                greeting.clone()
            )));
        }

        Self::send_command(&mut stream, format!("USER {}", user_name.clone()))?;
        let user_reply = Self::read_line(&mut stream)?;
        if !user_reply.starts_with("+OK") {
            return Err(MailError::authentication(format!(
                "POP3 USER failed for {}: {}",
                user_name.clone(),
                user_reply.clone()
            )));
        }

        Self::send_command(&mut stream, format!("PASS {}", password.clone()))?;
        let pass_reply = Self::read_line(&mut stream)?;
        if !pass_reply.starts_with("+OK") {
            return Err(MailError::authentication(format!(
                "POP3 PASS failed for {}: {}",
                user_name.clone(),
                pass_reply.clone()
            )));
        }

        Self::send_command(&mut stream, "QUIT".to_string())?;
        self.connected = true;

        Ok(())
    }

    fn send_command(stream: &mut TcpStream, command: String) -> Result<(), MailError> {
        stream
            .write_all(format!("{}\r\n", command.clone()).as_bytes())
            .map_err(|err| MailError::transport(format!("POP3 command write failed ({command}): {err}")))
    }

    fn read_line(stream: &mut TcpStream) -> Result<String, MailError> {
        let mut reader = BufReader::new(stream);
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .map_err(|err| MailError::transport(format!("POP3 read line failed: {err}")))?;

        Ok(line.trim_end().to_string())
    }
}
