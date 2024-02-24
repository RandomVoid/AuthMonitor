const PAM_PREFIX: &str = "pam_unix";
const PAM_PREFIX_LENGTH: usize = PAM_PREFIX.len();
const AUTH_FAILURE_MESSAGE: &str = "authentication failure";

pub fn is_auth_failed_message(message: &str) -> bool {
    return match message.find(PAM_PREFIX) {
        None => false,
        Some(pam_prefix_position) => {
            message[pam_prefix_position + PAM_PREFIX_LENGTH..].contains(AUTH_FAILURE_MESSAGE)
        }
    };
}

#[cfg(test)]
#[path = "./auth_message_parser.test.rs"]
mod test;
