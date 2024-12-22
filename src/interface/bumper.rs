use bevy::prelude::*;
use std::sync::Arc;

use crate::theme::{
    color::Display,
    fonts::FontResources,
    icons::Icon,
};

use crate::NavigateTo;
use crate::primitives::button::primary_default;
use crate::primitives::button::ButtonComponent;
use crate::primitives::button::CustomButton;

pub struct Bumper {
    bumper_content_node: Node,
    bumper_node: Node,
}

impl Bumper {
    pub fn new() -> Self {
        Self {
            bumper_content_node: Node {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
                padding: UiRect {
                    top: Val::Px(16.0),
                    bottom: Val::Px(16.0),
                    left: Val::Px(24.0),
                    right: Val::Px(24.0),
                    ..default()
                },
                ..default()
            },
            bumper_node: Node {
                width: Val::Percent(100.0),
                max_width: Val::Px(512.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                ..default()
            },
        }
    }

    pub fn button_bumper(
        self,
        parent: &mut ChildBuilder,
        fonts: &Res<FontResources>,
        asset_server: &Res<AssetServer>,
        buttons: Vec<CustomButton>,
    ) {
        let colors = Display::new();
        parent.spawn((self.bumper_node)).with_children(|parent| {
            parent.spawn(self.bumper_content_node).with_children(|child| {
                for button in buttons {
                    ButtonComponent::spawn_button(child, &asset_server, &fonts, button);
                }
            });
        });
    }

}