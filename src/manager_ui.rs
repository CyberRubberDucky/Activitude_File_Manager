use bevy::prelude::*;
use bevy_simple_text_input::TextInputValue;
use bevy_simple_text_input::TextInput;
use crate::components::text_input::SearchBar;
use crate::components::popup::popup;
use crate::Folder;
use crate::FolderState;

use crate::folder::{
    FolderUISection,
    FolderName,
    FileName
};

use crate::theme::{
    color::Display,
    fonts::FontResources,
    icons::Icon,
};

pub fn display_files_and_folders(
    parent: &mut ChildBuilder,
    folder: &Folder,
    fonts: &Res<FontResources>,
    asset_server: &Res<AssetServer>,
) {

    // ==== Design Nodes ===== //

    let colors = Display::new();

    let column_node = Node {
        margin: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Start,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(6.0),
        align_items: AlignItems::Start,
        ..default()
    };

    let row_node = Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Start,
        align_items: AlignItems::Center,
        height: Val::Auto,
        margin: UiRect::all(Val::Px(5.0)),
        ..default()
    };

    let parent_node = Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Start,
        align_items: AlignItems::Center,
        flex_wrap: FlexWrap::Wrap,
        height: Val::Auto,
        width: Val::Percent(100.0),
        margin: UiRect::all(Val::Px(5.0)),
        ..default()
    };
    
    parent.spawn(parent_node.clone())
    .with_children(|parent| {

        // ==== Display Back Folder ===== //

        if let Some(_parent_name) = &folder.parent_name {
            parent.spawn(row_node.clone())
            .insert(Button)
            .insert(FolderName(". .".to_string()))
            .with_children(|parent| {
                parent.spawn(column_node.clone())
                .with_children(|parent| {
                    object(parent, asset_server, fonts, ". .", "Folder");
                });
            });
        }

        // ==== Display Files ===== //

        for (name, _file) in &folder.files {
            parent.spawn(row_node.clone())
            .insert(Button)
            .insert(FileName(name.clone()))
            .with_children(|parent| {
                parent.spawn(column_node.clone())
                .with_children(|parent| {
                    object(parent, asset_server, fonts, name, "File");
                });
            });
        }

        // ==== Display Folders ===== //

        for (name, _subfolder) in &folder.subfolders {
            parent.spawn(row_node.clone())
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

// ==== Update Folders/Files Displayed ===== //

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

// ==== Interaction System ===== //

pub fn button_interaction_system(
    mut interaction_query: Query<(&Interaction, &FolderName), (Changed<Interaction>, With<Button>)>,
    mut file_query: Query<(&Interaction, &FileName), (Changed<Interaction>, With<Button>)>,
    mut folder_state: ResMut<FolderState>,
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

                // ==== Go Back A Level ===== //

                if let Some(current_folder) = root.find_folder(&folder_state.current_folder) {
                    if let Some(parent_name) = &current_folder.parent_name {
                        folder_state.current_folder = parent_name.clone();
                        if let Some(parent_folder) = root.find_folder(parent_name) {
                            update_folder_ui(&mut commands, folder_ui_section.0, parent_folder, &fonts, &asset_server);
                            let path = parent_folder.get_path(&root);
                            for (_entity, mut text_input) in &mut query {
                                text_input.0 = format!("/{}/", path.clone());
                            }
                        }
                    }
                }

            } else {

                // ==== Open Folder ===== //

                folder_state.current_folder = folder_name.0.clone();
                if let Some(folder) = root.find_folder(&folder_name.0) {
                    update_folder_ui(&mut commands, folder_ui_section.0, folder, &fonts, &asset_server);
                    let path = folder.get_path(&root);
                    for (_entity, mut text_input) in &mut query {
                        text_input.0 = format!("/{}/", path.clone());
                    }
                }
            }
        }
    }

    // ==== Open File Popup ===== //

    for (interaction, file_name) in file_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if let Some(folder) = root.find_folder(&folder_state.current_folder) {
                if let Some(file) = folder.get_file(&file_name.0) {
                    folder_state.current_file_name = Some(file.name.clone());
                    popup(&mut commands, &fonts, &asset_server, &file.name, &file.content, file);
                }
            }
        }
    }
}

// ==== Folder or File Visual ===== //

pub fn object (
    parent: &mut ChildBuilder, 
    asset_server: &Res<AssetServer>,
    fonts: &Res<FontResources>,
    name: &str,
    variant: &str,
) {

    let colors = Display::new();
    let icon = if variant == "Folder" { Icon::Folder } else { Icon::File };
    parent.spawn ((
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(6.0),
            ..default()
        },
    )).with_children(|parent| {

        // ==== Icon ===== //

        parent.spawn((
            Icon::new(icon, asset_server),
            Node {
                height: Val::Px(72.0),
                width: Val::Px(72.0),
                ..default()
            },
        ));

        // ==== Label ===== //

        parent.spawn((
            Text::new(name),
            TextFont {
                font: fonts.style.text.clone(),
                font_size: fonts.size.md,
                ..default()
            },
            TextColor(colors.text_heading),
        ));
    });
}