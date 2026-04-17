package phpmailer

import "encoding/base64"

type OAuthTokenProvider interface {
	GetOAuth64() string
}

type SimpleOAuthTokenProvider struct {
	UserName    string
	AccessToken string
}

func NewSimpleOAuthTokenProvider(userName string, accessToken string) *SimpleOAuthTokenProvider {
	return &SimpleOAuthTokenProvider{UserName: userName, AccessToken: accessToken}
}

func (s *SimpleOAuthTokenProvider) GetOAuth64() string {
	payload := "user=" + s.UserName + "\x01auth=Bearer " + s.AccessToken + "\x01\x01"
	return base64.StdEncoding.EncodeToString([]byte(payload))
}
