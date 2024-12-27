use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use crate::theme::color::Display;

use crate::EXPAND;
use crate::components::button::{
    ButtonComponent, 
    context_button
};

use crate::FontResources;
use crate::theme::icons::Icon;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct ContextMenu;
#[derive(Component)]
pub struct NewFileButton;
#[derive(Component)]
pub struct NewFolderButton;

pub fn context_menu(
    mut commands: Commands,
    fonts: Res<FontResources>,
    asset_server: Res<AssetServer>,
    query_window: Query<&Window, With<PrimaryWindow>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut context_menu_query: Query<(Entity, &Children), With<ContextMenu>>,
) {
    let window = query_window.single();
    let colors = Display::new();

    if let Some(cursor_position) = window.cursor_position() {
        if mouse_button.just_pressed(MouseButton::Right) {

            // === Spawn context menu at mouse local === //
            if context_menu_query.is_empty() {
                let (height, width) = (
                    cursor_position.y / window.height(),
                    cursor_position.x / window.width(),
                );

                // === Define menu buttons === //
                let folder = context_button("Create Folder", Icon::Folder);
                let file = context_button("Create File", Icon::File);

                commands.spawn((
                    Node {
                        left: Val::Percent(width * 100.0),
                        top: Val::Percent(height * 100.0),
                        width: Val::Px(300.0),
                        border: UiRect::all(Val::Px(1.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BorderColor(colors.outline_secondary),
                    BackgroundColor(colors.bg_primary),
                    BorderRadius::all(Val::Px(8.0)),
                    ContextMenu,
                )).with_children(|child| {

                    // ==== Create Folder Button ===== //

                    child.spawn((
                        Node {
                            width: EXPAND,
                            ..default()
                        },
                        NewFolderButton,
                    )).with_children(|parent| {
                        ButtonComponent::spawn_button(parent, &asset_server, &fonts, folder);
                    });

                    // ==== Separator ===== //

                    child.spawn((
                        Node {
                            width: EXPAND,
                            height: Val::Px(1.0),
                            ..default()
                        },
                        BackgroundColor(colors.outline_secondary),
                    ));

                    // ==== Create File Button ===== //

                    child.spawn((
                        Node {
                            width: EXPAND,
                            ..default()
                        },
                        NewFileButton,
                    )).with_children(|parent| {
                        ButtonComponent::spawn_button(parent, &asset_server, &fonts, file);
                    });
                });
            }
        }

        // ==== Handle Left Click to Select Delete Option ===== //
        
        if mouse_button.just_pressed(MouseButton::Left) {
            for (entity, children) in context_menu_query.iter_mut() {
                for child in children.iter() {
                    commands.entity(*child).despawn_recursive();
                }
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
