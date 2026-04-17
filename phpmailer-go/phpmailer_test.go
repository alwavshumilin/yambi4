package phpmailer

import "strings"

import "testing"

func TestPreSendRequiresSenderAndRecipient(t *testing.T) {
	mailer := NewPHPMailer()
	_, err := mailer.PreSend()
	if err == nil {
		t.Fatalf("expected error when sender and recipient are missing")
	}
}

func TestPreSendBuildsPlainMessage(t *testing.T) {
	mailer := NewPHPMailer()
	mailer.SetFrom("sender@example.com", "Sender")
	mailer.AddAddress("receiver@example.com")
	mailer.Subject = "Hello"
	mailer.Body = "Body text"

	payload, err := mailer.PreSend()
	if err != nil {
		t.Fatalf("expected payload, got error: %v", err)
	}

	if !strings.Contains(payload, "From: Sender <sender@example.com>") {
		t.Fatalf("expected from header")
	}
	if !strings.Contains(payload, "To: receiver@example.com") {
		t.Fatalf("expected to header")
	}
	if !strings.Contains(payload, "Subject: Hello") {
		t.Fatalf("expected subject header")
	}
	if !strings.Contains(payload, "Body text") {
		t.Fatalf("expected body text")
	}
}

func TestPreSendBuildsMultipartWithAttachment(t *testing.T) {
	mailer := NewPHPMailer()
	mailer.SetFrom("sender@example.com", "Sender")
	mailer.AddAddress("receiver@example.com")
	mailer.Subject = "Attachment"
	mailer.Body = "Please check attachment"
	mailer.AddAttachment(Attachment{
		FileName:  "example.txt",
		MediaType: "text/plain",
		Data:      []byte("demo"),
	})

	payload, err := mailer.PreSend()
	if err != nil {
		t.Fatalf("expected payload, got error: %v", err)
	}

	if !strings.Contains(payload, "multipart/mixed") {
		t.Fatalf("expected multipart content type")
	}
	if !strings.Contains(payload, "example.txt") {
		t.Fatalf("expected attachment file name")
	}
	if !strings.Contains(payload, "ZGVtbw==") {
		t.Fatalf("expected base64 data")
	}
}
