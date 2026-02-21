//! Lightweight UI transition/animation helpers.
//!
//! Since Bevy 0.17 has no built-in CSS-like transitions for UI nodes, this module
//! provides simple components and systems for interpolating visual properties over time.

use bevy::prelude::*;

/// Animates the alpha of a `BackgroundColor` from `start` to `end` over `duration`.
///
/// Insert this component on a UI entity to trigger a fade animation.
/// The component auto-removes when the animation completes.
#[derive(Component)]
pub struct FadeTransition {
    /// Starting alpha value (0.0 = transparent, 1.0 = opaque).
    pub start_alpha: f32,
    /// Target alpha value.
    pub end_alpha: f32,
    /// Total animation duration.
    pub duration: f32,
    /// Elapsed time so far.
    pub elapsed: f32,
}

impl FadeTransition {
    /// Fade in from transparent.
    pub fn fade_in(duration: f32) -> Self {
        Self {
            start_alpha: 0.0,
            end_alpha: 1.0,
            duration,
            elapsed: 0.0,
        }
    }

    /// Fade out to transparent.
    pub fn fade_out(duration: f32) -> Self {
        Self {
            start_alpha: 1.0,
            end_alpha: 0.0,
            duration,
            elapsed: 0.0,
        }
    }

    /// Custom fade between two alpha values.
    pub fn new(start_alpha: f32, end_alpha: f32, duration: f32) -> Self {
        Self {
            start_alpha,
            end_alpha,
            duration,
            elapsed: 0.0,
        }
    }
}

/// Animates the background color from `start` to `end` over `duration`.
///
/// Insert this component on a UI entity to trigger a color transition.
/// The component auto-removes when the animation completes.
#[derive(Component)]
pub struct ColorTransition {
    /// Starting color.
    pub start: Color,
    /// Target color.
    pub end: Color,
    /// Total animation duration.
    pub duration: f32,
    /// Elapsed time so far.
    pub elapsed: f32,
}

impl ColorTransition {
    pub fn new(start: Color, end: Color, duration: f32) -> Self {
        Self {
            start,
            end,
            duration,
            elapsed: 0.0,
        }
    }
}

/// Easing functions for transitions.
#[derive(Clone, Copy, Debug, Default)]
pub enum Easing {
    /// Linear interpolation (no easing).
    #[default]
    Linear,
    /// Ease in — starts slow, accelerates.
    EaseIn,
    /// Ease out — starts fast, decelerates.
    EaseOut,
    /// Ease in-out — slow start and end.
    EaseInOut,
}

impl Easing {
    /// Apply the easing function to a normalized time `t` in [0, 1].
    pub fn apply(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::EaseIn => t * t,
            Easing::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            Easing::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
        }
    }
}

/// System that ticks `FadeTransition` components and updates `BackgroundColor` alpha.
pub fn tick_fade_transitions(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut FadeTransition, &mut BackgroundColor)>,
) {
    let dt = time.delta_secs();
    for (entity, mut fade, mut bg) in &mut query {
        fade.elapsed += dt;
        let t = (fade.elapsed / fade.duration).clamp(0.0, 1.0);
        let alpha = fade.start_alpha + (fade.end_alpha - fade.start_alpha) * t;
        bg.0.set_alpha(alpha);

        if t >= 1.0 {
            commands.entity(entity).remove::<FadeTransition>();
        }
    }
}

/// System that ticks `ColorTransition` components and updates `BackgroundColor`.
pub fn tick_color_transitions(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ColorTransition, &mut BackgroundColor)>,
) {
    let dt = time.delta_secs();
    for (entity, mut transition, mut bg) in &mut query {
        transition.elapsed += dt;
        let t = (transition.elapsed / transition.duration).clamp(0.0, 1.0);

        // Linear interpolation in sRGB space (good enough for UI)
        let start_linear = transition.start.to_linear();
        let end_linear = transition.end.to_linear();
        let r = start_linear.red + (end_linear.red - start_linear.red) * t;
        let g = start_linear.green + (end_linear.green - start_linear.green) * t;
        let b = start_linear.blue + (end_linear.blue - start_linear.blue) * t;
        let a = start_linear.alpha + (end_linear.alpha - start_linear.alpha) * t;
        bg.0 = Color::LinearRgba(LinearRgba::new(r, g, b, a));

        if t >= 1.0 {
            commands.entity(entity).remove::<ColorTransition>();
        }
    }
}

/// Plugin-style registration function.
///
/// Call `app.add_plugins(transitions::plugin)` or add the systems manually.
pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (tick_fade_transitions, tick_color_transitions),
    );
}
