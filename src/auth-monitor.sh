#!/bin/sh
. /etc/default/auth-monitor

log() {
  echo "$(date "+%Y-%m-%d %H:%M:%S") $1" >> "$LOG_FILE"
}

login_failed() {
  log "Authentication failed"
  log_tail=$(tail -n $MAX_LOGIN_ATTEMPTS "$LOG_FILE")

  non_fail_count=$(echo "$log_tail" | grep -c -v "failed")
  if [ $non_fail_count -gt 0 ]; then
    exit 0
  fi

  fail_count=$(echo "$log_tail" | grep -c "failed")
  if [ $fail_count -lt $MAX_LOGIN_ATTEMPTS ]; then
    exit 0
  fi

  log "Powering off"
  eval "$POWER_OFF"
  exit 0
}

login_succeeded() {
  log "Authentication succeeded"
  exit 0
}

if [ ! -r "$LOG_FILE" -a -w "$LOG_FILE" ]; then
  echo "Script requires read and write access to file: $LOG_FILE"
  exit 1
fi

if [ "$1" = "--test" ]; then
  POWER_OFF="echo $POWER_OFF"
  shift
fi

case "$1" in
  "fail") login_failed ;;
  "success") login_succeeded ;;
  *)
    echo "Unknown action"
    exit 2
    ;;
esac
