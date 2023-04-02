#!/bin/sh

# Maximum number of login attempts before your computer will be powered off.
# Default: 3
# By default if you enter your password incorrectly 3 times command POWER_OFF will be invoked.
max_login_attempts=${MAX_LOGIN_ATTEMPTS=3}

# Command that powers off your computer.
# Default: systemctl poweroff
power_off=${POWER_OFF="systemctl poweroff"}

# Path to the file with daemon PID
# Default: /var/run/auth-monitor.pid
pid_file=${PID_FILE="/var/run/auth-monitor.pid"}

create_pid_file() {
  if [ -e "$pid_file" ]; then
    echo "Daemon already running"
    exit 1
  fi
  echo $$ > "${pid_file}"
  if [ ! -e "$pid_file" ]; then
    echo "Error creating PID file: $pid_file"
    exit 2
  fi
  echo "PID file created: $pid_file"
}

delete_pid_file() {
  echo "Deleting PID file: $pid_file"
  rm "$pid_file"
}

failed_login_count=0

login_succeeded() {
  echo "Authentication succeeded"
  failed_login_count=0
}

login_failed() {
  echo "Authentication failed"
  failed_login_count=$((failed_login_count+1))
  if [ "$failed_login_count" -ge "$max_login_attempts" ]; then
    echo "Executing command: $power_off"
    eval "$power_off"
  fi
}

terminate() {
  delete_pid_file
  exit 0
}

trap "login_succeeded" USR1
trap "login_failed" USR2

trap "terminate" INT
trap "terminate" TERM

create_pid_file

while true; do
  sleep 0.25s
done
