#!/bin/sh

if [ "$(id -u)" -ne 0 ]; then
  echo "This script must be run by root."
  exit 1
fi

PROJECT_PATH=$(dirname "$0")

. "$PROJECT_PATH/config/auth-monitor"

CONFIG_PATH=/etc/default/
SCRIPTS_PATH=/usr/local/bin/

die() {
  echo "$@"
  exit 2
}

echo -n "Installing configuration "
cp "$PROJECT_PATH"/config/auth-monitor "$CONFIG_PATH" || die "Error installing configuration"
echo "OK"

echo -n "Installing script "
cp "$PROJECT_PATH"/src/auth-monitor.sh "$SCRIPTS_PATH" || die "Error installing script"
echo "OK"

echo -n "Creating log file: $LOG_FILE "
touch "$LOG_FILE" || die "Error creating log file"
chmod 666 "$LOG_FILE" || die "Error changing log file permisions"
echo "OK"

echo "Installation completed"
