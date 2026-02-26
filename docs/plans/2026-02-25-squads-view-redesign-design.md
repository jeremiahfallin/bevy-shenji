# Squads View Redesign

**Date:** 2026-02-25
**Status:** Approved

## Summary

Redesign the Squads view to match the Command Dashboard Alpha mockup. The new view features a squad card with status/progress, an agent selection grid, the selected character's job queue, and a persistent event log sidebar.

## Layout

```
+--------------------------------------------------+----------------------+
|  MAIN CONTENT (scrollable)                        |  EVENT LOG SIDEBAR   |
|                                                   |  (~280px fixed)      |
|  +----------------------------------------------+ |                      |
|  | SQUAD CARD                                    | |  "Event Log" header  |
|  | Squad Name  [Active badge]                    | |  timeline entries... |
|  | Operation: <derived from member actions>      | |                      |
|  | Status: <squad status text>                   | |                      |
|  |                                               | |                      |
|  | SELECTED CHARACTER PROGRESS                   | |                      |
|  | [=========>          ] 75%                     | |                      |
|  | Action: Gathering copper                      | |                      |
|  |                                               | |                      |
|  | SELECT AGENT (3-column grid)                  | |                      |
|  | [Rook Lvl4 *] [Kael Lvl3] [Vora Lvl5]       | |                      |
|  +----------------------------------------------+ |                      |
|                                                   |                      |
|  +----------------------------------------------+ |                      |
|  | JOB QUEUE for: [Selected Character]           | |                      |
|  | # | Job Name         | Steps | Active        | |                      |
|  | 1 | Gather copper    | 5     | >             | |                      |
|  | 2 | Mine iron        | 5     |               | |                      |
|  | [+ Add Gather Job] [Clear Jobs]               | |                      |
|  +----------------------------------------------+ |                      |
+--------------------------------------------------+----------------------+
```

## New Data Structures

### EventLog Resource

Persistent log of game events, separate from the temporary NotificationState.

```rust
pub struct EventLogEntry {
    pub message: String,
    pub level: NotificationLevel,
    pub game_tick: u64,
}

pub struct EventLog {
    pub entries: VecDeque<EventLogEntry>,  // newest first, capped at 100
}
```

Entries are pushed alongside existing notifications - when NotificationState::push() is called, the same event is also appended to EventLog.

### Squad Status

Add derived status to Squad:

```rust
pub struct Squad {
    pub name: String,
    pub members: Vec<String>,
    pub status: SquadStatus,
}

pub enum SquadStatus {
    Idle,
    Active,
    Traveling,
}
```

Status is derived from what the majority of squad members are currently doing.

## Visual Styling

Matches the Command Dashboard Alpha mockup dark theme:

- **Squad card**: SURFACE_RAISED bg, BORDER_DEFAULT border, rounded corners (RADIUS_LG)
- **Status badge**: Small pill - blue (Active), gray (Idle), amber (Traveling)
- **Progress bar**: GRAY_700 track, SUCCESS_500 fill, percentage text right-aligned
- **Agent cards**: 3-column grid. Selected: PRIMARY_500 border. Unselected: GRAY_700 border
- **Level badge**: Small pill with PRIMARY_500/20 bg, monospace text
- **Status dot**: Colored circle - green (active), gray (idle), amber (traveling/building)
- **Job queue**: Header row GRAY_900 bg, rows with border-bottom, colored job type badges
- **Event log sidebar**: Timeline dots with left border line, timestamps, color-coded messages

## Character Level

Displayed as average across all 32 skills using existing xp_to_level() formula:
`level = avg(xp_to_level(skill) for each skill)`

## Integration

- SquadsView is a single monolithic ImmediateAttach<CapsUi> component (matches existing patterns)
- Uses InspectorState.selected_character_id for character selection
- Reads ActionState.job_queue for job display
- EventLog resource is global, usable from any view later
- Squad status derived at render time from member ActionState queries

## Files Changed

- `src/game/resources.rs` - Add SquadStatus enum, update Squad struct, add EventLog resource
- `src/game/ui/content/squads.rs` - Complete rewrite of SquadsView
- `src/game/systems.rs` or new file - System to populate EventLog from notifications
