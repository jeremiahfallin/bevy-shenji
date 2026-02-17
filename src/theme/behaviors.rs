use bevy::{ecs::system::IntoObserverSystem, prelude::*};
use bevy_immediate::{CapSet, ImmCapAccessRequests, ImmCapability, ImmEntity, ImplCap};
use std::marker::PhantomData;

use crate::theme::resources::LucideAssets;
use crate::theme::widgets::icon::LucideIcon;
use crate::{asset_tracking::LoadResource, audio::sound_effect};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<LucideAssets>();
    app.load_resource::<InteractionAssets>();
    app.add_observer(play_on_hover_sound_effect);
    app.add_observer(play_on_click_sound_effect);
    app.add_observer(apply_lucide_font);
    app.add_systems(PostUpdate, enforce_lucide_font);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct InteractionAssets {
    #[dependency]
    hover: Handle<AudioSource>,
    #[dependency]
    click: Handle<AudioSource>,
}

impl FromWorld for InteractionAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            hover: assets.load("audio/sound_effects/button_hover.ogg"),
            click: assets.load("audio/sound_effects/button_click.ogg"),
        }
    }
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
pub struct HoverSound;

pub struct CapabilityObserver;

pub trait ImmUiInteractionExt {
    // Keep the simple API for 99% of cases
    fn on_click_once<M>(self, system: impl IntoObserverSystem<Pointer<Click>, (), M>) -> Self;

    // Add the robust API for advanced cases (multiple listeners)
    fn on_click_tagged<Marker: Send + Sync + 'static, M>(
        self,
        system: impl IntoObserverSystem<Pointer<Click>, (), M>,
    ) -> Self;
}

fn play_on_hover_sound_effect(
    trigger: On<Pointer<Over>>,
    mut commands: Commands,
    interaction_assets: Option<Res<InteractionAssets>>,
    interaction_query: Query<(), With<Interaction>>,
) {
    let Some(interaction_assets) = interaction_assets else {
        return;
    };

    if interaction_query.contains(trigger.entity) {
        commands.spawn(sound_effect(interaction_assets.hover.clone()));
    }
}

fn play_on_click_sound_effect(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    interaction_assets: Option<Res<InteractionAssets>>,
    interaction_query: Query<(), With<Interaction>>,
) {
    let Some(interaction_assets) = interaction_assets else {
        return;
    };

    if interaction_query.contains(trigger.entity) {
        commands.spawn(sound_effect(interaction_assets.click.clone()));
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
