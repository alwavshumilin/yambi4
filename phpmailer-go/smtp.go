package phpmailer

import (
	"bufio"
	"fmt"
	"net"
	"strings"
	"time"
)

type SMTPSecurity string

const (
	SMTPSecurityNone     SMTPSecurity = "none"
	SMTPSecurityStartTLS SMTPSecurity = "starttls"
	SMTPSecuritySSL      SMTPSecurity = "ssl"
)

type SMTPConnection struct {
	Host           string
	Port           int
	TimeoutSeconds int
	Security       SMTPSecurity
	Conn           net.Conn
	ServerHello    string
}

func NewSMTPConnection(host string, port int) *SMTPConnection {
	return &SMTPConnection{
		Host:           host,
		Port:           port,
		TimeoutSeconds: 300,
		Security:       SMTPSecurityNone,
	}
}

func (s *SMTPConnection) Connect() error {
	address := fmt.Sprintf("%s:%d", s.Host, s.Port)
	conn, err := net.DialTimeout("tcp", address, time.Duration(s.TimeoutSeconds)*time.Second)
	if err != nil {
		return wrapMailError(ErrorTransport, "failed to connect to smtp", err)
	}

	s.Conn = conn
	s.ServerHello, err = s.ReadResponse()
	if err != nil {
		return err
	}

	if !strings.HasPrefix(s.ServerHello, "220") {
		return newMailError(ErrorProtocol, "unexpected smtp greeting: "+s.ServerHello)
	}

	return nil
}

func (s *SMTPConnection) SendCommand(command string) (string, error) {
	if s.Conn == nil {
		return "", newMailError(ErrorTransport, "smtp connection is nil")
	}

	_, err := s.Conn.Write([]byte(command + "\r\n"))
	if err != nil {
		return "", wrapMailError(ErrorTransport, "smtp command write failed", err)
	}

	response, err := s.ReadResponse()
	if err != nil {
		return "", err
	}

	return response, nil
}

func (s *SMTPConnection) ReadResponse() (string, error) {
	if s.Conn == nil {
		return "", newMailError(ErrorTransport, "smtp connection is nil")
	}

	deadline := time.Now().Add(time.Duration(s.TimeoutSeconds) * time.Second)
	_ = s.Conn.SetReadDeadline(deadline)

	reader := bufio.NewReader(s.Conn)
	lines := make([]string, 0)

	for {
		line, err := reader.ReadString('\n')
		if err != nil {
			return "", wrapMailError(ErrorTransport, "smtp read failed", err)
		}

		cleanLine := strings.TrimRight(line, "\r\n")
		lines = append(lines, cleanLine)

		if len(cleanLine) >= 4 && cleanLine[3] == ' ' {
			break
		}
	}

	return strings.Join(lines, "\n"), nil
}

func (s *SMTPConnection) Quit() error {
	if s.Conn == nil {
		return nil
	}

	_, _ = s.SendCommand("QUIT")
	err := s.Conn.Close()
	s.Conn = nil
	if err != nil {
		return wrapMailError(ErrorTransport, "smtp close failed", err)
	}

	return nil
}
