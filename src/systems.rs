use bevy::prelude::*;

use bevy_simple_text_input::TextInputSubmitEvent;
use bevy_simple_text_input::TextInputValue;
use bevy_simple_text_input::TextInputInactive;
use bevy_simple_text_input::TextInput;

use crate::FontResources;
use crate::FolderState;
use crate::Folder;

use crate::filemanager::FolderUISection;
use crate::filemanager::SearchBar;
use crate::filemanager::update_folder_ui;
use crate::filemanager::FileName;
use crate::filemanager::FolderName;

use crate::components::popup::popup;
use crate::components::popup::TextEditor;
use crate::components::button::ButtonStyle;
use crate::components::button::InteractiveState;

use crate::theme::color::Display;
use crate::theme::color::ButtonColor;

// ==== Button Interaction System ===== //

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

// ==== Text Input System ===== //

pub fn text_input_system(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_input_query: Query<(Entity, &mut TextInputInactive, &mut BorderColor)>,
    mut events: EventReader<TextInputSubmitEvent>,
    mut editor_query: Query<&mut TextInputValue, With<TextEditor>>,
) {
    let colors = Display::new();

    // On Pressed
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

    // On Enter
    for event in events.read() {
        for mut text_input in &mut editor_query {
            text_input.0 = event.value.clone();
            text_input.0.push('\n');
        }
    }
    
}

// ==== Folder/File Interaction System ===== //

pub fn file_manager_system(
    mut folder_state: ResMut<FolderState>,
    mut commands: Commands,
    root: Res<Folder>,
    fonts: Res<FontResources>,
    asset_server: Res<AssetServer>,
    folder_ui_section: Res<FolderUISection>, 
    mut query: Query<(&TextInput, &mut TextInputValue), With<SearchBar>>, 
    mut file_query: Query<(&Interaction, &FileName), (Changed<Interaction>, With<Button>)>,
    mut interaction_query: Query<(&Interaction, &FolderName), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, folder_name) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if folder_name.0 == ". ." {

                // ==== Go Back ===== //

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
