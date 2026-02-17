use crate::theme::behaviors::{ImmUiInteractionExt, SecondaryClick};
use crate::theme::prelude::*;
use bevy::prelude::*;
use bevy_immediate::{CapSet, ImmEntity, ImplCap};

#[derive(Resource, Default)]
pub struct ContextMenuState {
    pub is_open: bool,
    pub position: Vec2,
    pub target: Option<Entity>,
    // Optional: generic 'type' if you have different menus (e.g., Inventory vs Unit)
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
pub struct ContextMenuRoot;

#[derive(Component)]
pub struct WithContextMenu;

pub trait ImmUiContextMenuExt {
    fn context_menu(self, context_type: ContextMenuType, target: Entity) -> Self;
}

impl<Cap> ImmUiContextMenuExt for ImmEntity<'_, '_, '_, Cap>
where
    // We need 'CapabilityObserver' to ensure we can attach observers
    Cap: CapSet + ImplCap<CapabilityObserver>,
{
    fn context_menu(mut self, context_type: ContextMenuType, target: Entity) -> Self {
        // A. Check for our SPECIFIC marker
        // This ensures we don't attach multiple context menu listeners to the same entity
        let has_menu = self
            .cap_get_component::<WithContextMenu>()
            .ok()
            .flatten()
            .is_some();

        if !has_menu {
            // B. Attach the Observer
            self.entity_commands().insert(WithContextMenu).observe(
                move |trigger: On<Pointer<Click>>, mut state: ResMut<ContextMenuState>| {
                    // C. Check for Right Click (Secondary)
                    if trigger.event().button == PointerButton::Secondary {
                        state.is_open = true;
                        // .position is the Vec2 screen coordinates
                        state.position = trigger.event().pointer_location.position;
                        state.context_type = context_type;
                        state.target = Some(target);

                        // Optional: Stop the event from bubbling up to parents
                        // trigger.propagate(false);
                    }
                },
            );
        }
        self
    }
}
