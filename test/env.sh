#!/bin/sh
current_path=$(dirname "$0")
export PID_FILE="${current_path}/auth-monitor.pid"
export POWER_OFF="echo POWER OFF"
