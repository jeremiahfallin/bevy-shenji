# RON Save Format Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Switch save files from JSON to RON format with pretty-printing for better Bevy ecosystem alignment.

**Architecture:** Replace all `serde_json` calls in `save.rs` with `ron` equivalents, change file extension from `.json` to `.ron`, remove unused `serde_json` dependency, and clean up the old autosave file.

**Tech Stack:** Rust, Bevy 0.17, `ron = "0.8"`, `serde`

---

### Task 1: Update save serialization to RON

**Files:**
- Modify: `src/game/save.rs:1-4` (imports)
- Modify: `src/game/save.rs:218` (save file extension)
- Modify: `src/game/save.rs:227-233` (serialization)

**Step 1: Replace imports**

In `src/game/save.rs`, remove the `serde_json` usage and add `ron` imports. Replace line 3:

```rust
// REMOVE this line:
// (no serde_json import exists at top — it's used inline)

// ADD after the serde import (line 3):
use ron::ser::PrettyConfig;
```

The full import block (lines 1-4) becomes:

```rust
use bevy::prelude::*;
use bevy::tasks::{IoTaskPool, Task, block_on, poll_once};
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
```

**Step 2: Change save file extension and serializer**

In `save_game` function, change line 218 from:

```rust
let filename = format!("assets/saves/{}.json", message.0);
```

to:

```rust
let filename = format!("assets/saves/{}.ron", message.0);
```

Then change lines 227-233 from:

```rust
match serde_json::to_string_pretty(&save_data) {
    Ok(json) => match std::fs::write(&filename, &json) {
        Ok(()) => Ok(filename),
        Err(e) => Err(format!("Write failed: {}", e)),
    },
    Err(e) => Err(format!("Serialization failed: {}", e)),
}
```

to:

```rust
match ron::ser::to_string_pretty(&save_data, PrettyConfig::default()) {
    Ok(ron_str) => match std::fs::write(&filename, &ron_str) {
        Ok(()) => Ok(filename),
        Err(e) => Err(format!("Write failed: {}", e)),
    },
    Err(e) => Err(format!("Serialization failed: {}", e)),
}
```

**Step 3: Compile and verify**

Run: `cargo check`
Expected: Compiles with no errors. There will be an "unused import: serde_json" warning — that's expected and fixed in Task 2.

**Step 4: Commit**

```bash
git add src/game/save.rs
git commit -m "feat(save): switch save serialization from JSON to RON"
```

---

### Task 2: Update load deserialization to RON

**Files:**
- Modify: `src/game/save.rs:270` (load file extension)
- Modify: `src/game/save.rs:277` (deserialization)

**Step 1: Change load file extension**

In `start_load_game` function, change line 270 from:

```rust
let filename = format!("assets/saves/{}.json", message.0);
```

to:

```rust
let filename = format!("assets/saves/{}.ron", message.0);
```

**Step 2: Change deserializer**

Change line 277 from:

```rust
Ok(json) => match serde_json::from_str::<SaveData>(&json) {
```

to:

```rust
Ok(ron_str) => match ron::from_str::<SaveData>(&ron_str) {
```

Also rename the variable on line 276 for clarity:

```rust
match std::fs::read_to_string(&filename) {
    Ok(ron_str) => match ron::from_str::<SaveData>(&ron_str) {
```

**Step 3: Compile and verify**

Run: `cargo check`
Expected: Compiles cleanly (no more serde_json usage, so no warnings either).

**Step 4: Commit**

```bash
git add src/game/save.rs
git commit -m "feat(save): switch load deserialization from JSON to RON"
```

---

### Task 3: Remove serde_json dependency

**Files:**
- Modify: `Cargo.toml:15` (remove serde_json)

**Step 1: Remove dependency**

Delete line 15 from `Cargo.toml`:

```toml
serde_json = "1.0.148"
```

**Step 2: Compile and verify**

Run: `cargo check`
Expected: Compiles with no errors. `serde_json` is not used anywhere else in the codebase.

**Step 3: Commit**

```bash
git add Cargo.toml
git commit -m "chore: remove unused serde_json dependency"
```

---

### Task 4: Clean up old autosave file

**Files:**
- Delete: `assets/saves/autosave.json`

**Step 1: Delete old save file**

```bash
rm assets/saves/autosave.json
```

**Step 2: Verify**

```bash
ls assets/saves/
```
Expected: Directory is empty (or contains only non-JSON files).

**Step 3: Commit**

```bash
git add assets/saves/autosave.json
git commit -m "chore: remove old JSON autosave file"
```

---

### Task 5: Smoke test

**Step 1: Run the game**

Run: `bevy run`

**Step 2: Verify save/load cycle**

1. Start a new game
2. Wait for autosave (60s) or trigger manual save
3. Check that `assets/saves/autosave.ron` is created
4. Verify the file content looks like RON (struct names, enum variants visible)
5. Quit and reload — verify game loads successfully from the `.ron` file

**Step 3: Final compile check**

Run: `cargo clippy`
Expected: No warnings or errors.
