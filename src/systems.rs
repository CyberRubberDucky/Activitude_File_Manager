use bevy::prelude::*;
use crate::components::button::ButtonStyle;
use crate::components::button::InteractiveState;
use crate::theme::color::{ButtonColor, Display};
use crate::FontResources;
use crate::components::context::NewFileButton;
use bevy_simple_text_input::TextInputSubmitEvent;
use crate::components::context::NewFolderButton;
use crate::filemanager::SearchBar;
use crate::filemanager::update_folder_ui;
use crate::components::popup::popup;
use crate::filemanager::FileName;
use crate::filemanager::FolderName;
use crate::filemanager::File;
use crate::components::popup::Popup;
use crate::components::popup::SaveButton;
use crate::components::popup::CancelButton;
use crate::components::popup::DeleteButton;
use crate::components::popup::TextEditor;
use bevy_simple_text_input::TextInputValue;
use bevy_simple_text_input::TextInputInactive;
use bevy_simple_text_input::TextInput;
use crate::FolderState;
use crate::Folder;
use crate::filemanager::FolderUISection;

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            Option<&ButtonStyle>,
            &InteractiveState,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color, button_style, state) in &mut interaction_query {
        if *state != InteractiveState::Disabled && *state != InteractiveState::Selected {
            if let Some(button_style) = button_style {
                match *interaction {
                    Interaction::Hovered => {
                        let colors: ButtonColor = ButtonColor::new(*button_style, InteractiveState::Hover);
                        *color = colors.background.into();
                        border_color.0 = colors.outline;
                    }
                    Interaction::None => {
                        let colors: ButtonColor = ButtonColor::new(*button_style, InteractiveState::Default);
                        *color = colors.background.into();
                        border_color.0 = colors.outline;
                    }
                    Interaction::Pressed => {
                        let colors: ButtonColor = ButtonColor::new(*button_style, InteractiveState::Selected);
                        *color = colors.background.into();
                        border_color.0 = colors.outline;
                    }
                }
            }
        }
    }
}

pub fn create_system(
    mut commands: Commands,
    fonts: Res<FontResources>,
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<(&Interaction, &Parent), (Changed<Interaction>, With<Button>)>,
    file_query: Query<&NewFileButton>,
    folder_query: Query<&NewFolderButton>,
    mut root: ResMut<Folder>,
    folder_state: ResMut<FolderState>,
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


pub fn popup_system(
    mut commands: Commands,
    mut popup_query: Query<(Entity, &Node, &Children), With<Popup>>,
    mut interaction_query: Query<(&Interaction, &Parent), (Changed<Interaction>, With<Button>)>,
    s_query: Query<&SaveButton>,
    c_query: Query<&CancelButton>,
    delete_query: Query<&DeleteButton>, 
    mut query: Query<&mut TextInputValue, With<TextEditor>>,
    mut root: ResMut<Folder>, 
    folder_state: ResMut<FolderState>,
    folder_ui_section: Res<FolderUISection>, 
    fonts: Res<FontResources>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, parent) in &mut interaction_query {
        if let Interaction::Pressed = *interaction {
            if s_query.get(parent.get()).is_ok() || c_query.get(parent.get()).is_ok() || delete_query.get(parent.get()).is_ok() {
                if s_query.get(parent.get()).is_ok() {

                    // ==== If Saving File ===== //

                    for text_input in &mut query {

                        // ==== Get Text Input ===== //

                        if let Some(current_folder) = root.find_folder_mut(&folder_state.current_folder) {
                            if let Some(file_name) = &folder_state.current_file_name {
                                if let Some(file) = current_folder.get_file_mut(file_name) {

                                    // ==== Set File Content To Text Input ===== //

                                    file.content = text_input.0.clone();
                                }
                            }
                        }
                    }
                } else if c_query.get(parent.get()).is_ok() {

                    // ==== Cancel (Do nothing) ===== //
                } else if delete_query.get(parent.get()).is_ok() {

                    // ==== Deleting File or Folder ===== //

                    if let Some(file_name) = &folder_state.current_file_name {
                        if let Some(current_folder) = root.find_folder_mut(&folder_state.current_folder) {
                            current_folder.files.remove(file_name);
                        }
                    } else {
                        let folder_name = folder_state.current_folder.clone();
                        if let Some(current_folder) = root.find_folder_mut(&folder_name) {
                            current_folder.subfolders.remove(&folder_name);
                        }
                    }
                }

                // ==== Close File Popup ===== //

                for (entity, _, children) in popup_query.iter_mut() {
                    for child in children.iter() {
                        commands.entity(*child).despawn_recursive();
                    }
                    commands.entity(entity).despawn_recursive();
                }

                // ==== Update UI  ===== //

                if let Some(current_folder) = root.find_folder_mut(&folder_state.current_folder) {
                    update_folder_ui(&mut commands, folder_ui_section.0, current_folder, &fonts, &asset_server);
                }
            }
        }
    }
}

pub fn listener(
    mut events: EventReader<TextInputSubmitEvent>,
    mut query: Query<&mut TextInputValue, With<TextEditor>>, 
) {
    for event in events.read() {
        for mut text_input in &mut query {
            text_input.0 = event.value.clone();
            text_input.0.push('\n');
        }
    }
}

pub fn focus(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_input_query: Query<(Entity, &mut TextInputInactive, &mut BorderColor)>,
) {
    let colors = Display::new();
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for (entity, mut inactive, mut border_color) in &mut text_input_query {
                if entity == interaction_entity {
                    inactive.0 = false;
                    *border_color = colors.outline_primary.into();
                } else {
                    inactive.0 = true;
                    *border_color = colors.outline_secondary.into();
                }
            }
        }
    }
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
                    popup(&mut commands, &fonts, &asset_server, &file.name, &file.content);
                }
            }
        }
    }
}
