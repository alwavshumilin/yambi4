package phpmailer

import (
	"bufio"
	"fmt"
	"net"
	"strings"
	"time"
)

type POP3 struct {
	Host           string
	Port           int
	TimeoutSeconds int
	Connected      bool
}

func NewPOP3(host string, port int) *POP3 {
	return &POP3{
		Host:           host,
		Port:           port,
		TimeoutSeconds: 30,
		Connected:      false,
	}
}

func (p *POP3) Authorise(userName string, password string, debug bool) error {
	address := fmt.Sprintf("%s:%d", p.Host, p.Port)
	conn, err := net.DialTimeout("tcp", address, time.Duration(p.TimeoutSeconds)*time.Second)
	if err != nil {
		return wrapMailError(ErrorTransport, "pop3 connect failed", err)
	}
	defer conn.Close()

	_ = conn.SetReadDeadline(time.Now().Add(time.Duration(p.TimeoutSeconds) * time.Second))
	_ = conn.SetWriteDeadline(time.Now().Add(time.Duration(p.TimeoutSeconds) * time.Second))

	greeting, err := pop3ReadLine(conn)
	if err != nil {
		return err
	}
	if debug {
		fmt.Printf("POP3 greeting: %s\n", greeting)
	}
	if !strings.HasPrefix(greeting, "+OK") {
		return newMailError(ErrorProtocol, "invalid pop3 greeting: "+greeting)
	}

	if err := pop3Send(conn, "USER "+userName); err != nil {
		return err
	}
	userReply, err := pop3ReadLine(conn)
	if err != nil {
		return err
	}
	if !strings.HasPrefix(userReply, "+OK") {
		return newMailError(ErrorAuthentication, "pop3 USER failed: "+userReply)
	}

	if err := pop3Send(conn, "PASS "+password); err != nil {
		return err
	}
	passReply, err := pop3ReadLine(conn)
	if err != nil {
		return err
	}
	if !strings.HasPrefix(passReply, "+OK") {
		return newMailError(ErrorAuthentication, "pop3 PASS failed: "+passReply)
	}

	_ = pop3Send(conn, "QUIT")
	p.Connected = true
	return nil
}

func pop3Send(conn net.Conn, command string) error {
	_, err := conn.Write([]byte(command + "\r\n"))
	if err != nil {
		return wrapMailError(ErrorTransport, "pop3 write failed", err)
	}

	return nil
}

func pop3ReadLine(conn net.Conn) (string, error) {
	reader := bufio.NewReader(conn)
	line, err := reader.ReadString('\n')
	if err != nil {
		return "", wrapMailError(ErrorTransport, "pop3 read failed", err)
	}

	return strings.TrimRight(line, "\r\n"), nil
}
