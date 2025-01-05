use bevy::prelude::*;
use ramp_ds::components::Button;
use ramp_ds::traits::Component;
use ramp_ds::prelude::*;
use crate::Theme;

#[derive(Component)]
pub struct Popup;

pub fn popup(
    commands: &mut Commands,
    theme: &Res<Theme>,
    name: &str,
    content: &str,
) {
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
        parent.spawn((
            Node {
                width: Val::Px(800.0),
                height: Val::Px(550.0),
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
            Header::new(name, Size::Medium, None, None, false).box_spawn(parent, theme);
            TextEditor::new(content).box_spawn(parent, theme);
            parent.spawn((
                Node {
                    width: EXPAND,
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    padding: UiRect {
                        top: Val::Px(16.0),
                        bottom: Val::Px(16.0),
                        ..default()
                    },
                    ..default()
                },
            )).with_children(|parent| {
                Button::secondary("Delete", "delete", || "Delete".to_string()).box_spawn(parent, theme);
                parent.spawn(Node{width: EXPAND, ..default()});
                Button::secondary("Cancel", "exit", || "Cancel".to_string()).box_spawn(parent, theme);
                Button::secondary("Save", "save", || "Save".to_string()).box_spawn(parent, theme);
            });
        });
    });
}
