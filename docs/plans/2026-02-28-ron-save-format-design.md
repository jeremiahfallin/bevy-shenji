# RON Save Format Design

**Date**: 2026-02-28
**Status**: Approved

## Goal

Switch the game's save file format from JSON to RON (Rust Object Notation) with pretty-printing. This aligns with Bevy conventions, improves readability with Rust type names, and provides better type safety for enums and tuples.

## Approach

**Approach B: RON with PrettyConfig** — Use `ron::ser::to_string_pretty` with `PrettyConfig::default()` for readable, indented output with struct names and enum variant labels.

## Design

### 1. Serialization Change (`src/game/save.rs`)

- **Saving**: Replace `serde_json::to_string_pretty(&save_data)` with `ron::ser::to_string_pretty(&save_data, PrettyConfig::default())`
- **Loading**: Replace `serde_json::from_str::<SaveData>(&json)` with `ron::from_str::<SaveData>(&content)`
- **File extension**: `.json` -> `.ron` (path pattern becomes `assets/saves/{name}.ron`)

### 2. Dependency Cleanup (`Cargo.toml`)

- Remove `serde_json` from dependencies (only used in `save.rs`)
- `ron = "0.8"` is already present

### 3. Old File Cleanup

- Delete `assets/saves/autosave.json`
- New saves write to `autosave.ron`

### 4. Error Handling

No changes needed — `ron::ser::to_string_pretty` and `ron::from_str` return `Result` types compatible with the existing `Err(format!(...))` pattern.

## Non-Goals

- No backward compatibility with old JSON saves (clean switch)
- No migration tooling
