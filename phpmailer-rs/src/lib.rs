mod encoding;
pub mod oauth;
pub mod phpmailer;
pub mod pop3;
pub mod smtp;

pub use oauth::OAuthTokenProvider;
pub use phpmailer::{Attachment, MailError, MessageDate, PHPMailer};
pub use pop3::Pop3;
pub use smtp::{SmtpConnection, SmtpSecurity};
