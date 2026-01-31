# **Implementation Plan: "React in Rust" UI Migration**

**Objective:** Complete the transition to the new Fluent API by replacing all ClassList usages and removing bevy\_flair dependencies. This unifies the codebase into a single, type-safe styling system.

## **Phase 1: Define Missing Design Tokens**

We need to create the Rust equivalents for container and layout classes currently hidden in styles.css.

### **Action 1: Create src/theme/styles/containers.rs**

Create a new file to hold structural styles.

* **Define style\_panel\_central**: Replaces .central-panel. Should apply flex column, full width/height, and background colors (e.g., GRAY\_900).  
* **Define style\_sidebar**: Replaces .sidebar. Fixed width (e.g., 250px), full height, distinct background color (e.g., GRAY\_800).  
* **Define style\_bottom\_bar**: Replaces .bottom-bar. Fixed height (e.g., 50px), full width, background color.  
* **Define style\_card**: Replaces .scenario-card or generic cards. Padding, border radius, background color (e.g., GRAY\_700), and potentially a border.

### **Action 2: Create src/theme/styles/grids.rs**

Create a new file for grid layouts used in Settings and Credits screens.

* **Define style\_grid\_2col**: Replaces .credits-grid and .settings-grid.  
  * display: Grid  
  * grid\_template\_columns: e.g., vec\!\[GridTrack::auto(), GridTrack::flex(1.0)\] or similar logic.  
  * row\_gap: e.g., 10.0.  
  * column\_gap: e.g., 20.0.

### **Action 3: Update src/theme/styles/mod.rs**

* Export the new modules (containers and grids) so they are available via crate::theme::prelude::\*.

## **Phase 2: Refactor Widgets**

Ensure all reusable widgets use the new styles internally and expose clean APIs.

### **Action 4: Finalize Button Usage**

* **Target:** src/theme/widgets/button/button.rs  
* **Task:** Ensure the .button() method applies style\_btn\_primary(self).  
* **Check:** Verify no ClassList references remain in this file.

### **Action 5: Finalize Label Usage**

* **Target:** src/theme/widgets/label/label.rs  
* **Task:** Ensure .label() applies default typography styles (using src/theme/primitives/text.rs).  
* **Task:** Implement .header() (if not already done via extension trait) to apply text\_xl(), font\_bold(), etc., replacing any need for .label-header classes.

## **Phase 3: Screen-by-Screen Migration**

Refactor the game screens to use the new traits (.button(), .style(...)) instead of ClassList.

### **Action 6: Refactor Title Screens**

* **File:** src/screens/title/main.rs  
  * **Change:** Replace manual button spawning:  
    * **Old:** ui.ch().on\_spawn\_insert(..., ClassList::new("btn-primary"))  
    * **New:** ui.ch().button().label("New Game").on\_click(on\_new\_game\_button)  
* **File:** src/screens/title/settings.rs  
  * **Change:** Replace ClassList::new("settings-grid") with .style(style\_grid\_2col).  
  * **Change:** Replace button spawning with .button().  
* **File:** src/screens/title/credits.rs  
  * **Change:** Replace ClassList::new("credits-grid") with .style(style\_grid\_2col).

### **Action 7: Refactor Game UI Layout**

* **File:** src/game/ui/layout.rs  
  * **Change:** Apply style\_sidebar to the sidebar node.  
  * **Change:** Apply style\_bottom\_bar to the bottom bar node.  
* **File:** src/game/ui/sidebar.rs  
  * **Change:** Remove ClassList. Ensure headers and labels use the fluent API extensions.

### **Action 8: Refactor New Game Screen**

* **File:** src/screens/new\_game.rs  
  * **Change:** Replace central-panel with .style(style\_panel\_central).  
  * **Change:** Replace scenario-card with .style(style\_card).

## **Phase 4: Cleanup & Dependencies**

Once ClassList is gone from the codebase, remove the bridge.

### **Action 9: Remove bevy\_flair**

* **File:** Cargo.toml \-\> Remove dependency.  
* **File:** src/main.rs \-\> Remove add\_plugins(FlairPlugin) and bevy\_flair imports.  
* **File:** styles.css \-\> Delete the file (it is no longer read).
