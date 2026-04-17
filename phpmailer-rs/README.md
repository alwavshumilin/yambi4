# phpmailer-rs

This directory contains a Rust rewrite scaffold inspired by the structure and behavior of [PHPMailer](https://github.com/PHPMailer/PHPMailer).

## Design goals followed

- Keep naming and flow close to the source project (`PHPMailer`, `SMTP`, `POP3`, OAuth provider).
- Minimize explicit lifetime usage.
- Avoid unsafe code.
- Avoid over-optimization; prefer readable ownership and `.clone()` patterns where they simplify a direct translation style.

## Implemented surface

- `PHPMailer` struct with address management, attachments, `pre_send`, `send`, and SMTP auth paths.
- `SmtpConnection` transport wrapper for basic command/response SMTP flow.
- `Pop3` authorization helper for POP-before-SMTP compatibility behavior.
- `OAuthTokenProvider` trait and a simple token provider implementation.
- Internal Base64 encoder to avoid third-party dependencies in constrained build environments.

## Status

This is a foundational rewrite scaffold intended to stay close to original semantics and naming while remaining idiomatic Rust where practical.
