use bevy::prelude::*;
use std::sync::Arc;

use crate::theme::{
    color::Display,
    fonts::FontResources,
    icons::Icon,
};

use crate::NavigateTo;
use crate::interface::button::primary_default;
use crate::interface::button::ButtonComponent;
use crate::interface::button::CustomButton;

pub struct Interface {
    pub node: Node,
    pub page_node: Node,
    pub content: Node,
    pub content_centered: Node,
}

impl Default for Interface {
    fn default() -> Self {
        Self::new()
    }
}

const EXPAND: Val = Val::Percent(100.0);

impl Interface {
    pub fn new() -> Self {
        Self {
            node: Node {
                width: EXPAND,
                height: EXPAND,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            page_node: Node {
                width: EXPAND,
                height: EXPAND,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            content: Node {
                width: EXPAND,
                height: EXPAND,
                max_width: Val::Px(512.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(24.0),
                ..default()
            },
            content_centered: Node {
                width: EXPAND,
                height: EXPAND,
                max_width: Val::Px(512.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        }
    }
}
