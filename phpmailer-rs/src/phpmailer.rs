use std::fmt::{Display, Formatter};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::oauth::OAuthTokenProvider;
use crate::smtp::SmtpConnection;

#[derive(Debug, Clone)]
pub struct MessageDate {
    pub unix_timestamp: u64,
}

impl MessageDate {
    pub fn now() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or(0);

        Self {
            unix_timestamp: timestamp,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Attachment {
    pub file_name: String,
    pub media_type: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum MailErrorKind {
    Transport,
    Protocol,
    Authentication,
    Input,
}

#[derive(Debug, Clone)]
pub struct MailError {
    pub kind: MailErrorKind,
    pub message: String,
}

impl MailError {
    pub fn transport(message: String) -> Self {
        Self {
            kind: MailErrorKind::Transport,
            message: message.clone(),
        }
    }

    pub fn protocol(message: String) -> Self {
        Self {
            kind: MailErrorKind::Protocol,
            message: message.clone(),
        }
    }

    pub fn authentication(message: String) -> Self {
        Self {
            kind: MailErrorKind::Authentication,
            message: message.clone(),
        }
    }

    pub fn input(message: String) -> Self {
        Self {
            kind: MailErrorKind::Input,
            message: message.clone(),
        }
    }
}

impl Display for MailError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message.clone())
    }
}

impl std::error::Error for MailError {}

pub struct PHPMailer {
    pub from: String,
    pub from_name: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub reply_to: Vec<String>,
    pub subject: String,
    pub body: String,
    pub alt_body: String,
    pub attachments: Vec<Attachment>,
    pub mailer: String,
    pub host: String,
    pub port: u16,
    pub smtp_auth: bool,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_keep_alive: bool,
    pub smtp_auto_tls: bool,
    pub smtp_debug: u8,
    pub helo: String,
    pub oauth_provider: Option<Box<dyn OAuthTokenProvider>>,
    pub smtp: Option<SmtpConnection>,
}

impl Default for PHPMailer {
    fn default() -> Self {
        Self::new()
    }
}

impl PHPMailer {
    pub fn new() -> Self {
        Self {
            from: String::new(),
            from_name: String::new(),
            to: Vec::new(),
            cc: Vec::new(),
            bcc: Vec::new(),
            reply_to: Vec::new(),
            subject: String::new(),
            body: String::new(),
            alt_body: String::new(),
            attachments: Vec::new(),
            mailer: "mail".to_string(),
            host: "localhost".to_string(),
            port: 25,
            smtp_auth: false,
            smtp_username: String::new(),
            smtp_password: String::new(),
            smtp_keep_alive: false,
            smtp_auto_tls: true,
            smtp_debug: 0,
            helo: "localhost".to_string(),
            oauth_provider: None,
            smtp: None,
        }
    }

    pub fn is_smtp(&mut self) {
        self.mailer = "smtp".to_string();
    }

    pub fn set_from(&mut self, address: String, name: String) {
        self.from = address.clone();
        self.from_name = name.clone();
    }

    pub fn add_address(&mut self, address: String) {
        self.to.push(address.clone());
    }

    pub fn add_cc(&mut self, address: String) {
        self.cc.push(address.clone());
    }

    pub fn add_bcc(&mut self, address: String) {
        self.bcc.push(address.clone());
    }

    pub fn add_reply_to(&mut self, address: String) {
        self.reply_to.push(address.clone());
    }

    pub fn add_attachment(&mut self, attachment: Attachment) {
        self.attachments.push(attachment.clone());
    }

    pub fn set_oauth_provider(&mut self, provider: Box<dyn OAuthTokenProvider>) {
        self.oauth_provider = Some(provider);
    }

    pub fn pre_send(&self) -> Result<String, MailError> {
        if self.from.is_empty() {
            return Err(MailError::input("From address is required".to_string()));
        }

        if self.to.is_empty() && self.cc.is_empty() && self.bcc.is_empty() {
            return Err(MailError::input(
                "At least one recipient is required".to_string(),
            ));
        }

        let mut headers = Vec::new();
        headers.push(format!("From: {} <{}>", self.from_name.clone(), self.from.clone()));
        headers.push(format!("To: {}", self.to.clone().join(", ")));

        if !self.cc.is_empty() {
            headers.push(format!("Cc: {}", self.cc.clone().join(", ")));
        }

        if !self.reply_to.is_empty() {
            headers.push(format!("Reply-To: {}", self.reply_to.clone().join(", ")));
        }

        headers.push(format!("Subject: {}", self.subject.clone()));
        headers.push("MIME-Version: 1.0".to_string());

        let boundary = format!("b1_{}", MessageDate::now().unix_timestamp);
        let mut message = String::new();

        if self.attachments.is_empty() {
            headers.push("Content-Type: text/plain; charset=UTF-8".to_string());
            message.push_str(format!("{}\r\n\r\n", headers.join("\r\n")).as_str());
            message.push_str(self.body.clone().as_str());
            message.push_str("\r\n");
        } else {
            headers.push(format!(
                "Content-Type: multipart/mixed; boundary=\"{}\"",
                boundary.clone()
            ));

            message.push_str(format!("{}\r\n\r\n", headers.join("\r\n")).as_str());
            message.push_str(format!("--{}\r\n", boundary.clone()).as_str());
            message.push_str("Content-Type: text/plain; charset=UTF-8\r\n\r\n");
            message.push_str(self.body.clone().as_str());
            message.push_str("\r\n");

            for attachment in self.attachments.clone() {
                let encoded = crate::encoding::encode_base64(attachment.data.clone());
                message.push_str(format!("--{}\r\n", boundary.clone()).as_str());
                message.push_str(
                    format!(
                        "Content-Type: {}; name=\"{}\"\r\n",
                        attachment.media_type.clone(),
                        attachment.file_name.clone()
                    )
                    .as_str(),
                );
                message.push_str("Content-Transfer-Encoding: base64\r\n");
                message.push_str(
                    format!(
                        "Content-Disposition: attachment; filename=\"{}\"\r\n\r\n",
                        attachment.file_name.clone()
                    )
                    .as_str(),
                );
                message.push_str(encoded.as_str());
                message.push_str("\r\n");
            }

            message.push_str(format!("--{}--\r\n", boundary.clone()).as_str());
        }

        Ok(message)
    }

    pub fn post_send(&mut self) -> Result<(), MailError> {
        if self.mailer.clone() != "smtp" {
            return Ok(());
        }

        let message = self.pre_send()?;
        self.smtp_send(message.clone())
    }

    pub fn send(&mut self) -> Result<(), MailError> {
        self.post_send()
    }

    fn smtp_send(&mut self, message: String) -> Result<(), MailError> {
        let mut smtp = if let Some(connection) = self.smtp.clone() {
            connection
        } else {
            SmtpConnection::new(self.host.clone(), self.port)
        };

        smtp.connect()?;

        let ehlo_reply = smtp.send_command(format!("EHLO {}", self.helo.clone()))?;
        if !ehlo_reply.starts_with("250") {
            return Err(MailError::protocol(format!(
                "EHLO failed: {}",
                ehlo_reply.clone()
            )));
        }

        if self.smtp_auth {
            self.smtp_authenticate(&mut smtp)?;
        }

        let from_reply = smtp.send_command(format!("MAIL FROM:<{}>", self.from.clone()))?;
        if !from_reply.starts_with("250") {
            return Err(MailError::protocol(format!(
                "MAIL FROM failed: {}",
                from_reply.clone()
            )));
        }

        let mut all_recipients = self.to.clone();
        all_recipients.extend(self.cc.clone());
        all_recipients.extend(self.bcc.clone());

        for recipient in all_recipients {
            let rcpt_reply = smtp.send_command(format!("RCPT TO:<{}>", recipient.clone()))?;
            if !rcpt_reply.starts_with("250") && !rcpt_reply.starts_with("251") {
                return Err(MailError::protocol(format!(
                    "RCPT TO failed for {}: {}",
                    recipient.clone(),
                    rcpt_reply.clone()
                )));
            }
        }

        let data_reply = smtp.send_command("DATA".to_string())?;
        if !data_reply.starts_with("354") {
            return Err(MailError::protocol(format!(
                "DATA command rejected: {}",
                data_reply.clone()
            )));
        }

        let normalized = message.replace("\n.", "\n..");
        let final_message = format!("{}\r\n.\r\n", normalized.clone());
        let message_reply = smtp.send_command(final_message.clone())?;
        if !message_reply.starts_with("250") {
            return Err(MailError::protocol(format!(
                "Message not accepted: {}",
                message_reply.clone()
            )));
        }

        if self.smtp_keep_alive {
            self.smtp = Some(smtp.clone());
        } else {
            smtp.quit()?;
            self.smtp = None;
        }

        Ok(())
    }

    fn smtp_authenticate(&self, smtp: &mut SmtpConnection) -> Result<(), MailError> {
        if let Some(provider) = self.oauth_provider.as_ref() {
            let auth_reply = smtp.send_command("AUTH XOAUTH2".to_string())?;
            if !auth_reply.starts_with("334") {
                return Err(MailError::authentication(format!(
                    "AUTH XOAUTH2 not accepted: {}",
                    auth_reply.clone()
                )));
            }

            let token_reply = smtp.send_command(provider.get_oauth64())?;
            if !token_reply.starts_with("235") {
                return Err(MailError::authentication(format!(
                    "OAuth token rejected: {}",
                    token_reply.clone()
                )));
            }

            return Ok(());
        }

        let login_reply = smtp.send_command("AUTH LOGIN".to_string())?;
        if !login_reply.starts_with("334") {
            return Err(MailError::authentication(format!(
                "AUTH LOGIN failed to start: {}",
                login_reply.clone()
            )));
        }

        let user64 = crate::encoding::encode_base64(self.smtp_username.clone().into_bytes());
        let user_reply = smtp.send_command(user64.clone())?;
        if !user_reply.starts_with("334") {
            return Err(MailError::authentication(format!(
                "SMTP username rejected: {}",
                user_reply.clone()
            )));
        }

        let password64 = crate::encoding::encode_base64(self.smtp_password.clone().into_bytes());
        let pass_reply = smtp.send_command(password64.clone())?;
        if !pass_reply.starts_with("235") {
            return Err(MailError::authentication(format!(
                "SMTP password rejected: {}",
                pass_reply.clone()
            )));
        }

        Ok(())
    }
}
