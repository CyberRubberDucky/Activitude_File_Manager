#![allow(unused)]

use bevy::prelude::*;

use crate::theme::{
    color::ButtonColor,
    fonts::FontResources,
    icons::Icon,
};


use crate::NavigateTo;

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

pub enum ButtonSize {
    Medium,
    Large,
}

#[derive(PartialEq)]
pub enum ButtonWidth {
    Expand,
    Hug,
}

pub struct CustomButton {
    label: String,
    icon: Option<Icon>,
    photo: Option<String>,
    style: ButtonStyle,
    width_style: ButtonWidth,
    size: ButtonSize,
    state: InteractiveState,
    action: NavigateTo,
    alignment: JustifyContent,
    enabled: bool,
    selected: bool,
}

impl CustomButton {
    pub fn new(
        label: &str,
        icon: Option<Icon>,
        photo: Option<String>,
        style: ButtonStyle,
        width_style: ButtonWidth,
        size: ButtonSize,
        state: InteractiveState,
        action: NavigateTo,
        alignment: JustifyContent,
        enabled: bool,
        selected: bool,
    ) -> Self {
        Self {
            label: label.to_string(),
            icon,
            photo,
            style,
            width_style,
            size,
            state,
            action,
            alignment,
            enabled,
            selected,
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
        let mut status = InteractiveState::Default;
        
        if data.enabled {
            if data.state == InteractiveState::Selected {
                status = InteractiveState::Selected;
            } else {
                status = InteractiveState::Default;
            }
        } else {
            status = InteractiveState::Disabled;
        }

        let colors: ButtonColor = ButtonColor::new(data.style, status);
        let font = fonts.style.label.clone();

        let (button_width, flex_grow) = match data.width_style {
            ButtonWidth::Expand => (Val::Percent(100.0), 1.0),
            ButtonWidth::Hug => (Val::Auto, 0.0),
        };

        let (height, padding, icon_size, icon_pad, font_size) = match data.size {
            ButtonSize::Large => (48.0, 24.0, 24.0, 12.0, fonts.size.lg),
            ButtonSize::Medium => (32.0, 12.0, 20.0, 4.0, fonts.size.md)
        };

        let mut button = parent.spawn((
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
            data.action,
        ));
        
        button.with_children(|button| {
            if let Some(icon) = data.icon {
                button.spawn((
                    Icon::new(data.icon.unwrap(), asset_server),
                    Node {
                        height: Val::Px(icon_size),
                        width: Val::Px(icon_size),
                        margin: UiRect::right(Val::Px(icon_pad)), 
                        ..default()
                    },
                ));
            }

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

        button.insert(data.style);
        button.insert(status);
        if data.selected { button.insert(Selectable); }
    }
}


#[derive(Component)]
pub struct ButtonInteraction {
    pub state: InteractiveState,
    pub is_selected: bool,
}

#[derive(Component)]
pub struct Selectable;

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            Option<&ButtonStyle>,
            &InteractiveState,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color, button_style, state) in &mut interaction_query {
        if *state != InteractiveState::Disabled && *state != InteractiveState::Selected {
            if let Some(button_style) = button_style {
                match *interaction {
                    Interaction::Hovered => {
                        let colors: ButtonColor = ButtonColor::new(*button_style, InteractiveState::Hover);
                        *color = colors.background.into();
                        border_color.0 = colors.outline.into();
                    }
                    Interaction::None => {
                        let colors: ButtonColor = ButtonColor::new(*button_style, InteractiveState::Default);
                        *color = colors.background.into();
                        border_color.0 = colors.outline.into();
                    }
                    Interaction::Pressed => {
                        let colors: ButtonColor = ButtonColor::new(*button_style, InteractiveState::Selected);
                        *color = colors.background.into();
                        border_color.0 = colors.outline.into();
                    }
                }
            }
        }
    }
}

pub fn primary_default(label: &str, enabled: bool, navigate_to: NavigateTo) -> CustomButton {
    return CustomButton::new(
        label,
        None,
        None,
        ButtonStyle::Primary,
        ButtonWidth::Expand,
        ButtonSize::Large,
        InteractiveState::Default,
        navigate_to,
        JustifyContent::Center,
        enabled,
        false,
    );
}

pub fn secondary_default(label: &str, icon: Icon, navigate_to: NavigateTo) -> CustomButton {
    return CustomButton::new(
        label,
        Some(icon),
        None,
        ButtonStyle::Secondary,
        ButtonWidth::Hug,
        ButtonSize::Medium,
        InteractiveState::Default,
        navigate_to,
        JustifyContent::Center,
        true,
        false,
    );
}