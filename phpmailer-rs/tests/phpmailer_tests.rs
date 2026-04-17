use phpmailer_rs::{Attachment, PHPMailer};

#[test]
fn pre_send_requires_sender_and_recipient() {
    let mailer = PHPMailer::new();
    let result = mailer.pre_send();

    assert!(result.is_err());
}

#[test]
fn pre_send_builds_plain_message() {
    let mut mailer = PHPMailer::new();
    mailer.set_from("sender@example.com".to_string(), "Sender".to_string());
    mailer.add_address("receiver@example.com".to_string());
    mailer.subject = "Hello".to_string();
    mailer.body = "Body text".to_string();

    let payload = mailer.pre_send().expect("pre_send should build payload");

    assert!(payload.contains("From: Sender <sender@example.com>"));
    assert!(payload.contains("To: receiver@example.com"));
    assert!(payload.contains("Subject: Hello"));
    assert!(payload.contains("Body text"));
}

#[test]
fn pre_send_builds_multipart_with_attachment() {
    let mut mailer = PHPMailer::new();
    mailer.set_from("sender@example.com".to_string(), "Sender".to_string());
    mailer.add_address("receiver@example.com".to_string());
    mailer.subject = "Attachment".to_string();
    mailer.body = "Please check attachment".to_string();
    mailer.add_attachment(Attachment {
        file_name: "example.txt".to_string(),
        media_type: "text/plain".to_string(),
        data: b"demo".to_vec(),
    });

    let payload = mailer.pre_send().expect("pre_send should build multipart payload");

    assert!(payload.contains("multipart/mixed"));
    assert!(payload.contains("example.txt"));
    assert!(payload.contains("ZGVtbw=="));
}
