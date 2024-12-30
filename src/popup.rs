use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use crate::Theme;
use crate::ramp_ds::components::button::default_button;

use bevy_simple_text_input::{
    TextInput,
    TextInputInactive,
    TextInputPlaceholder,
    TextInputTextColor,
    TextInputTextFont,
    TextInputValue,
};

#[derive(Component)]
pub struct Popup;
#[derive(Component)]
pub struct SaveButton;
#[derive(Component)]
pub struct CancelButton;
#[derive(Component)]
pub struct DeleteButton;
#[derive(Component)]
pub struct TextEditor;

pub fn text_editor(
    parent: &mut ChildBuilder,
    theme: &Res<Theme>,
    content: &str,
) {
    let font = theme.fonts.style.text.clone();
    let font_size = theme.fonts.size.md;

    parent.spawn((
        Node {
            border: UiRect::all(Val::Px(1.0)),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            max_height: Val::Px(1000.0),
            align_items: AlignItems::Start, 
            justify_content: JustifyContent::Start,
            padding: UiRect::all(Val::Px(16.0)),
            ..default()
        },
        TextEditor,
        BorderColor(theme.colors.outline.secondary),
        BackgroundColor(theme.colors.background.primary),
        BorderRadius::all(Val::Px(4.0)),
        FocusPolicy::Block,
        TextInput,
        TextInputTextFont(TextFont {
            font,
            font_size,
            ..default()
        }),
        TextInputTextColor(TextColor(theme.colors.text.primary)),
        TextInputInactive(true),
        TextInputValue(content.to_string()),
        TextInputPlaceholder {
            value: "Write to file...".to_string(),
            ..default()
        },
    ));
}

pub fn popup(
    commands: &mut Commands,
    theme: &Res<Theme>,
    name: &str,
    content: &str,
) {


    // ==== Screen Container ==== //

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            position_type: PositionType::Absolute,
            ..default()
        },
        Popup,
    )).with_children(|parent| {

        // ==== Popup ==== //

        parent.spawn((
            Node {
                width: Val::Px(800.0),
                height: Val::Px(550.0),
                row_gap: Val::Px(16.0),
                border: UiRect::all(Val::Px(1.0)),
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                padding: UiRect {
                    left: Val::Px(48.0),
                    right: Val::Px(48.0),
                    top: Val::Px(16.0),
                    bottom: Val::Px(16.0),
                },
                ..default()
            },
            BorderColor(theme.colors.outline.secondary),
            BackgroundColor(theme.colors.background.primary),
            BorderRadius::all(Val::Px(8.0)),
        )).with_children(|parent| {

            // ==== Header ==== //

            parent.spawn(Node {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(12.0)),
                ..default()
            }).with_children(|parent| {
                parent.spawn((
                    Text::new(name),
                    TextFont {
                        font: theme.fonts.style.heading.clone(),
                        font_size: theme.fonts.size.h4,
                        ..default()
                    },
                    TextColor(theme.colors.text.heading),
                ));
            });

            // ==== Text Input ==== //

            text_editor(parent, theme, content);

            // ==== Buttons ==== //

            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::End,
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    ..default()
                },
            )).with_children(|parent| {

                // ==== Delete Button ==== //
                default_button("Delete", theme.icons.get("delete").unwrap()).spawn_under(parent, DeleteButton, theme);

                // ==== Spacer ==== //
                parent.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        ..default()
                    },
                ));

                // ==== Cancel Button ==== //
                default_button("Cancel", theme.icons.get("exit").unwrap()).spawn_under(parent, CancelButton, theme);

                // ==== Save Button ==== //
                default_button("Save", theme.icons.get("save").unwrap()).spawn_under(parent, SaveButton, theme);

            });
        });
    });
}
