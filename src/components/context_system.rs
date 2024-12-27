use bevy::prelude::*;

use crate::theme::fonts::FontResources;

use crate::file_display::Folder;
use crate::file_display::FolderState;
use crate::file_display::File;
use crate::file_display::update_folder_ui;

use crate::file_manager::FolderUISection;

use crate::components::context::NewFileButton;
use crate::components::context::NewFolderButton;

pub fn context_system(
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