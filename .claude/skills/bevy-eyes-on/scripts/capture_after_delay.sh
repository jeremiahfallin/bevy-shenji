#!/usr/bin/env bash
# Capture a screenshot after a fixed delay.
# Fallback for when BRP state-gated capture is unavailable.
#
# Usage:
#   bash capture_after_delay.sh --delay-seconds 3
#
# Exits 0 and prints the screenshot path on success, non-zero on failure.

set -euo pipefail

BRP_URL="http://127.0.0.1:15702"
DELAY_SECONDS=3

usage() {
  echo "Usage: $0 [--delay-seconds <n>] [--brp-url <url>]"
  exit 1
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --delay-seconds) DELAY_SECONDS="$2"; shift 2 ;;
    --brp-url)       BRP_URL="$2"; shift 2 ;;
    --app)           shift 2 ;;  # ignored; app must already be running
    --help)          usage ;;
    *)               echo "Unknown arg: $1"; usage ;;
  esac
done

sleep "$DELAY_SECONDS"

# Request a screenshot via BRP.
resp=$(curl -sf -X POST "$BRP_URL" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"shenji/screenshot","id":1,"params":{}}' 2>/dev/null)

if [[ -z "$resp" ]]; then
  echo "Error: screenshot request failed — is the app running with BRP enabled?" >&2
  exit 1
fi

screenshot_path=$(echo "$resp" | python3 -c "import sys,json; print(json.load(sys.stdin)['result']['path'])")

# Wait for the file to appear (screenshot is async).
waited=0
while [[ ! -f "$screenshot_path" && $waited -lt 5000 ]]; do
  sleep 0.1
  waited=$((waited + 100))
done

if [[ ! -f "$screenshot_path" ]]; then
  echo "Error: screenshot file did not appear at $screenshot_path" >&2
  exit 1
fi

echo "$screenshot_path"
