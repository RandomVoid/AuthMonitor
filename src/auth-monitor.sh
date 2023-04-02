#!/bin/sh

pid_file="${PID_FILE="/var/run/auth-monitor.pid"}"

ensure_pid_file_exists() {
  if [ ! -e "${pid_file}" ]; then
    echo "Auth Monitor daemon not started"
    exit 1
  fi
}

login_failed() {
  ensure_pid_file_exists
  kill -USR2 "$(cat "${pid_file}")"
  exit 0
}

login_succeeded() {
  ensure_pid_file_exists
  kill -USR1 "$(cat "${pid_file}")"
  exit 0
}

case "$1" in
  "fail") login_failed ;;
  "success") login_succeeded ;;
  *)
    echo "Unknown action"
    exit 2
    ;;
esac
