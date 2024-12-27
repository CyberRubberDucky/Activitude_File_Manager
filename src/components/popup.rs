use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use crate::FontResources;

use crate::theme::icons::Icon;
use crate::theme::color::Display;

use crate::components::button::ButtonComponent;
use crate::components::button::default_button;

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
    fonts: &Res<FontResources>,
    content: &str,
) {
    let font = fonts.style.text.clone();
    let font_size = fonts.size.md;

    let colors = Display::new();

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
        BorderColor(colors.outline_secondary),
        BackgroundColor(colors.bg_primary),
        BorderRadius::all(Val::Px(4.0)),
        FocusPolicy::Block,
        TextInput,
        TextInputTextFont(TextFont {
            font,
            font_size,
            ..default()
        }),
        TextInputTextColor(TextColor(colors.text_primary)),
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
    fonts: &Res<FontResources>,
    asset_server: &Res<AssetServer>,
    name: &str,
    content: &str,
) {
    let colors = Display::new();

    // ==== Define Buttons ==== //

    let save = default_button("Save", Icon::Save);
    let cancel = default_button("Cancel", Icon::Exit);
    let delete = default_button("Delete", Icon::Delete);

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
            BorderColor(colors.outline_secondary),
            BackgroundColor(colors.bg_primary),
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
                        font: fonts.style.heading.clone(),
                        font_size: fonts.size.h4,
                        ..default()
                    },
                    TextColor(colors.text_heading),
                ));
            });

            // ==== Text Input ==== //

            text_editor(parent, fonts, content);

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

                parent.spawn((
                    Node::default(),
                    DeleteButton,
                )).with_children(|child| {
                    ButtonComponent::spawn_button(child, asset_server, fonts, delete);
                });

                // ==== Spacer ==== //

                parent.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        ..default()
                    },
                ));

                // ==== Cancel Button ==== //

                parent.spawn((
                    Node::default(),
                    CancelButton,
                )).with_children(|child| {
                    ButtonComponent::spawn_button(child, asset_server, fonts, cancel);
                });

                // ==== Save Button ==== //

                parent.spawn((
                    Node::default(),
                    SaveButton,
                )).with_children(|child| {
                    ButtonComponent::spawn_button(child, asset_server, fonts, save);
                });

            });
        });
    });
}
