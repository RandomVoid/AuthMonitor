const PAM_PREFIX: &str = "pam_unix";
const AUTH_FAILURE_MESSAGE: &str = "authentication failure";

pub fn is_auth_failed_message(message: &str) -> bool {
    let prefix_position = find_pam_prefix_end(message);
    if prefix_position.is_none() {
        return false;
    }
    let (_, after_prefix_part) = message.split_at(prefix_position.unwrap());
    if after_prefix_part.find(AUTH_FAILURE_MESSAGE).is_none() {
        return false;
    }
    return true;
}

fn find_pam_prefix_end(message: &str) -> Option<usize> {
    let position = message.find(PAM_PREFIX);
    if position.is_none() {
        return None;
    }
    return Some(position.unwrap() + PAM_PREFIX.len());
}

#[cfg(test)]
#[path = "./message_parser_test.rs"]
mod test;
