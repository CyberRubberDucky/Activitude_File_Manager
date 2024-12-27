
use bevy::prelude::*;
use bevy_simple_text_input::TextInputValue;

use crate::theme::fonts::FontResources;

use crate::file_display::Folder;
use crate::file_display::FolderState;
use crate::file_display::update_folder_ui;

use crate::components::popup::Popup;
use crate::components::popup::SaveButton;
use crate::components::popup::DeleteButton;
use crate::components::popup::TextEditor;

use crate::file_manager::FolderUISection;

pub fn popup_system(
    save_query: Query<&SaveButton>,
    delete_query: Query<&DeleteButton>, 
    mut root: ResMut<Folder>,
    mut commands: Commands,
    fonts: Res<FontResources>,
    asset_server: Res<AssetServer>,
    folder_state: ResMut<FolderState>,
    folder_ui_section: Res<FolderUISection>, 
    mut text_input_query: Query<&mut TextInputValue, With<TextEditor>>,
    mut popup_query: Query<(Entity, &Node, &Children), With<Popup>>,
    mut interaction_query: Query<(&Interaction, &Parent), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, parent) in &mut interaction_query {
        if let Interaction::Pressed = *interaction {

            // ==== Save or Delete ===== //

            if save_query.get(parent.get()).is_ok() {
                for text_input in &mut text_input_query {
                    if let Some(current_folder) = root.find_folder_mut(&folder_state.current_folder) {
                        if let Some(file_name) = &folder_state.current_file_name {
                            if let Some(file) = current_folder.get_file_mut(file_name) {
                                file.content = text_input.0.clone();
                            }
                        }
                    }
                }
            } else if delete_query.get(parent.get()).is_ok() {
                if let Some(file_name) = &folder_state.current_file_name {
                    if let Some(current_folder) = root.find_folder_mut(&folder_state.current_folder) {
                        current_folder.files.remove(file_name);
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