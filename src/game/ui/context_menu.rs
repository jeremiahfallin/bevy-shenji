use crate::game::character::CharacterInfo;
use crate::theme::prelude::*;
use bevy::prelude::*;
use bevy_immediate::{
    CapSet, Imm, ImmEntity, ImplCap,
    attach::{BevyImmediateAttachPlugin, ImmediateAttach},
    ui::CapsUi,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevyImmediateAttachPlugin::<CapsUi, ContextMenuOverlay>::new());
    app.add_systems(Update, close_context_menu_on_click);
}

#[derive(Resource, Default)]
pub struct ContextMenuState {
    pub is_open: bool,
    pub position: Vec2,
    pub target: Option<Entity>,
    pub context_type: ContextMenuType,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum ContextMenuType {
    #[default]
    None,
    InventoryItem,
    Unit,
    Character,
}

#[derive(Component)]
pub struct WithContextMenu;

pub trait ImmUiContextMenuExt {
    fn context_menu(self, context_type: ContextMenuType, target: Entity) -> Self;
}

impl<Cap> ImmUiContextMenuExt for ImmEntity<'_, '_, '_, Cap>
where
    Cap: CapSet + ImplCap<CapabilityObserver>,
{
    fn context_menu(mut self, context_type: ContextMenuType, target: Entity) -> Self {
        let has_menu = self
            .cap_get_component::<WithContextMenu>()
            .ok()
            .flatten()
            .is_some();

        if !has_menu {
            self.entity_commands().insert(WithContextMenu).observe(
                move |trigger: On<Pointer<Click>>, mut state: ResMut<ContextMenuState>| {
                    if trigger.event().button == PointerButton::Secondary {
                        state.is_open = true;
                        state.position = trigger.event().pointer_location.position;
                        state.context_type = context_type;
                        state.target = Some(target);
                    }
                },
            );
        }
        self
    }
}

// --- Context Menu Overlay Rendering ---

#[derive(Component)]
pub struct ContextMenuOverlay;

impl ImmediateAttach<CapsUi> for ContextMenuOverlay {
    type Params = (
        ResMut<'static, ContextMenuState>,
        Query<'static, 'static, &'static CharacterInfo>,
    );

    fn construct(
        ui: &mut Imm<CapsUi>,
        (menu_state, characters): &mut (ResMut<ContextMenuState>, Query<&CharacterInfo>),
    ) {
        if !menu_state.is_open {
            return;
        }

        let pos = menu_state.position;
        let context_type = menu_state.context_type;
        let target = menu_state.target;

        // Get target name for header
        let header_text = match (context_type, target) {
            (ContextMenuType::Character, Some(entity)) => {
                if let Ok(info) = characters.get(entity) {
                    info.name.clone()
                } else {
                    "Character".to_string()
                }
            }
            (ContextMenuType::Unit, _) => "Unit".to_string(),
            (ContextMenuType::InventoryItem, _) => "Item".to_string(),
            _ => "Menu".to_string(),
        };

        // Render the menu panel at the cursor position
        ui.ch_id("context_menu")
            .style(move |n: &mut Node| {
                n.position_type = PositionType::Absolute;
                n.left = Val::Px(pos.x);
                n.top = Val::Px(pos.y);
                n.min_width = Val::Px(150.0);
                n.flex_direction = FlexDirection::Column;
                n.padding = UiRect::all(Val::Px(4.0));
            })
            .bg(GRAY_800)
            .border(1.0)
            .border_color(GRAY_700)
            .rounded(4.0)
            .add(|ui| {
                // Header
                ui.ch()
                    .label(&header_text)
                    .text_size(13.0)
                    .text_color(GRAY_100);

                // Divider
                ui.ch().on_spawn_insert(|| {
                    (
                        Node {
                            height: Val::Px(1.0),
                            width: Val::Percent(100.0),
                            margin: UiRect::axes(Val::Px(0.0), Val::Px(4.0)),
                            ..default()
                        },
                        BackgroundColor(GRAY_700),
                    )
                });

                // Menu items based on context type
                match context_type {
                    ContextMenuType::Character => {
                        menu_item(ui, "Inspect");
                        menu_item(ui, "Assign to Squad");
                        menu_item(ui, "Dismiss");
                    }
                    ContextMenuType::Unit => {
                        menu_item(ui, "Move");
                        menu_item(ui, "Attack");
                    }
                    ContextMenuType::InventoryItem => {
                        menu_item(ui, "Use");
                        menu_item(ui, "Drop");
                    }
                    ContextMenuType::None => {}
                }
            });
    }
}

fn menu_item(ui: &mut Imm<CapsUi>, label: &str) {
    ui.ch()
        .button()
        .w_full()
        .style(|n: &mut Node| {
            n.padding = UiRect::axes(Val::Px(8.0), Val::Px(4.0));
        })
        .on_click_once(|_: On<Pointer<Click>>, mut state: ResMut<ContextMenuState>| {
            // Close the menu on any item click for now
            state.is_open = false;
        })
        .add(|ui| {
            ui.ch()
                .label(label)
                .text_size(13.0)
                .text_color(GRAY_100);
        });
}

/// Close the context menu when the user left-clicks anywhere else.
fn close_context_menu_on_click(
    mouse: Res<ButtonInput<MouseButton>>,
    mut state: ResMut<ContextMenuState>,
) {
    if state.is_open && mouse.just_pressed(MouseButton::Left) {
        state.is_open = false;
    }
}
