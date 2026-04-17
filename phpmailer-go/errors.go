package phpmailer

import "fmt"

type ErrorKind string

const (
	ErrorTransport      ErrorKind = "transport"
	ErrorProtocol       ErrorKind = "protocol"
	ErrorAuthentication ErrorKind = "authentication"
	ErrorInput          ErrorKind = "input"
)

type MailError struct {
	Kind    ErrorKind
	Message string
}

func (e *MailError) Error() string {
	return e.Message
}

func newMailError(kind ErrorKind, message string) error {
	return &MailError{Kind: kind, Message: message}
}

func wrapMailError(kind ErrorKind, message string, err error) error {
	return &MailError{Kind: kind, Message: fmt.Sprintf("%s: %v", message, err)}
}
