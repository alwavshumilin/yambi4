package phpmailer

import (
	"encoding/base64"
	"fmt"
	"strings"
	"time"
)

type Attachment struct {
	FileName  string
	MediaType string
	Data      []byte
}

type PHPMailer struct {
	From          string
	FromName      string
	To            []string
	CC            []string
	BCC           []string
	ReplyTo       []string
	Subject       string
	Body          string
	AltBody       string
	Attachments   []Attachment
	Mailer        string
	Host          string
	Port          int
	SMTPAuth      bool
	SMTPUsername  string
	SMTPPassword  string
	SMTPKeepAlive bool
	SMTPAutoTLS   bool
	SMTPDebug     int
	Helo          string
	OAuthProvider OAuthTokenProvider
	SMTP          *SMTPConnection
}

func NewPHPMailer() *PHPMailer {
	return &PHPMailer{
		To:            make([]string, 0),
		CC:            make([]string, 0),
		BCC:           make([]string, 0),
		ReplyTo:       make([]string, 0),
		Attachments:   make([]Attachment, 0),
		Mailer:        "mail",
		Host:          "localhost",
		Port:          25,
		SMTPAuth:      false,
		SMTPKeepAlive: false,
		SMTPAutoTLS:   true,
		SMTPDebug:     0,
		Helo:          "localhost",
	}
}

func (m *PHPMailer) IsSMTP() {
	m.Mailer = "smtp"
}

func (m *PHPMailer) SetFrom(address string, name string) {
	m.From = address
	m.FromName = name
}

func (m *PHPMailer) AddAddress(address string) {
	m.To = append(m.To, address)
}

func (m *PHPMailer) AddCC(address string) {
	m.CC = append(m.CC, address)
}

func (m *PHPMailer) AddBCC(address string) {
	m.BCC = append(m.BCC, address)
}

func (m *PHPMailer) AddReplyTo(address string) {
	m.ReplyTo = append(m.ReplyTo, address)
}

func (m *PHPMailer) AddAttachment(attachment Attachment) {
	copyAttachment := Attachment{
		FileName:  attachment.FileName,
		MediaType: attachment.MediaType,
		Data:      append([]byte{}, attachment.Data...),
	}
	m.Attachments = append(m.Attachments, copyAttachment)
}

func (m *PHPMailer) SetOAuthProvider(provider OAuthTokenProvider) {
	m.OAuthProvider = provider
}

func (m *PHPMailer) PreSend() (string, error) {
	if m.From == "" {
		return "", newMailError(ErrorInput, "from address is required")
	}

	if len(m.To) == 0 && len(m.CC) == 0 && len(m.BCC) == 0 {
		return "", newMailError(ErrorInput, "at least one recipient is required")
	}

	headers := make([]string, 0)
	headers = append(headers, fmt.Sprintf("From: %s <%s>", m.FromName, m.From))
	headers = append(headers, fmt.Sprintf("To: %s", strings.Join(append([]string{}, m.To...), ", ")))
	if len(m.CC) > 0 {
		headers = append(headers, fmt.Sprintf("Cc: %s", strings.Join(append([]string{}, m.CC...), ", ")))
	}
	if len(m.ReplyTo) > 0 {
		headers = append(headers, fmt.Sprintf("Reply-To: %s", strings.Join(append([]string{}, m.ReplyTo...), ", ")))
	}
	headers = append(headers, "Subject: "+m.Subject)
	headers = append(headers, "MIME-Version: 1.0")

	if len(m.Attachments) == 0 {
		headers = append(headers, "Content-Type: text/plain; charset=UTF-8")
		return strings.Join(headers, "\r\n") + "\r\n\r\n" + m.Body + "\r\n", nil
	}

	boundary := fmt.Sprintf("b1_%d", time.Now().Unix())
	headers = append(headers, fmt.Sprintf("Content-Type: multipart/mixed; boundary=\"%s\"", boundary))

	builder := strings.Builder{}
	builder.WriteString(strings.Join(headers, "\r\n"))
	builder.WriteString("\r\n\r\n")
	builder.WriteString("--" + boundary + "\r\n")
	builder.WriteString("Content-Type: text/plain; charset=UTF-8\r\n\r\n")
	builder.WriteString(m.Body)
	builder.WriteString("\r\n")

	for _, attachment := range append([]Attachment{}, m.Attachments...) {
		encoded := base64.StdEncoding.EncodeToString(append([]byte{}, attachment.Data...))
		builder.WriteString("--" + boundary + "\r\n")
		builder.WriteString(fmt.Sprintf("Content-Type: %s; name=\"%s\"\r\n", attachment.MediaType, attachment.FileName))
		builder.WriteString("Content-Transfer-Encoding: base64\r\n")
		builder.WriteString(fmt.Sprintf("Content-Disposition: attachment; filename=\"%s\"\r\n\r\n", attachment.FileName))
		builder.WriteString(encoded)
		builder.WriteString("\r\n")
	}

	builder.WriteString("--" + boundary + "--\r\n")
	return builder.String(), nil
}

func (m *PHPMailer) PostSend() error {
	if m.Mailer != "smtp" {
		return nil
	}

	message, err := m.PreSend()
	if err != nil {
		return err
	}

	return m.smtpSend(message)
}

func (m *PHPMailer) Send() error {
	return m.PostSend()
}

func (m *PHPMailer) smtpSend(message string) error {
	smtp := m.SMTP
	if smtp == nil {
		smtp = NewSMTPConnection(m.Host, m.Port)
	}

	if err := smtp.Connect(); err != nil {
		return err
	}

	ehloReply, err := smtp.SendCommand("EHLO " + m.Helo)
	if err != nil {
		return err
	}
	if !strings.HasPrefix(ehloReply, "250") {
		return newMailError(ErrorProtocol, "EHLO failed: "+ehloReply)
	}

	if m.SMTPAuth {
		if err := m.smtpAuthenticate(smtp); err != nil {
			return err
		}
	}

	fromReply, err := smtp.SendCommand("MAIL FROM:<" + m.From + ">")
	if err != nil {
		return err
	}
	if !strings.HasPrefix(fromReply, "250") {
		return newMailError(ErrorProtocol, "MAIL FROM failed: "+fromReply)
	}

	allRecipients := append([]string{}, m.To...)
	allRecipients = append(allRecipients, m.CC...)
	allRecipients = append(allRecipients, m.BCC...)

	for _, recipient := range allRecipients {
		rcptReply, rcptErr := smtp.SendCommand("RCPT TO:<" + recipient + ">")
		if rcptErr != nil {
			return rcptErr
		}
		if !strings.HasPrefix(rcptReply, "250") && !strings.HasPrefix(rcptReply, "251") {
			return newMailError(ErrorProtocol, "RCPT TO failed for "+recipient+": "+rcptReply)
		}
	}

	dataReply, err := smtp.SendCommand("DATA")
	if err != nil {
		return err
	}
	if !strings.HasPrefix(dataReply, "354") {
		return newMailError(ErrorProtocol, "DATA command rejected: "+dataReply)
	}

	normalized := strings.ReplaceAll(message, "\n.", "\n..")
	messageReply, err := smtp.SendCommand(normalized + "\r\n.\r\n")
	if err != nil {
		return err
	}
	if !strings.HasPrefix(messageReply, "250") {
		return newMailError(ErrorProtocol, "message not accepted: "+messageReply)
	}

	if m.SMTPKeepAlive {
		m.SMTP = smtp
	} else {
		_ = smtp.Quit()
		m.SMTP = nil
	}

	return nil
}

func (m *PHPMailer) smtpAuthenticate(smtp *SMTPConnection) error {
	if m.OAuthProvider != nil {
		authReply, err := smtp.SendCommand("AUTH XOAUTH2")
		if err != nil {
			return err
		}
		if !strings.HasPrefix(authReply, "334") {
			return newMailError(ErrorAuthentication, "AUTH XOAUTH2 not accepted: "+authReply)
		}

		tokenReply, err := smtp.SendCommand(m.OAuthProvider.GetOAuth64())
		if err != nil {
			return err
		}
		if !strings.HasPrefix(tokenReply, "235") {
			return newMailError(ErrorAuthentication, "OAuth token rejected: "+tokenReply)
		}

		return nil
	}

	loginReply, err := smtp.SendCommand("AUTH LOGIN")
	if err != nil {
		return err
	}
	if !strings.HasPrefix(loginReply, "334") {
		return newMailError(ErrorAuthentication, "AUTH LOGIN failed to start: "+loginReply)
	}

	user64 := base64.StdEncoding.EncodeToString([]byte(m.SMTPUsername))
	userReply, err := smtp.SendCommand(user64)
	if err != nil {
		return err
	}
	if !strings.HasPrefix(userReply, "334") {
		return newMailError(ErrorAuthentication, "SMTP username rejected: "+userReply)
	}

	password64 := base64.StdEncoding.EncodeToString([]byte(m.SMTPPassword))
	passReply, err := smtp.SendCommand(password64)
	if err != nil {
		return err
	}
	if !strings.HasPrefix(passReply, "235") {
		return newMailError(ErrorAuthentication, "SMTP password rejected: "+passReply)
	}

	return nil
}
