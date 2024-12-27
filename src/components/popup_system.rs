
use bevy::prelude::*;

use bevy_simple_text_input::TextInputValue;

use crate::file_display::Folder;
use crate::file_display::FolderState;
use crate::file_display::FolderUISection;
use crate::file_display::update_folder_ui;

use crate::components::popup::Popup;
use crate::components::popup::SaveButton;
use crate::components::popup::CancelButton;
use crate::components::popup::DeleteButton;
use crate::components::popup::TextEditor;

use crate::Theme;

pub fn popup_system(
    mut commands: Commands,
    mut root: ResMut<Folder>,
    folder_state: ResMut<FolderState>,
    theme: Res<Theme>,
    folder_ui_section: Res<FolderUISection>, 
    mut text_input_query: Query<&mut TextInputValue, With<TextEditor>>,
    mut popup_query: Query<(Entity, &Node, &Children), With<Popup>>,
    mut any_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut save_query: Query<&Interaction, (Changed<Interaction>, With<SaveButton>)>,
    mut cancel_query: Query<&Interaction, (Changed<Interaction>, With<CancelButton>)>,
    mut delete_query: Query<&Interaction, (Changed<Interaction>, With<DeleteButton>)>,
) {
    for interaction in &mut cancel_query {
        if let Interaction::Pressed = *interaction {

            // ==== Close File Popup ===== //

            for (entity, _, children) in popup_query.iter_mut() {
                for child in children.iter() {
                    commands.entity(*child).despawn_recursive();
                }
                commands.entity(entity).despawn_recursive();
            }

            // ==== Update UI  ===== //

            if let Some(current_folder) = root.find_folder_mut(&folder_state.current_folder) {
                update_folder_ui(&mut commands, folder_ui_section.0, current_folder, &theme);
            }
        }
    }

    for interaction in &mut save_query {
        if let Interaction::Pressed = *interaction {

            // ==== Save ==== //

            for text_input in &mut text_input_query {
                if let Some(current_folder) = root.find_folder_mut(&folder_state.current_folder) {
                    if let Some(file_name) = &folder_state.current_file_name {
                        if let Some(file) = current_folder.get_file_mut(file_name) {
                            file.content = text_input.0.clone();
                        }
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
                update_folder_ui(&mut commands, folder_ui_section.0, current_folder, &theme);
            }
        }
    }


    for interaction in &mut delete_query {
        if let Interaction::Pressed = *interaction {

            // ==== Delete ===== //

            if let Some(file_name) = &folder_state.current_file_name {
                if let Some(current_folder) = root.find_folder_mut(&folder_state.current_folder) {
                    current_folder.files.remove(file_name);
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
                update_folder_ui(&mut commands, folder_ui_section.0, current_folder, &theme);
            }
        }
    }

}
