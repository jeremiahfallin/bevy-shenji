use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    input::common_conditions::input_just_pressed,
    prelude::*,
};

use bevy_declarative::prelude::*;

use crate::{AppSystems, UiRoot, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    // Resources & Systems
    app.insert_resource(ClearColor(SPLASH_BACKGROUND_COLOR));
    app.add_systems(OnEnter(Screen::Splash), spawn_splash_screen);

    // Animation & Timers (Unchanged)
    app.add_systems(
        Update,
        (
            tick_fade_in_out.in_set(AppSystems::TickTimers),
            apply_fade_in_out.in_set(AppSystems::Update),
        )
            .run_if(in_state(Screen::Splash)),
    );

    app.add_systems(OnEnter(Screen::Splash), insert_splash_timer);
    app.add_systems(OnExit(Screen::Splash), remove_splash_timer);
    app.add_systems(
        Update,
        (
            tick_splash_timer.in_set(AppSystems::TickTimers),
            check_splash_timer.in_set(AppSystems::Update),
        )
            .run_if(in_state(Screen::Splash)),
    );

    app.add_systems(
        Update,
        enter_title_screen
            .run_if(input_just_pressed(KeyCode::Escape).and(in_state(Screen::Splash))),
    );
}

const SPLASH_BACKGROUND_COLOR: Color = Color::srgb(0.157, 0.157, 0.157);
const SPLASH_DURATION_SECS: f32 = 1.8;
const SPLASH_FADE_DURATION_SECS: f32 = 0.6;

#[derive(Component)]
pub struct SplashScreen;

fn spawn_splash_screen(
    mut commands: Commands,
    ui_root: Res<UiRoot>,
    asset_server: Res<AssetServer>,
) {
    let image_handle = asset_server.load_with_settings(
        "images/splash.png",
        |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::linear();
        },
    );

    let splash = div()
        .w_full()
        .h_full()
        .items_center()
        .justify_center()
        .bg(SPLASH_BACKGROUND_COLOR)
        .insert((
            SplashScreen,
            Name::new("Splash Screen"),
            DespawnOnExit(Screen::Splash),
        ))
        .child(div().w(Val::Percent(70.0)).m(Val::Auto).insert((
            Name::new("Splash image"),
            ImageNode::new(image_handle),
            ImageNodeFadeInOut {
                total_duration: SPLASH_DURATION_SECS,
                fade_duration: SPLASH_FADE_DURATION_SECS,
                t: 0.0,
            },
        )))
        .spawn(&mut commands)
        .id();

    commands.entity(ui_root.0).add_child(splash);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ImageNodeFadeInOut {
    /// Total duration in seconds.
    total_duration: f32,
    /// Fade duration in seconds.
    fade_duration: f32,
    /// Current progress in seconds, between 0 and [`Self::total_duration`].
    t: f32,
}

impl ImageNodeFadeInOut {
    fn alpha(&self) -> f32 {
        // Normalize by duration.
        let t = (self.t / self.total_duration).clamp(0.0, 1.0);
        let fade = self.fade_duration / self.total_duration;

        // Regular trapezoid-shaped graph, flat at the top with alpha = 1.0.
        ((1.0 - (2.0 * t - 1.0).abs()) / fade).min(1.0)
    }
}

fn tick_fade_in_out(time: Res<Time>, mut animation_query: Query<&mut ImageNodeFadeInOut>) {
    for mut anim in &mut animation_query {
        anim.t += time.delta_secs();
    }
}

fn apply_fade_in_out(mut animation_query: Query<(&ImageNodeFadeInOut, &mut ImageNode)>) {
    for (anim, mut image) in &mut animation_query {
        image.color.set_alpha(anim.alpha())
    }
}

#[derive(Resource, Debug, Clone, PartialEq, Reflect)]
#[reflect(Resource)]
struct SplashTimer(Timer);

impl Default for SplashTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(SPLASH_DURATION_SECS, TimerMode::Once))
    }
}

fn insert_splash_timer(mut commands: Commands) {
    commands.init_resource::<SplashTimer>();
}

fn remove_splash_timer(mut commands: Commands) {
    commands.remove_resource::<SplashTimer>();
}

fn tick_splash_timer(time: Res<Time>, mut timer: ResMut<SplashTimer>) {
    timer.0.tick(time.delta());
}

fn check_splash_timer(timer: ResMut<SplashTimer>, mut next_screen: ResMut<NextState<Screen>>) {
    if timer.0.just_finished() {
        next_screen.set(Screen::Title);
    }
}

fn enter_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
