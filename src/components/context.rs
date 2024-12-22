use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use crate::theme::color::Display;

use crate::components::button::{
    ButtonComponent, 
    InteractiveState,
    context_button
};

use crate::FontResources;
use crate::theme::icons::Icon;
use bevy::window::PrimaryWindow;
use crate::folder::File;
use crate::FolderState;
use crate::folder::FolderUISection;
use crate::manager_ui::update_folder_ui;
use crate::Folder;

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
    mut context_menu_query: Query<(Entity, &Node, &Children), With<ContextMenu>>,
    mut root: ResMut<Folder>, // Access the root folder to delete items
    mut folder_state: ResMut<FolderState>, // Access the current folder state
) {
    let window = query_window.single();
    let colors = Display::new();

    if let Some(cursor_position) = window.cursor_position() {
        if mouse_button.just_pressed(MouseButton::Right) {
            if context_menu_query.is_empty() {
                let (height, width) = (
                    cursor_position.y / window.height(),
                    cursor_position.x / window.width(),
                );

                // Add buttons for Create Folder, Create File, and Delete
                let folder = context_button("Create Folder", InteractiveState::Default, Icon::Folder);
                let file = context_button("Create File", InteractiveState::Default, Icon::File);
                let delete = context_button("Delete", InteractiveState::Default, Icon::Exit);

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
                            width: Val::Percent(100.0),
                            ..default()
                        },
                        NewFolderButton,
                    )).with_children(|parent| {
                        ButtonComponent::spawn_button(parent, &asset_server, &fonts, folder);
                    });

                    // ==== Separator ===== //
                    child.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(1.0),
                            ..default()
                        },
                        BackgroundColor(colors.outline_secondary),
                    ));

                    // ==== Create File Button ===== //
                    child.spawn((
                        Node {
                            width: Val::Percent(100.0),
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
            for (entity, node, children) in context_menu_query.iter_mut() {
                for child in children.iter() {
                    commands.entity(*child).despawn_recursive();
                }
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

pub fn new_system(
    mut commands: Commands,
    fonts: Res<FontResources>,
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<(&Interaction, &Parent), (Changed<Interaction>, With<Button>)>,
    file_query: Query<&NewFileButton>,
    folder_query: Query<&NewFolderButton>,
    mut root: ResMut<Folder>,
    mut folder_state: ResMut<FolderState>,
    folder_ui_section: Res<FolderUISection>,
) {
    for (interaction, parent) in &mut interaction_query {
        if let Interaction::Pressed = *interaction {

            // ==== On File Creation ===== //

            if file_query.get(parent.get()).is_ok() {
                if let Some(current_folder) = root.find_folder_mut(&folder_state.current_folder) {

                    // ==== Generate Name ===== //

                    let new_file_name = format!("file{}.txt", current_folder.files.len() + 1);

                    // ==== Add file to Current Folder ===== //

                    current_folder.add_file(File {
                        name: new_file_name.clone(),
                        content: String::new(),
                    });

                    // ==== Update UI ===== //

                    update_folder_ui(&mut commands, folder_ui_section.0, current_folder, &fonts, &asset_server);
                }
            }

            // ==== On Folder Creation ===== //

            if folder_query.get(parent.get()).is_ok() {
                if let Some(current_folder) = root.find_folder_mut(&folder_state.current_folder) {
                    let new_folder_name = format!("folder {}", current_folder.subfolders.len() + 1);

                    current_folder.add_subfolder(Folder::new(&new_folder_name, Some(folder_state.current_folder.clone())));

                    update_folder_ui(&mut commands, folder_ui_section.0, current_folder, &fonts, &asset_server);
                }
            }
        }
    }
}