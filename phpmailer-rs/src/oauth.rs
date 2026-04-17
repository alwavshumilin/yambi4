/// Equivalent of PHPMailer's OAuthTokenProvider interface.
pub trait OAuthTokenProvider {
    fn get_oauth64(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct SimpleOAuthTokenProvider {
    user_name: String,
    access_token: String,
}

impl SimpleOAuthTokenProvider {
    pub fn new(user_name: String, access_token: String) -> Self {
        Self {
            user_name: user_name.clone(),
            access_token: access_token.clone(),
        }
    }

    pub fn user_name(&self) -> String {
        self.user_name.clone()
    }

    pub fn access_token(&self) -> String {
        self.access_token.clone()
    }
}

impl OAuthTokenProvider for SimpleOAuthTokenProvider {
    fn get_oauth64(&self) -> String {
        let auth_string = format!(
            "user={}\u{1}auth=Bearer {}\u{1}\u{1}",
            self.user_name.clone(),
            self.access_token.clone()
        );
        crate::encoding::encode_base64(auth_string.into_bytes())
    }
}
