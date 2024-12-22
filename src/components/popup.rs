
use bevy::prelude::*;
use crate::theme::color::Display;
use crate::components::button::{CustomButton, ButtonWidth, ButtonComponent, ButtonSize, InteractiveState, ButtonStyle};
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
    commands: &mut Commands,
    fonts: &Res<FontResources>,
    asset_server: &Res<AssetServer>,
    name: &str,
    content: &str,
) {
    let colors = Display::new();

    // ==== Define Buttons ==== //

    let save = context_button("Save", InteractiveState::Default, Icon::Save);
    let cancel = context_button("Cancel", InteractiveState::Default, Icon::Exit);

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
            small_header(parent, fonts, name);
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


pub fn small_header (
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
    title: &str, 
) {
    let colors = Display::new();

    let node = Node {
        width: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Row,
        padding: UiRect::all(Val::Px(12.0)),
        ..default()
    };

    parent.spawn(node).with_children(|parent| {
        parent.spawn((
            Text::new(title),
            TextFont {
                font: fonts.style.heading.clone(),
                font_size: fonts.size.h4,
                ..default()
            },
            TextColor(colors.text_heading),
        ));
    });
}