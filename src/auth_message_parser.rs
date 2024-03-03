pub struct AuthMessageParser {
    patterns: Vec<AuthFailedMessagePattern>,
}

struct AuthFailedMessagePattern {
    prefix: String,
    message: String,
}

impl AuthMessageParser {
    pub fn new() -> AuthMessageParser {
        let pam_message = AuthFailedMessagePattern {
            prefix: String::from("pam_unix"),
            message: String::from("authentication failure"),
        };
        let unix_chkpwd_message = AuthFailedMessagePattern {
            prefix: String::from("unix_chkpwd"),
            message: String::from("password check failed"),
        };
        return AuthMessageParser {
            patterns: vec![pam_message, unix_chkpwd_message],
        };
    }

    pub fn is_auth_failed_message(&self, message: &str) -> bool {
        for pattern in &self.patterns {
            match message.find(&pattern.prefix) {
                None => {}
                Some(prefix_position) => {
                    let message_after_prefix = &message[prefix_position + pattern.prefix.len()..];
                    if message_after_prefix.contains(&pattern.message) {
                        return true;
                    }
                }
            };
        }
        return false;
    }
}

#[cfg(test)]
#[path = "./auth_message_parser.test.rs"]
mod test;
