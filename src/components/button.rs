use bevy::prelude::*;

use crate::Theme;
use crate::theme::color::ButtonColor;

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

pub struct CustomButton{
    label: String,
    icon: ImageNode,
    style: ButtonStyle,
    width_style: ButtonWidth,
    alignment: JustifyContent,
}

impl CustomButton {
    pub fn create_on<T: Bundle>(
        self,
        parent: &mut ChildBuilder,
        tag: T,
        theme: &Res<Theme>
    ) {
        let status = InteractiveState::Default;

        let colors: ButtonColor = ButtonColor::new(self.style, status);
        let font = theme.fonts.style.label.clone();

        let (button_width, flex_grow) = match self.width_style {
            ButtonWidth::Expand => (Val::Percent(100.0), 1.0),
            ButtonWidth::Hug => (Val::Auto, 0.0),
        };

        let (height, padding, icon_size, icon_pad, font_size) = (32.0, 12.0, 20.0, 4.0, theme.fonts.size.md);

        parent.spawn((
            Button,
            Node {
                flex_grow,
                height: Val::Px(height),
                flex_basis: button_width,
                width: button_width,
                border: UiRect::all(Val::Px(1.0)),
                justify_content: self.alignment,
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
            self.style,
            status,
            tag,
        )).with_children(|button| {

            // === Spawn Icon === //

            button.spawn((
                self.icon,
                Node {
                    height: Val::Px(icon_size),
                    width: Val::Px(icon_size),
                    margin: UiRect::right(Val::Px(icon_pad)), 
                    ..default()
                },
            ));

            // === Spawn Label === //

            button.spawn((
                Text::new(self.label),
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

pub fn default_button(label: &str, icon: ImageNode) -> CustomButton {
    CustomButton {
        label: label.to_string(),
        icon,
        style: ButtonStyle::Secondary,
        width_style: ButtonWidth::Hug,
        alignment: JustifyContent::Center,
    }
}

pub fn context_button(label: &str, icon: ImageNode) -> CustomButton {
    CustomButton {
        label: label.to_string(),
        icon,
        style: ButtonStyle::Ghost,
        width_style: ButtonWidth::Expand,
        alignment: JustifyContent::Start,
    }
}

// ==== Handle Button Interactions ==== //

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
                        border_color.0 = colors.outline;
                    }
                    Interaction::None => {
                        let colors: ButtonColor = ButtonColor::new(*button_style, InteractiveState::Default);
                        *color = colors.background.into();
                        border_color.0 = colors.outline;
                    }
                    Interaction::Pressed => {
                        let colors: ButtonColor = ButtonColor::new(*button_style, InteractiveState::Selected);
                        *color = colors.background.into();
                        border_color.0 = colors.outline;
                    }
                }
            }
        }
    }
}