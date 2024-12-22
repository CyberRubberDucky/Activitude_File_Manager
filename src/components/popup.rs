
use bevy::{prelude::*, ui::FocusPolicy};
use crate::theme::color::Display;
use crate::interface::header::{header, Header};
use crate::interface::button::{CustomButton, ButtonWidth, ButtonComponent, ButtonSize, InteractiveState, ButtonStyle, primary_default};
use crate::FontResources;
use crate::components::text_editor::text_editor;
use crate::components::context::ContextButton;
use crate::NavigateTo;
use crate::theme::icons::Icon;

#[derive(Component)]
pub struct Popup;
#[derive(Component)]
pub struct SaveButton;
#[derive(Component)]
pub struct CancelButton;

pub fn popup(
    mut commands: &mut Commands,
    fonts: &Res<FontResources>,
    asset_server: &Res<AssetServer>,
    name: &str,
    content: &str,
) {
    let colors = Display::new();
    let save = context_button("Save", InteractiveState::Default, Icon::Save);
    let cancel = context_button("Cancel", InteractiveState::Default, Icon::Exit);

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
            header(parent, &fonts, &asset_server, Header::File, name);
            text_editor(parent, &fonts, content);

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
                parent.spawn((
                    Node::default(),
                    CancelButton,
                )).with_children(|child| {
                    ButtonComponent::spawn_button(child, &asset_server, &fonts, cancel);
                });

                parent.spawn((
                    Node::default(),
                    SaveButton,
                )).with_children(|child| {
                    ButtonComponent::spawn_button(child, &asset_server, &fonts, save);
                });
            });
        });
    });
}

pub fn menu_handler(
    mut commands: Commands,
    fonts: Res<FontResources>,
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<(&Interaction, &Parent), (Changed<Interaction>, With<Button>)>,
    query: Query<&ContextButton>,
) {
    for (interaction, parent) in &mut interaction_query {
        if let Interaction::Pressed = *interaction {
            if query.get(parent.get()).is_ok() {
                popup(&mut commands, &fonts, &asset_server, "", "");
            }
        }
    }
}

pub fn save_button(
    mut commands: Commands,
    fonts: Res<FontResources>,
    asset_server: Res<AssetServer>,
    mut popup_query: Query<(Entity, &Node, &Children), With<Popup>>,
    mut interaction_query: Query<(&Interaction, &Parent), (Changed<Interaction>, With<Button>)>,
    s_query: Query<&SaveButton>,
    c_query: Query<&CancelButton>,
) {
    for (interaction, parent) in &mut interaction_query {
        if let Interaction::Pressed = *interaction {
            if s_query.get(parent.get()).is_ok() || c_query.get(parent.get()).is_ok() {
                println!("{}", if s_query.get(parent.get()).is_ok() { "Saving" } else { "Cancel" });
                for (entity, _, children) in popup_query.iter_mut() {
                    for child in children.iter() {
                        commands.entity(*child).despawn_recursive();
                    }
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}

pub fn spacer(parent: &mut ChildBuilder) {
    parent.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    });
}

fn context_button(label: &str, status: InteractiveState, icon: Icon) -> CustomButton {
    CustomButton::new(
        label,
        Some(icon),
        None,
        ButtonStyle::Secondary,
        ButtonWidth::Hug,
        ButtonSize::Medium,
        status,
        NavigateTo::None,
        JustifyContent::Center,
        true,
        false,
    )
}