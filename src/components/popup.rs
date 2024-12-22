
use bevy::{prelude::*, ui::FocusPolicy};
use crate::theme::color::Display;


use crate::primitives::button::{
    CustomButton, 
    ButtonWidth, 
    ButtonComponent, 
    ButtonSize, 
    InteractiveState, 
    ButtonStyle, 
    primary_default,
};


use crate::FontResources;
use crate::components::text_editor::text_editor;
use crate::NavigateTo;
use crate::theme::icons::Icon;

use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct Popup;
#[derive(Component)]
pub struct SaveButton;

pub fn popup(
    mut commands: &mut Commands,
    fonts: &Res<FontResources>,
    asset_server: &Res<AssetServer>,
) {
    let colors = Display::new();
    let save = context_button("Save", InteractiveState::Default, Icon::Save);

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
                height: Val::Px(500.0),
                border: UiRect::all(Val::Px(1.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                padding: UiRect {
                    left: Val::Px(16.0),
                    right: Val::Px(16.0),
                    top: Val::Px(16.0),
                    bottom: Val::Px(16.0)
                },
                ..default()
            },
            BorderColor(colors.outline_secondary),
            BackgroundColor(colors.bg_primary),
            BorderRadius::all(Val::Px(8.0)),
        )).with_children(|parent| {
            text_editor(parent, &fonts);
            spacer(parent);
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::End,
                    ..default()
                },
                SaveButton
            )).with_children(|child| {
                ButtonComponent::spawn_button(child, &asset_server, &fonts, save);
            });
        });
    });
}

use crate::components::context::ContextButton;

pub fn menu_handler(
    mut commands: Commands,
    fonts: Res<FontResources>,
    asset_server: Res<AssetServer>,

    mut interaction_query: Query<
        (&Interaction, &Parent),
        (Changed<Interaction>, With<Button>),
    >,
    query: Query<&ContextButton>,
) {
    for (interaction, parent) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if query.get(parent.get()).is_ok() {
                    popup(&mut commands, &fonts, &asset_server);
                }
            }
            _ => {}
        }
    }
}


pub fn save_button(
    mut commands: Commands,
    fonts: Res<FontResources>,
    asset_server: Res<AssetServer>,
    mut popup_query: Query<(Entity, &Node, &Children), With<Popup>>,

    mut interaction_query: Query<
        (&Interaction, &Parent),
        (Changed<Interaction>, With<Button>),
    >,
    query: Query<&SaveButton>,
) {
    for (interaction, parent) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if query.get(parent.get()).is_ok() {
                    for (entity, node, children) in popup_query.iter_mut() {
                        for child in children.iter() {
                            commands.entity(*child).despawn_recursive();
                        }
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn spacer (parent: &mut ChildBuilder) {
    parent.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    });
}

fn context_button (label: &str, status: InteractiveState, icon: Icon) -> CustomButton {
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
