use bevy::{ecs::system::IntoObserverSystem, prelude::*};
use bevy_immediate::{CapSet, ImmCapAccessRequests, ImmCapability, ImmEntity, ImplCap};
use std::marker::PhantomData;

use crate::theme::resources::LucideAssets;
use crate::theme::widgets::icon::LucideIcon;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<LucideAssets>();
    app.init_resource::<ThemeConfig>();
    app.add_observer(play_on_hover_sound_effect);
    app.add_observer(play_on_click_sound_effect);
    app.add_observer(apply_lucide_font);
    app.add_systems(PostUpdate, enforce_lucide_font);
}

/// Configuration resource for the theme system.
///
/// Games should insert this resource to configure interaction sounds.
/// If no sounds are configured, sound effects are silently skipped.
///
/// # Example
/// ```rust,no_run
/// app.insert_resource(ThemeConfig {
///     hover_sound: Some(assets.load("audio/hover.ogg")),
///     click_sound: Some(assets.load("audio/click.ogg")),
/// });
/// ```
#[derive(Resource, Default, Clone)]
pub struct ThemeConfig {
    /// Sound to play when a UI element with `Interaction` is hovered.
    pub hover_sound: Option<Handle<AudioSource>>,
    /// Sound to play when a UI element with `Interaction` is clicked.
    pub click_sound: Option<Handle<AudioSource>>,
}

#[derive(Component)]
pub struct ObserverMarker<T: Send + Sync + 'static>(PhantomData<T>);

impl<T: Send + Sync + 'static> Default for ObserverMarker<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

pub struct PrimaryClick;
pub struct SecondaryClick;
pub struct HoverIn;
pub struct HoverOut;
pub struct RightClick;

pub struct CapabilityObserver;

pub trait ImmUiInteractionExt {
    // Keep the simple API for 99% of cases
    fn on_click_once<M>(self, system: impl IntoObserverSystem<Pointer<Click>, (), M>) -> Self;

    // Add the robust API for advanced cases (multiple listeners)
    fn on_click_tagged<Marker: Send + Sync + 'static, M>(
        self,
        system: impl IntoObserverSystem<Pointer<Click>, (), M>,
    ) -> Self;

    /// Register a handler for right-click (secondary button).
    fn on_right_click<M>(self, system: impl IntoObserverSystem<Pointer<Click>, (), M>) -> Self;

    /// Register a handler for pointer entering this entity.
    fn on_hover<M>(self, system: impl IntoObserverSystem<Pointer<Over>, (), M>) -> Self;

    /// Register a handler for pointer leaving this entity.
    fn on_hover_end<M>(self, system: impl IntoObserverSystem<Pointer<Out>, (), M>) -> Self;
}

fn play_on_hover_sound_effect(
    trigger: On<Pointer<Over>>,
    mut commands: Commands,
    theme_config: Res<ThemeConfig>,
    interaction_query: Query<(), With<Interaction>>,
) {
    if let Some(ref sound) = theme_config.hover_sound {
        if interaction_query.contains(trigger.entity) {
            commands.spawn((
                AudioPlayer(sound.clone()),
                PlaybackSettings::DESPAWN,
            ));
        }
    }
}

fn play_on_click_sound_effect(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    theme_config: Res<ThemeConfig>,
    interaction_query: Query<(), With<Interaction>>,
) {
    if let Some(ref sound) = theme_config.click_sound {
        if interaction_query.contains(trigger.entity) {
            commands.spawn((
                AudioPlayer(sound.clone()),
                PlaybackSettings::DESPAWN,
            ));
        }
    }
}

impl<Cap> ImmUiInteractionExt for ImmEntity<'_, '_, '_, Cap>
where
    Cap: CapSet + ImplCap<CapabilityObserver>,
{
    fn on_click_tagged<Marker: Send + Sync + 'static, M>(
        mut self,
        system: impl IntoObserverSystem<Pointer<Click>, (), M>,
    ) -> Self {
        // Check for the *Specific* marker type
        let has_marker = self
            .cap_get_component::<ObserverMarker<Marker>>()
            .ok()
            .flatten()
            .is_some();

        if !has_marker {
            self.entity_commands()
                .insert(ObserverMarker::<Marker>::default())
                .observe(system);
        }
        self
    }

    // default implementation forwards to the "Primary" tag
    fn on_click_once<M>(self, system: impl IntoObserverSystem<Pointer<Click>, (), M>) -> Self {
        self.on_click_tagged::<PrimaryClick, _>(system)
    }

    fn on_right_click<M>(
        mut self,
        system: impl IntoObserverSystem<Pointer<Click>, (), M>,
    ) -> Self {
        let has_marker = self
            .cap_get_component::<ObserverMarker<RightClick>>()
            .ok()
            .flatten()
            .is_some();

        if !has_marker {
            self.entity_commands()
                .insert(ObserverMarker::<RightClick>::default())
                .observe(system);
        }
        self
    }

    fn on_hover<M>(mut self, system: impl IntoObserverSystem<Pointer<Over>, (), M>) -> Self {
        let has_marker = self
            .cap_get_component::<ObserverMarker<HoverIn>>()
            .ok()
            .flatten()
            .is_some();

        if !has_marker {
            self.entity_commands()
                .insert(ObserverMarker::<HoverIn>::default())
                .observe(system);
        }
        self
    }

    fn on_hover_end<M>(mut self, system: impl IntoObserverSystem<Pointer<Out>, (), M>) -> Self {
        let has_marker = self
            .cap_get_component::<ObserverMarker<HoverOut>>()
            .ok()
            .flatten()
            .is_some();

        if !has_marker {
            self.entity_commands()
                .insert(ObserverMarker::<HoverOut>::default())
                .observe(system);
        }
        self
    }
}

fn apply_lucide_font(
    trigger: On<Add, LucideIcon>,
    mut query: Query<&mut TextFont>,
    lucide_assets: Res<LucideAssets>,
) {
    if let Ok(mut font) = query.get_mut(trigger.entity) {
        font.font = lucide_assets.font.clone();
    }
}

fn enforce_lucide_font(
    mut query: Query<(&mut TextFont, &LucideIcon), Changed<TextFont>>,
    lucide_assets: Res<LucideAssets>,
) {
    for (mut font, _) in &mut query {
        // If the font got reset to something else (like default), force it back.
        if font.font != lucide_assets.font {
            font.font = lucide_assets.font.clone();
        }
    }
}
