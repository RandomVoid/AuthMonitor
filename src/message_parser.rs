const PAM_PREFIX: &str = "pam_unix";
const PAM_PREFIX_LENGTH: usize = PAM_PREFIX.len();
const AUTH_FAILURE_MESSAGE: &str = "authentication failure";

pub fn is_auth_failed_message(message: &str) -> bool {
    let prefix_position = find_pam_prefix_end(message);
    if prefix_position.is_none() {
        return false;
    }
    let (_, after_prefix_part) = message.split_at(prefix_position.unwrap());
    return after_prefix_part.contains(AUTH_FAILURE_MESSAGE);
}

fn find_pam_prefix_end(message: &str) -> Option<usize> {
    let position = message.find(PAM_PREFIX)?;
    return Some(position + PAM_PREFIX_LENGTH);
}

#[cfg(test)]
#[path = "./message_parser_test.rs"]
mod test;
