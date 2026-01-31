use bevy::prelude::*;
use bevy_immediate::ui::CapsUi;
use bevy_immediate::ui::base::CapabilityUiBase as ForeignCapabilityUiBase;
use bevy_immediate::{CapSet, ImmCapAccessRequests, ImmCapability, ImmEntity, ImplCap};

pub struct CapabilityUiLayout;

impl ImmCapability for CapabilityUiLayout {
    fn build<Cap: CapSet>(app: &mut App, cap_req: &mut ImmCapAccessRequests<Cap>) {
        ForeignCapabilityUiBase::build(app, cap_req);
    }
}

impl ImplCap<CapabilityUiLayout> for CapsUi {}

pub trait ImmUiStyleExt {
    fn style(self, func: impl FnOnce(&mut Node)) -> Self;
}

impl<Cap> ImmUiStyleExt for ImmEntity<'_, '_, '_, Cap>
where
    Cap: ImplCap<CapabilityUiLayout>,
{
    fn style(mut self, func: impl FnOnce(&mut Node)) -> Self {
        // Attempt to get mutable access to the Style component.
        if let Ok(Some(mut style)) = self.cap_get_component_mut::<Node>() {
            func(&mut style);
        } else {
            let mut node = Node::default();
            func(&mut node);

            // Magic Fix: Ensure every styled node is visible
            self.entity_commands().insert((
                node,
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
            ));
        }
        self
    }
}
