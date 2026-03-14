---
name: bevy-eyes-on
description: "Capture Bevy visuals with BRP state-gated scenarios first, and timed screenshots only as fallback. Use when a task needs reliable screenshots of specific game states."
---

# Bevy Eyes-On

Use a minimal visual-check workflow for the Shenji Bevy app.

Default to BRP state-gated capture. Use timed capture only when BRP hooks are unavailable.

## Prerequisites

The app must be running with BRP enabled (dev builds only):
```sh
bevy run   # BRP is enabled automatically in dev builds (port 15702)
```

## 1) BRP State-Gated Screenshot (Primary)

```bash
bash .claude/skills/bevy-eyes-on/scripts/capture_after_brp_sequence.sh \
  --requests-jsonl /path/to/scenario.jsonl \
  --brp-url http://127.0.0.1:15702 \
  --settle-ms 250
```

Or with Python (more portable):
```bash
python .claude/skills/bevy-eyes-on/scripts/capture_after_brp_sequence.py \
  --requests-jsonl /path/to/scenario.jsonl \
  --brp-url http://127.0.0.1:15702 \
  --settle-ms 250
```

Use for screenshots of gameplay/submenus/precise states.

`scenario.jsonl` is one JSON object per line:
- action call: raw BRP body, or `{"body": <request>, "wait_ms": <n>}`
- state probe with gate:
`{"probe": <request>, "until": {"path": "result.menu", "equals": "settings", "timeout_ms": 5000, "interval_ms": 100}}`
- `until` supports exactly one matcher: `equals` or `in`.
- `path` uses dot lookup through objects/lists (list index by numeric segment, e.g. `result.items.0.id`).

### Available BRP Methods

Built-in Bevy methods:
- `bevy/query`, `bevy/get`, `bevy/spawn`, `bevy/insert`, `bevy/remove`, `bevy/destroy`, `bevy/list`

Custom Shenji methods:
- `shenji/game_state` — Returns `{"screen": "...", "menu": "...", "active_view": "..."}`.
  Possible screens: `Splash`, `Title`, `NewGame`, `Loading`, `Gameplay`.
  Possible menus: `None`, `Main`, `Settings`, `Credits`, `Pause`.
  Possible views: `Dashboard`, `Research`, `Squads`, `Characters`, `Locations`, `Buildings`.
- `shenji/screenshot` — Takes a screenshot, returns `{"path": "..."}`.

### Example Scenario

```json
{"probe":{"method":"shenji/game_state","params":{}},"until":{"path":"result.screen","equals":"Gameplay","timeout_ms":10000,"interval_ms":200}}
{"probe":{"method":"shenji/game_state","params":{}},"until":{"path":"result.menu","equals":"None","timeout_ms":4000,"interval_ms":100}}
```

## 2) Timed Screenshot (Fallback)

```bash
bash .claude/skills/bevy-eyes-on/scripts/capture_after_delay.sh \
  --delay-seconds 3
```

Use only for non-specific capture when state-driven BRP is unavailable.

## Behavior

- Reuse a running app by default.
- Prefer state-driven BRP scenarios; do not rely on fixed sleeps for state transitions.
- Screenshots are saved to `./screenshots/` with timestamp filenames.
- No loops, no heavyweight run artifacts.

## Output

- Print one absolute screenshot path on success.
- Exit non-zero with a short error on failure.

Run `--help` on either script for full flags.
