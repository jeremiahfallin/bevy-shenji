#!/usr/bin/env bash
# Capture a screenshot after driving a Bevy app through a BRP scenario.
#
# Usage:
#   bash capture_after_brp_sequence.sh \
#     --requests-jsonl /path/to/scenario.jsonl \
#     --brp-url http://127.0.0.1:15702 \
#     --settle-ms 250
#
# Each line in scenario.jsonl is one of:
#   Action call:  {"body":{"method":"...","params":{}},"wait_ms":120}
#   State probe:  {"probe":{"method":"...","params":{}},"until":{"path":"result.menu","equals":"pause","timeout_ms":4000,"interval_ms":100}}
#
# Exits 0 and prints the screenshot path on success, non-zero on failure.

set -euo pipefail

BRP_URL="http://127.0.0.1:15702"
REQUESTS_JSONL=""
SETTLE_MS=250
NEXT_ID=1

usage() {
  echo "Usage: $0 --requests-jsonl <file> [--brp-url <url>] [--settle-ms <ms>]"
  exit 1
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --requests-jsonl) REQUESTS_JSONL="$2"; shift 2 ;;
    --brp-url)        BRP_URL="$2"; shift 2 ;;
    --settle-ms)      SETTLE_MS="$2"; shift 2 ;;
    --app)            shift 2 ;;  # ignored on Windows; app must already be running
    --help)           usage ;;
    *)                echo "Unknown arg: $1"; usage ;;
  esac
done

if [[ -z "$REQUESTS_JSONL" ]]; then
  echo "Error: --requests-jsonl is required" >&2
  exit 1
fi

if [[ ! -f "$REQUESTS_JSONL" ]]; then
  echo "Error: file not found: $REQUESTS_JSONL" >&2
  exit 1
fi

# Send a JSON-RPC request and return the result.
brp_call() {
  local method="$1"
  local params="$2"
  local id=$NEXT_ID
  NEXT_ID=$((NEXT_ID + 1))

  local payload
  payload=$(printf '{"jsonrpc":"2.0","method":"%s","id":%d,"params":%s}' "$method" "$id" "$params")

  curl -sf -X POST "$BRP_URL" \
    -H "Content-Type: application/json" \
    -d "$payload" 2>/dev/null
}

# Resolve a dot-path into a JSON value. E.g. "result.menu" on {"result":{"menu":"pause"}}
json_path() {
  local json="$1"
  local path="$2"
  # Convert dot-path to jq path
  local jq_path
  jq_path=$(echo "$path" | sed 's/\./\./g' | sed 's/^/./' | sed 's/\.\([0-9]\+\)/[\1]/g')
  echo "$json" | python3 -c "
import sys, json
data = json.load(sys.stdin)
parts = '$path'.split('.')
val = data
for p in parts:
    if isinstance(val, list):
        val = val[int(p)]
    else:
        val = val[p]
print(json.dumps(val) if not isinstance(val, str) else val)
" 2>/dev/null
}

# Process each line of the JSONL file.
while IFS= read -r line || [[ -n "$line" ]]; do
  # Skip empty lines and comments
  [[ -z "$line" || "$line" == \#* ]] && continue

  # Determine if this is a probe or an action call.
  is_probe=$(echo "$line" | python3 -c "import sys,json; d=json.load(sys.stdin); print('probe' if 'probe' in d else 'action')" 2>/dev/null)

  if [[ "$is_probe" == "probe" ]]; then
    # State probe with polling gate.
    probe_method=$(echo "$line" | python3 -c "import sys,json; print(json.load(sys.stdin)['probe']['method'])")
    probe_params=$(echo "$line" | python3 -c "import sys,json; print(json.dumps(json.load(sys.stdin)['probe'].get('params',{})))")
    until_path=$(echo "$line" | python3 -c "import sys,json; print(json.load(sys.stdin)['until']['path'])")
    timeout_ms=$(echo "$line" | python3 -c "import sys,json; print(json.load(sys.stdin)['until'].get('timeout_ms',5000))")
    interval_ms=$(echo "$line" | python3 -c "import sys,json; print(json.load(sys.stdin)['until'].get('interval_ms',100))")

    # Check for equals or in matcher
    matcher_type=$(echo "$line" | python3 -c "
import sys,json
u=json.load(sys.stdin)['until']
print('equals' if 'equals' in u else 'in' if 'in' in u else 'unknown')
")
    matcher_value=$(echo "$line" | python3 -c "
import sys,json
u=json.load(sys.stdin)['until']
v = u.get('equals', u.get('in', ''))
print(json.dumps(v) if isinstance(v, list) else v)
")

    elapsed=0
    matched=false
    while [[ $elapsed -lt $timeout_ms ]]; do
      resp=$(brp_call "$probe_method" "$probe_params" || echo "")
      if [[ -n "$resp" ]]; then
        actual=$(json_path "$resp" "$until_path" || echo "")
        if [[ "$matcher_type" == "equals" && "$actual" == "$matcher_value" ]]; then
          matched=true
          break
        elif [[ "$matcher_type" == "in" ]]; then
          if echo "$matcher_value" | python3 -c "import sys,json; vals=json.load(sys.stdin); print('yes' if '$actual' in [str(v) for v in vals] else 'no')" 2>/dev/null | grep -q yes; then
            matched=true
            break
          fi
        fi
      fi
      sleep_secs=$(python3 -c "print($interval_ms / 1000.0)")
      sleep "$sleep_secs"
      elapsed=$((elapsed + interval_ms))
    done

    if [[ "$matched" != "true" ]]; then
      echo "Error: probe timed out waiting for $until_path to match" >&2
      exit 1
    fi
  else
    # Action call.
    body_method=$(echo "$line" | python3 -c "import sys,json; d=json.load(sys.stdin); b=d.get('body',d); print(b['method'])")
    body_params=$(echo "$line" | python3 -c "import sys,json; d=json.load(sys.stdin); b=d.get('body',d); print(json.dumps(b.get('params',{})))")
    wait_ms=$(echo "$line" | python3 -c "import sys,json; print(json.load(sys.stdin).get('wait_ms',0))" 2>/dev/null || echo 0)

    brp_call "$body_method" "$body_params" > /dev/null

    if [[ "$wait_ms" -gt 0 ]]; then
      sleep_secs=$(python3 -c "print($wait_ms / 1000.0)")
      sleep "$sleep_secs"
    fi
  fi
done < "$REQUESTS_JSONL"

# Settle before taking the screenshot.
if [[ "$SETTLE_MS" -gt 0 ]]; then
  sleep_secs=$(python3 -c "print($SETTLE_MS / 1000.0)")
  sleep "$sleep_secs"
fi

# Request a screenshot via BRP.
resp=$(brp_call "shenji/screenshot" "{}")
if [[ -z "$resp" ]]; then
  echo "Error: screenshot request failed" >&2
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
