use bevy::prelude::*;

use crate::theme::{
    color::ButtonColor,
    fonts::FontResources,
    icons::Icon,
};

#[derive(Copy, Clone, Component, Debug)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Ghost,
}

#[derive(Copy, Clone, Component, PartialEq)]
pub enum InteractiveState {
    Default,
    Selected,
    Hover,
    Disabled,
}

#[derive(PartialEq)]
pub enum ButtonWidth {
    Expand,
    Hug,
}

pub struct CustomButton {
    label: String,
    icon: Icon,
    style: ButtonStyle,
    width_style: ButtonWidth,
    alignment: JustifyContent,
}

impl CustomButton {
    pub fn new(
        label: &str,
        icon: Icon,
        style: ButtonStyle,
        width_style: ButtonWidth,
        alignment: JustifyContent,
    ) -> Self {
        Self {
            label: label.to_string(),
            icon,
            style,
            width_style,
            alignment,
        }
    }
}

pub struct ButtonComponent;

impl ButtonComponent {
    pub fn spawn_button(
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
        fonts: &Res<FontResources>,
        data: CustomButton,
    ) {
        let status = InteractiveState::Default;

        let colors: ButtonColor = ButtonColor::new(data.style, status);
        let font = fonts.style.label.clone();

        let (button_width, flex_grow) = match data.width_style {
            ButtonWidth::Expand => (Val::Percent(100.0), 1.0),
            ButtonWidth::Hug => (Val::Auto, 0.0),
        };

        let (height, padding, icon_size, icon_pad, font_size) = (32.0, 12.0, 20.0, 4.0, fonts.size.md);

        parent.spawn((
            Button,
            Node {
                flex_grow,
                height: Val::Px(height),
                flex_basis: button_width,
                width: button_width,
                border: UiRect::all(Val::Px(1.0)),
                justify_content: data.alignment,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                padding: UiRect {
                    left: Val::Px(padding),
                    right: Val::Px(padding),
                    ..default()
                },
                ..default()
            },
            BorderColor(colors.outline),
            BorderRadius::MAX,
            BackgroundColor(colors.background),
            data.style,
            status,
        )).with_children(|button| {

            // === Spawn Icon === //

            button.spawn((
                Icon::new(data.icon, asset_server),
                Node {
                    height: Val::Px(icon_size),
                    width: Val::Px(icon_size),
                    margin: UiRect::right(Val::Px(icon_pad)), 
                    ..default()
                },
            ));

            // === Spawn Label === //

            button.spawn((
                Text::new(data.label),
                TextFont {
                    font,
                    font_size,
                    ..default()
                },
                TextColor(colors.label),
            ));     
        });

    }
}

pub fn default_button(label: &str, icon: Icon) -> CustomButton {
    CustomButton::new(
        label,
        icon,
        ButtonStyle::Secondary,
        ButtonWidth::Hug,
        JustifyContent::Center,
    )
}

pub fn context_button(label: &str, icon: Icon) -> CustomButton {
    CustomButton::new(
        label,
        icon,
        ButtonStyle::Ghost,
        ButtonWidth::Expand,
        JustifyContent::Start,
    )
}