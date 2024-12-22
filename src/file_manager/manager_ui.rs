use bevy::{prelude::*, ui::FocusPolicy};


use crate::theme::{
    color::Display,
    fonts::FontResources,
    icons::Icon,
};

use crate::components::text_input::SearchBar;
use bevy_simple_text_input::TextInputValue;
use bevy_simple_text_input::TextInput;
use crate::file_manager::folder::FolderUISection;
use crate::file_manager::folder::FolderName;
use crate::file_manager::folder::FileName;
use crate::file_manager::object::object;
use crate::Folder;
use crate::RootNode;
use crate::FolderState;

use crate::components::popup::popup;

pub fn display_files_and_folders(
    parent: &mut ChildBuilder,
    folder: &Folder,
    fonts: &Res<FontResources>,
    asset_server: &Res<AssetServer>,
) {
    let colors = Display::new();

    let column_node = Node {
        margin: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(6.0),
        align_items: AlignItems::Center,
        ..default()
    };

    let row_node = Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Start,
        align_items: AlignItems::Center,
        width: Val::Percent(100.0),
        height: Val::Auto,
        margin: UiRect::all(Val::Px(5.0)),
        ..default()
    };

    if let Some(parent_name) = &folder.parent_name {
        parent.spawn(column_node.clone())
        .insert(Button)
        .insert(FolderName(". .".to_string()))
        .with_children(|button| {
            object(button, asset_server, fonts, ". .", "Folder");
        });
    }

    parent.spawn(row_node.clone())
    .with_children(|parent| {
        for (name, file) in &folder.files {
            parent.spawn(column_node.clone())
            .insert(FileName(name.clone()))
            .insert(Button)
            .with_children(|button| {
                object(button, asset_server, fonts, name, "File");
            });
        }

        for (name, subfolder) in &folder.subfolders {
            parent
            .spawn(row_node.clone())
            .insert(Button)
            .insert(FolderName(name.clone()))
            .with_children(|parent| {
                parent.spawn(column_node.clone())
                .with_children(|parent| {
                    object(parent, asset_server, fonts, name, "Folder");
                });
            });
        }
    });
}

pub fn update_folder_ui(
    commands: &mut Commands,
    folder_ui_section: Entity,
    folder: &Folder,
    fonts: &Res<FontResources>,
    asset_server: &Res<AssetServer>,
) {
    commands.entity(folder_ui_section).despawn_descendants();
    commands.entity(folder_ui_section).with_children(|parent| {
        display_files_and_folders(parent, folder, fonts, asset_server);
    });
}

pub fn button_interaction_system(
    mut interaction_query: Query<(&Interaction, &FolderName), (Changed<Interaction>, With<Button>)>,
    mut file_query: Query<(&Interaction, &FileName), (Changed<Interaction>, With<Button>)>,
    mut folder_state: ResMut<FolderState>,
    root_node: Res<RootNode>,
    mut commands: Commands,
    root: Res<Folder>,
    fonts: Res<FontResources>,
    asset_server: Res<AssetServer>,
    folder_ui_section: Res<FolderUISection>, 
    mut query: Query<(&TextInput, &mut TextInputValue), With<SearchBar>>, 
) {
    for (interaction, folder_name) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if folder_name.0 == ". ." {
                if let Some(current_folder) = root.find_folder(&folder_state.current_folder) {
                    if let Some(parent_name) = &current_folder.parent_name {
                        folder_state.current_folder = parent_name.clone();
                        if let Some(parent_folder) = root.find_folder(parent_name) {
                            update_folder_ui(&mut commands, folder_ui_section.0, parent_folder, &fonts, &asset_server);
                            let path = parent_folder.get_path(&root);
                            for (entity, mut text_input) in &mut query {
                                text_input.0 = format!("/{}/", path.clone());
                            }
                        }
                    }
                }
            } else {
                folder_state.current_folder = folder_name.0.clone();
                if let Some(folder) = root.find_folder(&folder_name.0) {
                    update_folder_ui(&mut commands, folder_ui_section.0, folder, &fonts, &asset_server);
                    let path = folder.get_path(&root);
                    for (entity, mut text_input) in &mut query {
                        text_input.0 = format!("/{}/", path.clone());
                    }
                }
            }
        }
    }
    for (interaction, file_name) in file_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if let Some(folder) = root.find_folder(&folder_state.current_folder) {
                if let Some(file) = folder.get_file(&file_name.0) {
                    println!("File: {:?}", file);
                    popup(&mut commands, &fonts, &asset_server, &file.name, &file.content);
                }
            }
        }
    }
}