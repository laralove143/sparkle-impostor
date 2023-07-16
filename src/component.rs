//! Handling the message to clone having components

use twilight_model::channel::message::{component::ActionRow, Component};

use crate::{error::Error, MessageSource};

/// Info about the message's components
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Info {
    /// URL components in the message, which can be replicated
    pub url_components: Vec<Component>,
    /// Whether the message has components that can't be replicated
    pub has_invalid_components: bool,
}

impl MessageSource<'_> {
    /// Check that the message has no non-URL components
    ///
    /// URL components can be replicated, so they're allowed
    ///
    /// # Errors
    ///
    /// Returns [`Error::SourceComponent`] if the message has a non-URL
    /// component
    #[allow(clippy::missing_const_for_fn)]
    pub fn check_component(self) -> Result<Self, Error> {
        if self.component_info.has_invalid_components {
            return Err(Error::SourceComponent);
        }

        Ok(self)
    }
}

pub(crate) fn filter_valid(components: &[Component]) -> Vec<Component> {
    components
        .iter()
        .filter_map(|component| {
            if let Component::ActionRow(action_row) = component {
                let components_inner = action_row
                    .components
                    .iter()
                    .filter(|inner_component| is_valid(inner_component))
                    .cloned()
                    .collect::<Vec<_>>();

                if components_inner.is_empty() {
                    None
                } else {
                    Some(Component::ActionRow(ActionRow {
                        components: components_inner,
                    }))
                }
            } else {
                is_valid(component).then(|| component.clone())
            }
        })
        .collect()
}

pub(crate) const fn is_valid(component: &Component) -> bool {
    matches!(component, Component::Button(button) if button.custom_id.is_none())
}