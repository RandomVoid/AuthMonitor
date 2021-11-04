# AuthMonitor
Linux shell tool that will power off computer after nth incorrect login attempt. If you use disk encryption, which is strongly recommended, in most cases the password used for disk encryption is much stronger than user password, so this script can be used as additional protection when notebook is lost or stolen. If you don't use disk encryption it won't be useful for you.

## How it works
This is a simple script that can be attached to PAM to log success and failed login attempts. On each failed attempt it checks if maximum number of attempts has been reached. If so it powers off the computer.

This tool has been created which single user environment in mind, so it's not monitoring login attempts separately for different users.

And the most important thing: **DO NOT USE IT ON SERVERS!**

## Installation
Following instruction has been created and tested on Ubuntu 21.04.
1. Run the installation script. It will copy script to `/usr/local/bin/` and configuration to `/etc/default/`:
```
sudo ./install.sh
```
2. Create a backup of PAM configuration file:
```
sudo cp /etc/pam.d/common-auth /etc/pam.d/common-auth.bak
```
3. Open `/etc/pam.d/common-auth` in text editor and change:
```
# here are the per-package modules (the "Primary" block)
auth    [success=1 default=ignore]      pam_unix.so nullok_secure
# here's the fallback if no module succeeds
auth    requisite                       pam_deny.so
# prime the stack with a positive return value if there isn't one already;
# this avoids us returning an error just because nothing sets a success code
# since the modules above will each just jump around
auth    required                        pam_permit.so
# and here are more per-package modules (the "Additional" block)
auth    optional                        pam_cap.so 
# end of pam-auth-update config
```
to
```
# here are the per-package modules (the "Primary" block)
auth    [success=2 default=ignore]      pam_unix.so nullok_secure
# here's the fallback if no module succeeds
auth    optional                        pam_exec.so /usr/local/bin/auth-monitor.sh fail
auth    requisite                       pam_deny.so
# prime the stack with a positive return value if there isn't one already;
# this avoids us returning an error just because nothing sets a success code
# since the modules above will each just jump around
auth    optional                        pam_exec.so /usr/local/bin/auth-monitor.sh success
auth    required                        pam_permit.so
# and here are more per-package modules (the "Additional" block)
auth    optional                        pam_cap.so 
# end of pam-auth-update config
```
This may looks complicated but you have to do only two changes in this file. First change, following line:
```
auth    [success=1 default=ignore]      pam_unix.so nullok_secure
```
must be changed to:
```
auth    [success=2 default=ignore]      pam_unix.so nullok_secure
auth    optional                        pam_exec.so /usr/local/bin/auth-monitor.sh fail
```
This will allow to execute the script on failed login attempt. Second change is adding following line:
```
auth    optional                        pam_exec.so /usr/local/bin/auth-monitor.sh success
```
This instruction will execute script on login success.

## Configuration
Configuration options are in file `/etc/default/auth-monitor`.
```
# Maximum number of login attempts before your computer will be powered off.
# Default: 3 
# By default if you enter your password incorrectly 3 times command POWER_OFF
# will be invoked.
MAX_LOGIN_ATTEMPTS=3

# Command that powers off your computer.
# Default: systemctl poweroff
POWER_OFF="systemctl poweroff"

# File were logs are stored.
# Don't change unless you know what are you doing.
# Default: /var/log/auth-monitor.log
LOG_FILE="/var/log/auth-monitor.log"
```

## Testing
If you want to test script you can run `auth-monitor.sh` with `--test` option. The script won't power off your computer but instead only print executed command on the standard output.

Run command:
```
auth-monitor.sh --test fail
```
Or add option in PAM configuration:
```
auth    optional                        pam_exec.so /usr/local/bin/auth-monitor.sh --test fail
```
You can observe logs by running following command in second console:
```
tail -f /var/log/auth-monitor.log
```

## Troubleshooting
If you can't login to your system after changing PAM configuration, please boot into recovery mode and use root shell to undo changes in `/etc/pam.d/common-auth`. If you created a backup, as it was suggested in the installation instruction, you can simply execute following command:
```
cp /etc/pam.d/common-auth.bak /etc/pam.d/common-auth
```
But in most cases it only requires to redo step 3 from Installation instruction.
Instruction how to boot into recovery mode on Ubuntu: https://wiki.ubuntu.com/RecoveryMode
