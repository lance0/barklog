#!/bin/bash
# Generates sample logs with ANSI colors to test bark
# Usage: ./scripts/gen_logs.sh > /tmp/test.log &
# Then run: cargo run -- /tmp/test.log

while true; do
    level=$((RANDOM % 4))
    case $level in
        0) echo -e "\033[32m[INFO]\033[0m $(date +%H:%M:%S) Application running normally" ;;
        1) echo -e "\033[33m[WARN]\033[0m $(date +%H:%M:%S) Memory usage at 75%" ;;
        2) echo -e "\033[31m[ERROR]\033[0m $(date +%H:%M:%S) Connection timeout to database" ;;
        3) echo -e "\033[34m[DEBUG]\033[0m $(date +%H:%M:%S) Processing request id=$RANDOM" ;;
    esac
    sleep 0.5
done
