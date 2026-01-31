use bevy::picking::events::{Drag, DragStart, Pointer, Scroll};
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui_widgets::{
    ControlOrientation, CoreScrollbarDragState, CoreScrollbarThumb, Scrollbar, ScrollbarPlugin,
};
use bevy_immediate::{CapSet, Imm, ImmEntity};

// 1. THE PLUGIN
// Add this to your main app to enable scrolling logic
pub struct ScrollWidgetPlugin;

impl Plugin for ScrollWidgetPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<ScrollbarPlugin>() {
            app.add_plugins(ScrollbarPlugin);
        }

        // This system handles the visual color change of the scrollbar
        app.add_systems(Update, update_scrollbar_style_on_drag);

        // This observer attaches input logic whenever you create a scroll area
        app.add_observer(
            |event: On<bevy::ecs::lifecycle::Add, ScrollableContent>, mut commands: Commands| {
                commands
                    .entity(event.event().entity)
                    .insert(ScrollState::default())
                    .observe(scroll_on_mouse)
                    .observe(scroll_on_drag_start)
                    .observe(scroll_on_drag);
            },
        );
    }
}

// 2. COMPONENTS
#[derive(Component)]
pub struct ScrollableContent; // Marker for the content area

#[derive(Component, Default)]
struct ScrollState {
    initial_pos: Vec2,
}

// 3. THE TRAIT (The "API")
// This allows you to call .scrollarea() on your UI builder
pub trait ScrollBarWidget<Caps: CapSet> {
    fn scrollarea(
        self,
        outer_node_style: Node,
        content_node_style: Node,
        content_bundle: impl Bundle,
        content: impl FnOnce(&mut Imm<'_, '_, Caps>),
    ) -> Self;
}

impl<Caps> ScrollBarWidget<Caps> for ImmEntity<'_, '_, '_, Caps>
where
    Caps: CapSet,
{
    fn scrollarea(
        self,
        outer_node_style: Node,
        content_node_style: Node,
        content_bundle: impl Bundle,
        content: impl FnOnce(&mut Imm<'_, '_, Caps>),
    ) -> Self {
        let grid_template = |scrollbar: bool| {
            if scrollbar {
                vec![RepeatedGridTrack::flex(1, 1.), RepeatedGridTrack::auto(1)]
            } else {
                vec![RepeatedGridTrack::flex(1, 1.)]
            }
        };

        let horizontal = content_node_style.overflow.x == OverflowAxis::Scroll;
        let vertical = content_node_style.overflow.y == OverflowAxis::Scroll;

        self.on_spawn_insert(|| {
            (
                Node {
                    display: Display::Grid,
                    grid_template_columns: grid_template(horizontal),
                    grid_template_rows: grid_template(vertical),

                    // Use all remaining values from user provided style
                    ..outer_node_style
                },
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
            )
        })
        .add(|ui| {
            let mut scrollarea_content = ui.ch().on_spawn_insert(|| {
                (
                    content_node_style,
                    ScrollPosition(Vec2::ZERO),
                    // FIX: Use the public component that the Plugin is listening for
                    ScrollableContent,
                    content_bundle,
                    Visibility::default(),
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
                )
            });

            // Store entity for scrollable content area
            let scrollbar_target = scrollarea_content.entity();

            // Finalize construction of scrollarea entity
            scrollarea_content.add(content);

            if vertical {
                // Vertical scrollbar
                ui.ch().on_spawn_insert(|| {
                    (
                        Node {
                            width: px(8),
                            grid_row: GridPlacement::start(1),
                            grid_column: GridPlacement::start(2),
                            ..default()
                        },
                        Scrollbar {
                            orientation: ControlOrientation::Vertical,
                            target: scrollbar_target,
                            min_thumb_length: 8.0,
                        },
                        Children::spawn(Spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                ..default()
                            },
                            Hovered::default(),
                            BackgroundColor(colors::GRAY2.into()),
                            BorderRadius::all(px(4)),
                            CoreScrollbarThumb,
                        ))),
                        Visibility::default(),
                        InheritedVisibility::default(),
                        ViewVisibility::default(),
                    )
                });
            }

            if horizontal {
                // Horizontal scrollbar
                ui.ch().on_spawn_insert(|| {
                    (
                        Node {
                            min_height: px(8),
                            grid_row: GridPlacement::start(2),
                            grid_column: GridPlacement::start(1),
                            ..default()
                        },
                        Scrollbar {
                            orientation: ControlOrientation::Horizontal,
                            target: scrollbar_target,
                            min_thumb_length: 8.0,
                        },
                        Children::spawn(Spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                ..default()
                            },
                            Hovered::default(),
                            BackgroundColor(colors::GRAY2.into()),
                            BorderRadius::all(px(4)),
                            CoreScrollbarThumb,
                        ))),
                        Visibility::default(),
                        InheritedVisibility::default(),
                        ViewVisibility::default(),
                    )
                });
            }
        })
    }
}

// 4. THE SYSTEMS

fn scroll_on_mouse(
    scroll: On<Pointer<Scroll>>,
    // FIX: Query for ScrollableContent instead of MyScrollableNode
    mut scroll_position_query: Query<(&mut ScrollPosition, &ComputedNode), With<ScrollableContent>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if let Ok((mut scroll_position, node)) = scroll_position_query.get_mut(scroll.entity) {
        let visible_size = node.size() * node.inverse_scale_factor;
        let content_size = node.content_size() * node.inverse_scale_factor;
        let max_range = (content_size - visible_size).max(Vec2::ZERO);

        let mut delta = Vec2::new(scroll.x, scroll.y);

        if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
            std::mem::swap(&mut delta.x, &mut delta.y);
        }

        let inverse_scale_factor: f32 = node.inverse_scale_factor;

        match scroll.unit {
            bevy::input::mouse::MouseScrollUnit::Line => {
                delta *= 28.;
            }
            bevy::input::mouse::MouseScrollUnit::Pixel => {
                delta *= 1.0 / inverse_scale_factor.max(0.0001);
            }
        }

        scroll_position.0 = (scroll_position.0 - delta).min(max_range).max(Vec2::ZERO);
    }
}

fn scroll_on_drag_start(
    mut drag_start: On<Pointer<DragStart>>,
    // FIX: Query for ScrollableContent
    mut scroll_position_query: Query<(&ComputedNode, &mut ScrollState), With<ScrollableContent>>,
) {
    if let Ok((computed_node, mut state)) = scroll_position_query.get_mut(drag_start.entity) {
        drag_start.propagate(false);
        state.initial_pos = computed_node.scroll_position;
    }
}

fn scroll_on_drag(
    mut drag: On<Pointer<Drag>>,
    ui_scale: Res<UiScale>,
    // FIX: Query for ScrollableContent
    mut scroll_position_query: Query<
        (&mut ScrollPosition, &ComputedNode, &ScrollState),
        With<ScrollableContent>,
    >,
) {
    if let Ok((mut scroll_position, comp_node, state)) = scroll_position_query.get_mut(drag.entity)
    {
        let visible_size = comp_node.size();
        let content_size = comp_node.content_size();

        drag.propagate(false);

        let max_range = (content_size - visible_size).max(Vec2::ZERO);

        // Convert from screen coordinates to UI coordinates then back to physical coordinates
        let distance = drag.distance / (comp_node.inverse_scale_factor * ui_scale.0);

        scroll_position.0 = ((state.initial_pos - distance)
            .min(max_range)
            .max(Vec2::ZERO))
            * comp_node.inverse_scale_factor;
    }
}

// Update the color of the scrollbar thumb.
#[allow(clippy::type_complexity)]
fn update_scrollbar_style_on_drag(
    mut q_thumb: Query<
        (&mut BackgroundColor, &Hovered, &CoreScrollbarDragState),
        (
            With<CoreScrollbarThumb>,
            Or<(Changed<Hovered>, Changed<CoreScrollbarDragState>)>,
        ),
    >,
) {
    for (mut thumb_bg, Hovered(is_hovering), drag) in q_thumb.iter_mut() {
        let color: Color = if *is_hovering || drag.dragging {
            colors::GRAY3
        } else {
            colors::GRAY2
        }
        .into();

        if thumb_bg.0 != color {
            thumb_bg.0 = color;
        }
    }
}

mod colors {
    use bevy::color::Srgba;

    pub const GRAY1: Srgba = Srgba::new(0.224, 0.224, 0.243, 1.0);
    pub const GRAY2: Srgba = Srgba::new(0.486, 0.486, 0.529, 1.0);
    pub const GRAY3: Srgba = Srgba::new(1.0, 1.0, 1.0, 1.0);
}
