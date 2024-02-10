#[cfg(test)]
use crate::message_parser::is_auth_failed_message;

#[test]
fn contains_auth_failed_message_success() {
    let messages = [
        "2023-04-22T12:20:32.512681+02:00 workstation sudo: pam_unix(sudo:auth): authentication failure; logname=john uid=1000 euid=0 tty=/dev/pts/7 ruser=john rhost=  user=john",
        "2023-04-22T12:22:53.157054+02:00 workstation kscreenlocker_greet: pam_unix(kde:auth): authentication failure; logname= uid=1000 euid=1000 tty= ruser= rhost=  user=john",
    ];
    for message in messages {
        assert!(is_auth_failed_message(message));
    }
}
