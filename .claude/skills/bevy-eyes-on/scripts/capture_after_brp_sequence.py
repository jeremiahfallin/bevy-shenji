#!/usr/bin/env python3
"""
Capture a screenshot after driving a Bevy app through a BRP scenario.

Usage:
    python capture_after_brp_sequence.py \
        --requests-jsonl scenario.jsonl \
        --brp-url http://127.0.0.1:15702 \
        --settle-ms 250

Each line in scenario.jsonl is one of:
    Action call:  {"body":{"method":"...","params":{}},"wait_ms":120}
    State probe:  {"probe":{"method":"...","params":{}},"until":{"path":"result.menu","equals":"pause","timeout_ms":4000,"interval_ms":100}}
"""

import argparse
import json
import os
import sys
import time
import urllib.request
import urllib.error

next_id = 1


def brp_call(url: str, method: str, params: dict) -> dict | None:
    """Send a JSON-RPC request to the BRP endpoint."""
    global next_id
    payload = json.dumps({
        "jsonrpc": "2.0",
        "method": method,
        "id": next_id,
        "params": params,
    }).encode()
    next_id += 1

    req = urllib.request.Request(
        url,
        data=payload,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    try:
        with urllib.request.urlopen(req, timeout=5) as resp:
            return json.loads(resp.read())
    except (urllib.error.URLError, TimeoutError, json.JSONDecodeError):
        return None


def resolve_path(data: dict, path: str):
    """Resolve a dot-separated path through nested dicts/lists."""
    val = data
    for part in path.split("."):
        if isinstance(val, list):
            val = val[int(part)]
        elif isinstance(val, dict):
            val = val[part]
        else:
            return None
    return val


def process_scenario(url: str, jsonl_path: str, settle_ms: int) -> str:
    """Run the scenario and return the screenshot path."""
    with open(jsonl_path, "r") as f:
        lines = f.readlines()

    for line in lines:
        line = line.strip()
        if not line or line.startswith("#"):
            continue

        entry = json.loads(line)

        if "probe" in entry:
            # State probe with polling gate.
            probe = entry["probe"]
            until = entry["until"]
            method = probe["method"]
            params = probe.get("params", {})
            path = until["path"]
            timeout_ms = until.get("timeout_ms", 5000)
            interval_ms = until.get("interval_ms", 100)

            matcher_type = "equals" if "equals" in until else "in" if "in" in until else None
            matcher_value = until.get("equals", until.get("in"))

            elapsed = 0
            matched = False
            while elapsed < timeout_ms:
                resp = brp_call(url, method, params)
                if resp:
                    try:
                        actual = resolve_path(resp, path)
                        if matcher_type == "equals" and str(actual) == str(matcher_value):
                            matched = True
                            break
                        elif matcher_type == "in" and str(actual) in [str(v) for v in matcher_value]:
                            matched = True
                            break
                    except (KeyError, IndexError, TypeError):
                        pass
                time.sleep(interval_ms / 1000.0)
                elapsed += interval_ms

            if not matched:
                print(f"Error: probe timed out waiting for {path} to match", file=sys.stderr)
                sys.exit(1)
        else:
            # Action call.
            body = entry.get("body", entry)
            method = body["method"]
            params = body.get("params", {})
            wait_ms = entry.get("wait_ms", 0)

            brp_call(url, method, params)

            if wait_ms > 0:
                time.sleep(wait_ms / 1000.0)

    # Settle before taking the screenshot.
    if settle_ms > 0:
        time.sleep(settle_ms / 1000.0)

    # Request a screenshot via BRP.
    resp = brp_call(url, "shenji/screenshot", {})
    if not resp or "result" not in resp:
        print("Error: screenshot request failed", file=sys.stderr)
        sys.exit(1)

    screenshot_path = resp["result"]["path"]

    # Wait for the file to appear (screenshot is async).
    waited = 0
    while not os.path.exists(screenshot_path) and waited < 5000:
        time.sleep(0.1)
        waited += 100

    if not os.path.exists(screenshot_path):
        print(f"Error: screenshot file did not appear at {screenshot_path}", file=sys.stderr)
        sys.exit(1)

    return screenshot_path


def main():
    parser = argparse.ArgumentParser(description="Capture screenshot after BRP scenario")
    parser.add_argument("--requests-jsonl", required=True, help="Path to scenario JSONL file")
    parser.add_argument("--brp-url", default="http://127.0.0.1:15702", help="BRP endpoint URL")
    parser.add_argument("--settle-ms", type=int, default=250, help="Settle time before screenshot (ms)")
    parser.add_argument("--app", default=None, help="Ignored (app must be running)")
    args = parser.parse_args()

    path = process_scenario(args.brp_url, args.requests_jsonl, args.settle_ms)
    print(path)


if __name__ == "__main__":
    main()
