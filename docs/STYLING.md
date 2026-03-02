# Styling Reference

Complete reference for the Shenji theme system. Intended for both human developers and AI agents.

```rust
use crate::theme::prelude::*;
```

The theme system lives in `src/theme/` and is fully re-exported via the prelude. UI is built with `bevy_immediate` 0.4 (immediate-mode, trait-based). The core pattern is:

```rust
ui.ch()                          // spawn a child entity, returns ImmEntity
    .flex_col()                  // chainable layout/style methods
    .p(SPACE_4)
    .bg(GRAY_800)
    .add(|ui| {                  // add children
        ui.ch().label("Hello");
    });
```

Use `.style(|n: &mut Node| { ... })` as an escape hatch for raw Bevy `Node` access.

---

## Color Palette

Source: `src/theme/styles/palette.rs`

All colors are `Color` constants.

### Primary

| Name          | RGB                | Usage            |
|---------------|--------------------|------------------|
| `PRIMARY_400` | `(0.3, 0.5, 0.9)` | Hover / light    |
| `PRIMARY_500` | `(0.2, 0.4, 0.8)` | Default primary  |
| `PRIMARY_600` | `(0.15, 0.3, 0.7)` | Pressed / dark  |
| `PRIMARY_700` | `(0.1, 0.22, 0.55)` | Deep accent     |

### Secondary

| Name             | RGB                  |
|------------------|----------------------|
| `SECONDARY_400`  | `(0.55, 0.4, 0.85)` |
| `SECONDARY_500`  | `(0.45, 0.3, 0.75)` |
| `SECONDARY_600`  | `(0.35, 0.22, 0.62)` |

### Accent

| Name          | RGB                  |
|---------------|----------------------|
| `ACCENT_400`  | `(0.0, 0.75, 0.65)` |
| `ACCENT_500`  | `(0.0, 0.65, 0.55)` |
| `ACCENT_600`  | `(0.0, 0.52, 0.45)` |

### Gray Scale

| Name       | Value  | Usage               |
|------------|--------|----------------------|
| `GRAY_50`  | `0.96` | Lightest background  |
| `GRAY_100` | `0.9`  |                      |
| `GRAY_200` | `0.8`  |                      |
| `GRAY_300` | `0.7`  |                      |
| `GRAY_400` | `0.6`  |                      |
| `GRAY_500` | `0.5`  | Mid-tone             |
| `GRAY_600` | `0.35` |                      |
| `GRAY_700` | `0.25` | Panel backgrounds    |
| `GRAY_800` | `0.15` | Sidebar / cards      |
| `GRAY_900` | `0.1`  | Dark backgrounds     |
| `GRAY_950` | `0.05` | Darkest              |

All gray values are uniform RGB (e.g. `GRAY_500` = `(0.5, 0.5, 0.5)`).

### Semantic -- Status

| Name           | RGB                    | Usage         |
|----------------|------------------------|---------------|
| `SUCCESS_400`  | `(0.25, 0.7, 0.35)`   | Light green   |
| `SUCCESS_500`  | `(0.2, 0.6, 0.3)`     | Default green |
| `SUCCESS_600`  | `(0.15, 0.55, 0.25)`  | Dark green    |
| `WARNING_400`  | `(0.95, 0.75, 0.2)`   | Light yellow  |
| `WARNING_500`  | `(0.85, 0.65, 0.15)`  | Default yellow|
| `WARNING_600`  | `(0.75, 0.55, 0.1)`   | Dark yellow   |
| `ERROR_400`    | `(0.85, 0.3, 0.3)`    | Light red     |
| `ERROR_500`    | `(0.75, 0.25, 0.25)`  | Default red   |
| `ERROR_600`    | `(0.7, 0.2, 0.2)`     | Dark red      |
| `INFO_400`     | `(0.3, 0.5, 0.8)`     | Light blue    |
| `INFO_500`     | `(0.25, 0.45, 0.75)`  | Default blue  |
| `INFO_600`     | `(0.2, 0.4, 0.7)`     | Dark blue     |

### Semantic -- Text

| Name               | Value                    | Usage               |
|--------------------|--------------------------|----------------------|
| `TEXT_PRIMARY`      | `White`                  | Primary text         |
| `TEXT_SECONDARY`    | `(0.8, 0.8, 0.8)`       | Secondary text       |
| `TEXT_MUTED`        | `(0.5, 0.5, 0.5)`       | De-emphasized        |
| `TEXT_DISABLED`     | `(0.35, 0.35, 0.35)`    | Disabled controls    |
| `TEXT_PLACEHOLDER`  | `srgba(1, 1, 1, 0.35)`  | Input placeholders   |
| `TEXT_INVERSE`      | `(0.1, 0.1, 0.1)`       | Text on light bg     |

### Semantic -- Surface

| Name              | Value                  | Usage                |
|-------------------|------------------------|----------------------|
| `SURFACE_BASE`    | `(0.08, 0.08, 0.08)`  | App background       |
| `SURFACE_RAISED`  | `(0.12, 0.12, 0.12)`  | Cards / panels       |
| `SURFACE_OVERLAY` | `(0.15, 0.15, 0.15)`  | Modals / dropdowns   |
| `SURFACE_INSET`   | `(0.1, 0.1, 0.1)`     | Wells / inputs       |

### Semantic -- Border

| Name              | Value                  | Usage           |
|-------------------|------------------------|-----------------|
| `BORDER_DEFAULT`  | `(0.25, 0.25, 0.25)`  | Standard border |
| `BORDER_STRONG`   | `(0.35, 0.35, 0.35)`  | Emphasis border |
| `BORDER_MUTED`    | `(0.18, 0.18, 0.18)`  | Subtle border   |

### Overlay

| Name                | Value                   |
|---------------------|-------------------------|
| `OVERLAY_BACKDROP`  | `srgba(0, 0, 0, 0.5)`  |
| `OVERLAY_SUBTLE`    | `srgba(0, 0, 0, 0.2)`  |

### Other

| Name              | Value                        |
|-------------------|------------------------------|
| `GOLD_400`        | Gold, light                  |
| `GOLD_500`        | Gold, default                |
| `GOLD_600`        | Gold, dark                   |
| `TABLE_HEADER_BG` | `srgba(0.1, 0.1, 0.1, 0.5)` |
| `TABLE_STRIPE_BG` | `srgba(1, 1, 1, 0.03)`      |
| `TRANSPARENT`     | `Color::NONE`                |
| `WHITE`           | `Color::WHITE`               |
| `BLACK`           | `Color::BLACK`               |

---

## Spacing

Source: `src/theme/styles/spacing.rs`

4px base unit. All values are `f32` constants in pixels.

| Name       | Pixels |
|------------|--------|
| `SPACE_0`  | 0      |
| `SPACE_0_5`| 2      |
| `SPACE_1`  | 4      |
| `SPACE_1_5`| 6      |
| `SPACE_2`  | 8      |
| `SPACE_2_5`| 10     |
| `SPACE_3`  | 12     |
| `SPACE_4`  | 16     |
| `SPACE_5`  | 20     |
| `SPACE_6`  | 24     |
| `SPACE_8`  | 32     |
| `SPACE_10` | 40     |
| `SPACE_16` | 64     |
| `SPACE_24` | 96     |

---

## Typography

Source: `src/theme/styles/typography.rs`

### Font Sizes

| Name        | Pixels |
|-------------|--------|
| `TEXT_XS`   | 12     |
| `TEXT_SM`   | 14     |
| `TEXT_BASE` | 16     |
| `TEXT_LG`   | 18     |
| `TEXT_XL`   | 20     |
| `TEXT_2XL`  | 24     |
| `TEXT_3XL`  | 30     |
| `TEXT_4XL`  | 48     |
| `TEXT_5XL`  | 64     |

### Line Heights

| Name              | Multiplier |
|-------------------|------------|
| `LEADING_TIGHT`   | 1.25       |
| `LEADING_NORMAL`  | 1.5        |
| `LEADING_RELAXED` | 1.75       |

### Typography Presets

Applied via `.apply()`. See [The `apply()` Pattern](#the-apply-pattern).

| Preset             | Size      | Weight | Color          |
|--------------------|-----------|--------|----------------|
| `style_heading_1`  | `TEXT_3XL`| Bold   | `TEXT_PRIMARY`  |
| `style_heading_2`  | `TEXT_2XL`| Bold   | `TEXT_PRIMARY`  |
| `style_heading_3`  | `TEXT_XL` | Bold   | `TEXT_PRIMARY`  |
| `style_body`       | `TEXT_BASE`| Normal| `TEXT_SECONDARY`|
| `style_body_sm`    | `TEXT_SM` | Normal | `TEXT_SECONDARY`|
| `style_caption`    | `TEXT_XS` | Normal | `TEXT_MUTED`    |
| `style_overline`   | `TEXT_XS` | Bold   | `TEXT_MUTED`    |

```rust
ui.ch().label("Page Title").apply(style_heading_1);
ui.ch().label("Body text here").apply(style_body);
ui.ch().label("SECTION").apply(style_overline);
```

---

## Borders

Source: `src/theme/styles/borders.rs`

### Border Widths

| Name                    | Pixels |
|-------------------------|--------|
| `BORDER_WIDTH_0`        | 0      |
| `BORDER_WIDTH_DEFAULT`  | 1      |
| `BORDER_WIDTH_2`        | 2      |
| `BORDER_WIDTH_3`        | 3      |
| `BORDER_WIDTH_4`        | 4      |

### Border Radii

| Name             | Pixels |
|------------------|--------|
| `RADIUS_NONE`    | 0      |
| `RADIUS_SM`      | 2      |
| `RADIUS_DEFAULT` | 4      |
| `RADIUS_MD`      | 6      |
| `RADIUS_LG`      | 8      |
| `RADIUS_XL`      | 12     |
| `RADIUS_2XL`     | 16     |
| `RADIUS_FULL`    | 9999   |

---

## Layout Primitives

Trait: `ImmUiLayout` on `ImmEntity`. All methods are chainable.

### Display

| Method       | Description              |
|--------------|--------------------------|
| `.flex()`    | Set display to Flex      |
| `.grid()`    | Set display to Grid      |
| `.hidden()`  | Set display to None      |

### Position

| Method        | Description                         |
|---------------|-------------------------------------|
| `.absolute()` | Position absolute                   |
| `.relative()` | Position relative                   |
| `.inset_0()`  | All edges to 0 (fill parent)       |

### Flex Container

| Method          | Description                  |
|-----------------|------------------------------|
| `.flex_row()`   | Flex + row direction         |
| `.flex_col()`   | Flex + column direction      |
| `.flex_wrap()`  | Enable wrapping              |
| `.flex_nowrap()`| Disable wrapping             |

### Flex Item

| Method           | Description                       |
|------------------|-----------------------------------|
| `.flex_1()`      | `flex: 1` (grow and shrink)      |
| `.flex_grow()`   | Allow growing                     |
| `.flex_shrink_0()` | Prevent shrinking              |

### Alignment

| Method               | Description                    |
|----------------------|--------------------------------|
| `.items_center()`    | Align items center             |
| `.items_start()`     | Align items flex-start         |
| `.items_end()`       | Align items flex-end           |
| `.items_stretch()`   | Align items stretch            |
| `.justify_center()`  | Justify content center         |
| `.justify_start()`   | Justify content flex-start     |
| `.justify_end()`     | Justify content flex-end       |
| `.justify_between()` | Justify content space-between  |
| `.justify_around()`  | Justify content space-around   |

### Self Alignment

| Method                | Description                     |
|-----------------------|---------------------------------|
| `.self_center()`      | Align self center               |
| `.self_start()`       | Align self flex-start           |
| `.self_end()`         | Align self flex-end             |
| `.self_stretch()`     | Align self stretch              |
| `.justify_self(val)`  | Set justify-self directly       |

### Sizing

| Method            | Description                         |
|-------------------|-------------------------------------|
| `.w(val)`         | Set width (`impl Into<Val>`)        |
| `.h(val)`         | Set height (`impl Into<Val>`)       |
| `.w_full()`       | Width 100%                          |
| `.h_full()`       | Height 100%                         |
| `.w_percent(f32)` | Width as percentage                 |
| `.h_percent(f32)` | Height as percentage                |
| `.min_w(val)`     | Minimum width                       |
| `.min_h(val)`     | Minimum height                      |
| `.max_w(val)`     | Maximum width                       |
| `.max_h(val)`     | Maximum height                      |

### Padding

All take `impl Into<Val>`.

| Method     | Description         |
|------------|---------------------|
| `.p(val)`  | All sides           |
| `.px(val)` | Horizontal (L + R)  |
| `.py(val)` | Vertical (T + B)    |
| `.pl(val)` | Left                |
| `.pr(val)` | Right               |
| `.pt(val)` | Top                 |
| `.pb(val)` | Bottom              |

### Margin

All take `impl Into<Val>` except `.m_auto()`.

| Method      | Description         |
|-------------|---------------------|
| `.m(val)`   | All sides           |
| `.mx(val)`  | Horizontal (L + R)  |
| `.my(val)`  | Vertical (T + B)    |
| `.ml(val)`  | Left                |
| `.mr(val)`  | Right               |
| `.mt(val)`  | Top                 |
| `.mb(val)`  | Bottom              |
| `.m_auto()` | Auto margin (center)|

### Grid

| Method                          | Description                          |
|---------------------------------|--------------------------------------|
| `.grid_cols(count)`             | Equal-width columns                  |
| `.grid_template_columns(tracks)`| Custom column track definitions      |
| `.col_span(span)`               | Span across columns                  |
| `.gap(f32)`                     | Row and column gap in pixels         |
| `.row_gap(f32)`                 | Row gap only                         |
| `.column_gap(f32)`              | Column gap only                      |

### Overflow

| Method                  | Description                    |
|-------------------------|--------------------------------|
| `.overflow_hidden()`    | Clip both axes                 |
| `.overflow_clip()`      | Clip both axes                 |
| `.overflow_visible()`   | Visible both axes              |
| `.overflow_x_hidden()`  | Clip horizontal                |
| `.overflow_y_hidden()`  | Clip vertical                  |
| `.overflow_x_visible()` | Visible horizontal             |
| `.overflow_y_visible()` | Visible vertical               |

### Scroll

| Method        | Description              |
|---------------|--------------------------|
| `.scroll_y()` | Enable vertical scroll   |
| `.scroll_x()` | Enable horizontal scroll |

### Aspect Ratio

| Method              | Description        |
|---------------------|--------------------|
| `.aspect_ratio(f32)`| Custom ratio       |
| `.aspect_square()`  | 1:1 ratio          |
| `.aspect_video()`   | 16:9 ratio         |

### Position Offsets

All take `impl Into<Val>`.

| Method        | Description      |
|---------------|------------------|
| `.top(val)`   | Top offset       |
| `.bottom(val)`| Bottom offset    |
| `.left(val)`  | Left offset      |
| `.right(val)` | Right offset     |

### Example

```rust
ui.ch()
    .flex_row()
    .items_center()
    .justify_between()
    .w_full()
    .p(SPACE_4)
    .gap(SPACE_2)
    .add(|ui| {
        ui.ch().label("Left");
        ui.ch().label("Right");
    });
```

---

## Visual Primitives

Trait: `ImmUiVisuals` on `ImmEntity`. All methods are chainable.

### Background

| Method       | Description                  |
|--------------|------------------------------|
| `.bg(color)` | Set background color         |

### Border

| Method                | Description                  |
|-----------------------|------------------------------|
| `.border(width_f32)`  | Set border width (all sides) |
| `.border_color(color)`| Set border color             |

### Rounded Corners

| Method           | Description                   |
|------------------|-------------------------------|
| `.rounded(px)`   | Border radius in pixels       |
| `.rounded_md()`  | 6px radius                    |
| `.rounded_full()` | 50% radius (circle/pill)     |

### Opacity

| Method           | Description                         |
|------------------|-------------------------------------|
| `.opacity(f32)`  | Set alpha on BackgroundColor (0..1) |

### Z-Index

| Method                | Description          |
|-----------------------|----------------------|
| `.z_index(i32)`       | Local z-index        |
| `.z_index_global(i32)`| Global z-index       |

### Outline

| Method                    | Description              |
|---------------------------|--------------------------|
| `.outline(width, color)`  | Set outline              |
| `.outline_width(f32)`     | Outline width only       |
| `.outline_color(color)`   | Outline color only       |
| `.outline_offset(f32)`    | Outline offset           |
| `.outline_none()`         | Remove outline           |

### Shadow

| Method                              | Description                        |
|-------------------------------------|------------------------------------|
| `.shadow(x, y, blur, spread, color)`| Custom box shadow                  |
| `.shadow_sm()`                      | Small shadow preset                |
| `.shadow_md()`                      | Medium shadow preset               |
| `.shadow_lg()`                      | Large shadow preset                |
| `.shadow_none()`                    | Remove shadow                      |

### Example

```rust
ui.ch()
    .bg(SURFACE_RAISED)
    .border(BORDER_WIDTH_DEFAULT)
    .border_color(BORDER_DEFAULT)
    .rounded(RADIUS_LG)
    .shadow_md()
    .p(SPACE_4)
    .add(|ui| {
        ui.ch().label("A styled card");
    });
```

---

## Text Primitives

Trait: `ImmUiTextStyleExtension` on `ImmEntity`. All methods are chainable.

### Text Style

| Method                   | Description                     |
|--------------------------|---------------------------------|
| `.text_size(f32)`        | Set font size in pixels         |
| `.text_color(color)`     | Set text color                  |
| `.text_align(Justify)`   | Set text justification          |
| `.text_left()`           | Left-align text                 |
| `.text_center()`         | Center-align text               |
| `.text_right()`          | Right-align text                |
| `.font_bold()`           | Bold weight (placeholder/no-op) |
| `.whitespace_nowrap()`   | Disable line wrapping           |

### Size Shortcuts

| Method        | Size (px) |
|---------------|-----------|
| `.text_xs()`  | 12        |
| `.text_sm()`  | 14        |
| `.text_base()`| 16        |
| `.text_lg()`  | 18        |
| `.text_xl()`  | 20        |
| `.text_2xl()` | 24        |

### Example

```rust
ui.ch()
    .label("Score: 42")
    .text_xl()
    .text_color(ACCENT_500)
    .font_bold()
    .whitespace_nowrap();
```

---

## Image Primitives

Trait: `ImmUiImageExt` on `ImmEntity`. All methods are chainable.

| Method                       | Description              |
|------------------------------|--------------------------|
| `.image(handle: Handle<Image>)` | Set image source      |
| `.image_color(color)`        | Tint the image           |

### Example

```rust
ui.ch()
    .image(asset_server.load("images/icon.png"))
    .w(32.0)
    .h(32.0)
    .image_color(PRIMARY_500);
```

---

## Style Presets

All presets are functions applied via `.apply(preset_name)`. See [The `apply()` Pattern](#the-apply-pattern).

### Container Presets

| Preset                 | Description                                                            |
|------------------------|------------------------------------------------------------------------|
| `style_panel_central`  | `flex_col`, `w_full`, `h_full`, `bg(GRAY_900)`                        |
| `style_sidebar`        | `w(250px)`, `h_full`, `bg(GRAY_800)`                                  |
| `style_bottom_bar`     | `w_full`, `h(50px)`, `bg(GRAY_700)`                                   |
| `style_card`           | `flex_col`, `w(50%)`, `p(SPACE_2_5)`, `rounded(RADIUS_LG)`, `bg(GRAY_700)` |
| `style_card_elevated`  | `flex_col`, `p(SPACE_4)`, `rounded(RADIUS_LG)`, `bg(SURFACE_RAISED)`, `shadow_md` |
| `style_card_outlined`  | `flex_col`, `p(SPACE_4)`, `rounded(RADIUS_LG)`, transparent bg, border |
| `style_modal_overlay`  | `absolute`, `inset_0`, `bg(OVERLAY_BACKDROP)`, centered, `z(100)`     |
| `style_modal_dialog`   | `flex_col`, `p(SPACE_6)`, `rounded(RADIUS_XL)`, `bg(SURFACE_OVERLAY)`, `shadow_lg`, `min_w(320)`, `max_w(560)` |
| `style_tooltip`        | `absolute`, `p(SPACE_2)`, `rounded(RADIUS_DEFAULT)`, `bg(GRAY_800)`, `shadow_sm`, `z(200)` |
| `style_toast`          | `flex_row`, `items_center`, `p(SPACE_4)`, `rounded(RADIUS_LG)`, `bg(SURFACE_RAISED)`, `shadow_md`, bordered |
| `style_well`           | `flex_col`, `p(SPACE_4)`, `rounded(RADIUS_MD)`, `bg(SURFACE_INSET)`   |

```rust
// Elevated card
ui.ch().apply(style_card_elevated).add(|ui| {
    ui.ch().label("Card Title").apply(style_heading_3);
    ui.ch().label("Card body text goes here.").apply(style_body);
});

// Well (inset area)
ui.ch().apply(style_well).add(|ui| {
    ui.ch().label("Inset content").apply(style_caption);
});
```

### Button Presets

All buttons share a base: `h(40px)`, `px(SPACE_4)`, `rounded(6)`, `flex_row`, `items_center`, `justify_center`, `text_sm`, `font_bold`.

| Preset                       | Normal                                    | Hover                                    |
|------------------------------|-------------------------------------------|------------------------------------------|
| `style_btn_primary`          | `bg(PRIMARY_500)`, white text             | `style_btn_primary_hover`: `bg(PRIMARY_600)` |
| `style_btn_secondary`        | `bg(GRAY_700)`, `GRAY_200` text           | `style_btn_secondary_hover`: `bg(GRAY_600)` |
| `style_btn_ghost`            | transparent, `GRAY_200` text              | `style_btn_ghost_hover`: `bg(GRAY_700)` |
| `style_btn_outline`          | transparent, bordered                     | `style_btn_outline_hover`: `bg(GRAY_800)`, `BORDER_STRONG` |
| `style_btn_danger`           | `bg(ERROR_600)`, white text               | `style_btn_danger_hover`: `bg(ERROR_500)` |

| Size Modifier        | Properties                        |
|-----------------------|-----------------------------------|
| `style_btn_sm`        | `h(32px)`, `px(SPACE_3)`, `text_xs` |
| `style_btn_lg`        | `h(48px)`, `px(SPACE_6)`, `text_base` |
| `style_btn_disabled`  | `bg(GRAY_800)`, `TEXT_DISABLED`   |

```rust
// Primary button
ui.ch().apply(style_btn_primary).label("Save");

// Small danger button
ui.ch().apply(style_btn_danger).apply(style_btn_sm).label("Delete");
```

### Grid Preset

| Preset            | Description                                              |
|-------------------|----------------------------------------------------------|
| `style_grid_2col` | Grid with `auto + flex(1)` columns, `row_gap(SPACE_2_5)`, `column_gap(SPACE_5)` |

```rust
ui.ch().apply(style_grid_2col).add(|ui| {
    ui.ch().label("Name:").apply(style_body_sm);
    ui.ch().label("Kael").apply(style_body);
    ui.ch().label("Class:").apply(style_body_sm);
    ui.ch().label("Mage").apply(style_body);
});
```

---

## Widgets

### Label

Trait: `ImmUiLabel`

| Method                   | Description                     |
|--------------------------|---------------------------------|
| `.label(text)`           | Create text node, `text_base`   |
| `.size(LabelSize)`       | Set size variant                |
| `.color(color)`          | Set text color                  |
| `.alpha(f32)`            | Set text alpha                  |
| `.single_line()`         | Prevent wrapping                |
| `.truncate()`            | Truncate overflow               |
| `.weight(FontWeight)`    | Normal, Bold, ExtraBold         |
| `.italic()`              | Italic (no-op currently)        |

`LabelSize` variants: `Small` (12), `Default` (16), `Large` (20), `XLarge` (24).

```rust
ui.ch().label("Character Name").size(LabelSize::Large).color(TEXT_PRIMARY);
ui.ch().label("HP: 100/100").size(LabelSize::Small).color(SUCCESS_500);
```

### Icon

Trait: `ImmUiIconExt`

| Method                          | Description            |
|---------------------------------|------------------------|
| `.icon(lucide_icons::Icon::Name)` | Render a Lucide icon |

```rust
ui.ch().icon(lucide_icons::Icon::Sword).text_color(PRIMARY_500);
```

### Button

Trait: `ImmUiButton`

| Method                             | Description                            |
|------------------------------------|----------------------------------------|
| `.button()`                        | Interactive button, primary style, auto hover |
| `.icon_button()`                   | Button with icon+label layout (column gap)    |
| `.with_label(text)`                | Add text label                         |
| `.with_styled_label(text, style_fn)` | Add label with custom style          |
| `.with_icon(lucide_icon)`          | Add icon to button                     |
| `.disabled(bool)`                  | Gray out + `FocusPolicy::Pass`         |

```rust
// Simple button
ui.ch().button().with_label("Attack");

// Icon button
ui.ch().icon_button()
    .with_icon(lucide_icons::Icon::Shield)
    .with_label("Defend");

// Disabled button
ui.ch().button().with_label("Locked").disabled(true);
```

### Badge

Trait: `ImmUiBadge`

| Method                          | Description                  |
|---------------------------------|------------------------------|
| `.badge(text)`                  | Small colored tag, gray default |
| `.badge_variant(BadgeVariant)`  | Set variant                  |

`BadgeVariant` variants: `Default`, `Primary`, `Success`, `Danger`, `Info`.

```rust
ui.ch().badge("NEW").badge_variant(BadgeVariant::Primary);
ui.ch().badge("CRITICAL").badge_variant(BadgeVariant::Danger);
```

### Divider

Trait: `ImmUiDivider`

| Method                    | Description                |
|---------------------------|----------------------------|
| `.divider()`              | Horizontal 1px line, `GRAY_700` |
| `.divider_vertical()`     | Vertical 1px line          |
| `.divider_color(color)`   | Override divider color     |
| `.divider_thickness(px)`  | Override thickness         |

```rust
ui.ch().label("Section A");
ui.ch().divider();
ui.ch().label("Section B");
```

### ProgressBar

Trait: `ImmUiProgressBar`

| Method                    | Description                        |
|---------------------------|------------------------------------|
| `.progress_bar(0.0..=1.0)`| Horizontal bar, 6px, `PRIMARY_500` fill |
| `.progress_color(color)`  | Override fill color                |
| `.progress_bg(color)`     | Override track color               |
| `.progress_height(px)`    | Override height                    |
| `.progress_rounded(px)`   | Override border radius             |

```rust
ui.ch().progress_bar(0.75).progress_color(SUCCESS_500);
ui.ch().progress_bar(0.3).progress_color(WARNING_500).progress_height(10.0);
```

### Tabs

Traits: `ImmUiTabBar`, `ImmUiTab`

| Method                    | Description                                 |
|---------------------------|---------------------------------------------|
| `.tab_bar(\|ui\| { ... })`| Horizontal tab container with bottom border |
| `.tab(label, active)`     | Single tab, `PRIMARY_500` underline when active |

```rust
ui.ch().tab_bar(|ui| {
    ui.ch().tab("Stats", true);
    ui.ch().tab("Inventory", false);
    ui.ch().tab("Skills", false);
});
```

### Tooltip

Trait: `ImmUiTooltip`

| Method                              | Description                      |
|-------------------------------------|----------------------------------|
| `.tooltip(text)`                    | Absolute positioned, `GRAY_900` bg |
| `.tooltip_position(TooltipPosition)`| Top (default), Bottom, Left, Right |

```rust
ui.ch().button().with_label("?")
    .tooltip("Click for help")
    .tooltip_position(TooltipPosition::Bottom);
```

### Modal

Trait: `ImmUiModal`

| Method                          | Description                      |
|---------------------------------|----------------------------------|
| `.modal_overlay(\|ui\| { ... })`| Fullscreen backdrop, `z(1000)`   |
| `.modal_dialog(\|ui\| { ... })` | Centered dialog card             |
| `.modal_size(ModalSize)`        | Small (320px), Medium (480px), Large (640px) |
| `.modal_header(title)`          | Dialog header                    |
| `.modal_body(\|ui\| { ... })`   | Dialog body                      |
| `.modal_footer(\|ui\| { ... })` | Dialog footer                    |

```rust
ui.ch().modal_overlay(|ui| {
    ui.ch().modal_dialog(|ui| {
        ui.ch().modal_header("Confirm Action");
        ui.ch().modal_body(|ui| {
            ui.ch().label("Are you sure?").apply(style_body);
        });
        ui.ch().modal_footer(|ui| {
            ui.ch().button().with_label("Cancel").apply(style_btn_ghost);
            ui.ch().button().with_label("Confirm");
        });
    }).modal_size(ModalSize::Small);
});
```

### List

Builder struct. Created via `List::new()` or `ui.list()`.

| Method                                    | Description                  |
|-------------------------------------------|------------------------------|
| `.header(text)`                           | Section header               |
| `.empty_message(text)`                    | Shown when no items          |
| `.toggle(Option<bool>)`                   | Collapsible toggle state     |
| `.render(ui, has_items, \|ui\| { ... })`  | Render the list              |

### ListItem

Builder struct. Created via `ListItem::new(id)` or `ui.list_item(id)`.

| Method                          | Description                     |
|---------------------------------|---------------------------------|
| `.spacing(ListItemSpacing)`     | ExtraDense, Dense, Sparse       |
| `.selected(bool)`               | Highlight as selected           |
| `.disabled(bool)`               | Gray out                        |
| `.indent_level(usize)`          | Indentation depth               |
| `.toggle(Option<bool>)`         | Expand/collapse toggle          |
| `.on_toggle(callback)`          | Toggle handler                  |
| `.on_click(callback)`           | Click handler                   |
| `.start_slot(\|ui\| { ... })`   | Left content slot               |
| `.end_slot(\|ui\| { ... })`     | Right content slot              |
| `.render(ui, \|ui\| { ... })`   | Render the item                 |

```rust
List::new()
    .header("Party Members")
    .empty_message("No members yet")
    .render(ui, !members.is_empty(), |ui| {
        for member in &members {
            ListItem::new(member.id)
                .selected(member.id == selected_id)
                .start_slot(|ui| {
                    ui.ch().icon(lucide_icons::Icon::User);
                })
                .end_slot(|ui| {
                    ui.ch().badge("Lv.5").badge_variant(BadgeVariant::Primary);
                })
                .on_click(|_| { /* handle click */ })
                .render(ui, |ui| {
                    ui.ch().label(&member.name);
                });
        }
    });
```

### Table

Builder struct. Created via `Table::new()` or `ui.table()`.

| Method                                  | Description                          |
|-----------------------------------------|--------------------------------------|
| `.column(Column::auto())`               | Auto-sized column                    |
| `.column(Column::flex(val))`            | Flex-sized column                    |
| `.column(Column::px(val))`              | Fixed pixel-width column             |
| `.striped(bool)`                        | Alternating row backgrounds          |
| `.render(ui, \|table\| { ... })`        | Render the table                     |

`TableCtx` methods (inside `.render`):

| Method                          | Description               |
|---------------------------------|---------------------------|
| `.thead(\|row\| { ... })`       | Header row                |
| `.tbody(\|body\| { ... })`      | Table body                |
| `.tr(\|row\| { ... })`          | Data row (inside tbody)   |
| `.tfoot_row(\|row\| { ... })`   | Footer row                |
| `.tfoot(\|ui\| { ... })`        | Free-form footer          |

`RowCtx` methods (inside `.thead`, `.tr`, `.tfoot_row`):

| Method                    | Description         |
|---------------------------|---------------------|
| `.th(\|ui\| { ... })`     | Header cell         |
| `.td(\|ui\| { ... })`     | Data cell           |

```rust
Table::new()
    .column(Column::flex(1.0))
    .column(Column::px(80.0))
    .column(Column::auto())
    .striped(true)
    .render(ui, |table| {
        table.thead(|row| {
            row.th(|ui| { ui.ch().label("Name"); });
            row.th(|ui| { ui.ch().label("Level"); });
            row.th(|ui| { ui.ch().label("Status"); });
        });
        table.tbody(|body| {
            for character in &characters {
                body.tr(|row| {
                    row.td(|ui| { ui.ch().label(&character.name); });
                    row.td(|ui| { ui.ch().label(&format!("{}", character.level)); });
                    row.td(|ui| { ui.ch().badge("Active").badge_variant(BadgeVariant::Success); });
                });
            }
        });
    });
```

---

## Scroll

Trait: `ImmUiScrollExt`

| Method                                      | Description                                    |
|---------------------------------------------|------------------------------------------------|
| `.scrollarea(inner_style_fn, content_fn)`   | Clipping outer container + scrollable inner    |
| `.scroll_view(content_fn)`                  | Shorthand; sets `flex_col` on inner            |

```rust
ui.ch().h(300.0).scroll_view(|ui| {
    for i in 0..50 {
        ui.ch().label(&format!("Item {}", i));
    }
});
```

---

## The `apply()` Pattern

Trait: `ImmUiApply`

```rust
// Apply a preset
ui.ch().apply(style_card_elevated).add(|ui| { ... });

// Apply a typography preset
ui.ch().label("Title").apply(style_heading_2);

// Chain multiple presets
ui.ch().apply(style_btn_primary).apply(style_btn_sm);
```

`.apply(style_fn)` passes the `ImmEntity` through a style function and returns it for further chaining. All style presets in this document are designed to be used with `.apply()`.

Define custom presets:

```rust
fn style_my_panel(e: ImmEntity) -> ImmEntity {
    e.flex_col()
        .p(SPACE_4)
        .gap(SPACE_2)
        .bg(GRAY_800)
        .rounded(RADIUS_LG)
}

// Usage
ui.ch().apply(style_my_panel).add(|ui| { ... });
```

---

## The `style()` Escape Hatch

Trait: `ImmUiStyleExt`

Raw access to Bevy's `Node` component for properties not covered by the fluent API.

```rust
ui.ch()
    .flex_col()
    .style(|n: &mut Node| {
        n.row_gap = Val::Px(4.0);
        n.column_gap = Val::Px(8.0);
    })
    .add(|ui| { ... });
```

Use sparingly. Prefer the chainable API when a method exists.
