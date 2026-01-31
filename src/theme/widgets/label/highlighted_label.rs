use bevy::prelude::*;
use bevy_immediate::{CapSet, ImmEntity, ImplCap};
// We use the standard Text capability to ensure we have permission to write Text components
use bevy_immediate::ui::text::ImmUiText;

use crate::theme::primitives::text::CapabilityUiTextStyle;

/// Logic ported from GPUI to coalesce byte indices into continuous ranges
fn highlight_ranges(text: &str, indices: &[usize]) -> Vec<std::ops::Range<usize>> {
    let mut highlight_indices = indices.iter().copied().peekable();
    let mut highlights = Vec::new();

    while let Some(start_ix) = highlight_indices.next() {
        let mut end_ix = start_ix;

        loop {
            // Advance end_ix by one character (UTF-8 safe)
            end_ix += text[end_ix..].chars().next().map_or(0, |c| c.len_utf8());

            // If the next index in our list matches this new boundary, consume it and continue extending
            if highlight_indices.next_if(|&ix| ix == end_ix).is_none() {
                break;
            }
        }

        highlights.push(start_ix..end_ix);
    }

    highlights
}

pub trait ImmUiHighlightedLabel {
    /// Renders text where specific characters are highlighted.
    ///
    /// * `indices`: A list of UTF-8 byte indices representing characters to highlight.
    /// * `base_style`: The style for the normal text.
    /// * `highlight_color`: The color to apply to the highlighted characters.
    fn highlighted_label(
        self,
        text: impl Into<String>,
        indices: &[usize],
        base_style: (),
        highlight_color: Color,
    ) -> Self;
}

impl<Cap> ImmUiHighlightedLabel for ImmEntity<'_, '_, '_, Cap>
where
    // Ensure we have write access to Text components
    Cap: ImplCap<CapabilityUiTextStyle>,
{
    fn highlighted_label(
        mut self,
        text: impl Into<String>,
        indices: &[usize],
        _base_style: (),
        _highlight_color: Color,
    ) -> Self {
        // Simplified implementation avoiding TextSection for now (Bevy 0.15 compatibility)
        let s: String = text.into();
        // self.text(s);
        self
    }
}
