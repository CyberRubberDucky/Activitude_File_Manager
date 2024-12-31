use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::input::mouse::MouseButton;

use crate::EXPAND;
use crate::Theme;

use crate::ramp_ds::components::button::context_button;

#[derive(Component)]
pub struct ContextMenu;
#[derive(Component)]
pub struct NewFileButton;
#[derive(Component)]
pub struct NewFolderButton;

pub fn context_menu(
    mut commands: Commands,
    query_window: Query<&Window, With<PrimaryWindow>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut context_menu_query: Query<(Entity, &Children), With<ContextMenu>>,
    theme: Res<Theme>,
) {
    let window = query_window.single();

    if let Some(cursor_position) = window.cursor_position() {
        if mouse_button.just_pressed(MouseButton::Right) {

            // === Spawn context menu at mouse local === //
            if context_menu_query.is_empty() {
                let (height, width) = (
                    cursor_position.y / window.height(),
                    cursor_position.x / window.width(),
                );

                // === Define menu buttons === //

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
                    BorderColor(theme.colors.outline.secondary),
                    BackgroundColor(theme.colors.background.primary),
                    BorderRadius::all(Val::Px(8.0)),
                    ContextMenu,
                )).with_children(|child| {

                    // ==== Create Folder Button ===== //

                    child.spawn((
                        Node {
                            width: EXPAND,
                            padding: UiRect::all(Val::Px(4.0)),
                            ..default()
                        },
                    )).with_children(|parent| {
                        context_button("Create Folder", theme.icons.get("folder")).spawn_under(parent, NewFolderButton, &theme);
                    });

                    // ==== Separator ===== //

                    child.spawn((
                        Node {
                            width: EXPAND,
                            height: Val::Px(1.0),
                            ..default()
                        },
                        BackgroundColor(theme.colors.outline.secondary),
                    ));

                    // ==== Create File Button ===== //

                    child.spawn((
                        Node {
                            width: EXPAND,
                            padding: UiRect::all(Val::Px(4.0)),
                            ..default()
                        },
                    )).with_children(|parent| {
                        context_button("Create File", theme.icons.get("file")).spawn_under(parent, NewFileButton, &theme);
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
