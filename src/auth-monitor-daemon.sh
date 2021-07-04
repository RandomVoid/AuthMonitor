#!/bin/sh
. /etc/default/auth-monitor

socket_file="/var/run/auth-monitor.sock"

create_socket() {
  mknod $socket_file p && chmod 666 $socket_file
  if [ $? -ne 0 ]; then
    echo "Error creating socket: $socket_file"
    exit 1
  fi
  echo "Communication socket created: $socket_file"
}

delete_socket() {
  echo "Deleting communication socket: $socket_file"
  rm $socket_file
}

failed_login_count=0

login_succeeded() {
  echo "Authentication succeeded"
  failed_login_count=0
}

login_failed() {
  echo "Authentication failed"
  failed_login_count=$(($failed_login_count+1))
  if [ $failed_login_count -ge $MAX_LOGIN_ATTEMPTS ]; then
    echo "Executing command: $POWER_OFF"
    eval "$POWER_OFF"
  fi
}

terminate() {
  delete_socket
  exit 0
}

trap "terminate" TERM

if [ "$1" = "--test" ]; then
  POWER_OFF="echo $POWER_OFF"
  shift
fi

create_socket

while true; do
  sleep 0.25s
done
