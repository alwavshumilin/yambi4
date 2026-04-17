# phpmailer-go

This directory contains a Golang rewrite scaffold inspired by PHPMailer.

## Design choices

- Keep naming and flow close to PHPMailer (`PHPMailer`, SMTP, POP3, OAuth provider).
- Keep code simple and explicit over heavy optimization.
- Use copies and straightforward ownership where it helps a direct rewrite style.
- Avoid unsafe code.
- Keep lock-free and advanced concurrency usage sparse.

## Implemented surface

- `PHPMailer` model with fields and methods for message assembly and SMTP sending.
- `SMTPConnection` helper for command/response SMTP traffic.
- `POP3` helper for POP-before-SMTP authorization behavior.
- OAuth token provider interface.
