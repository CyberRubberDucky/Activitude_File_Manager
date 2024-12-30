use bevy::prelude::*;

use crate::Theme;

use crate::file_display::Folder;
use crate::file_display::FolderState;
use crate::file_display::File;
use crate::file_display::update_folder_ui;
use crate::file_display::FolderUISection;

use crate::context::NewFileButton;
use crate::context::NewFolderButton;

pub fn context_system(
    mut commands: Commands,
    theme: Res<Theme>,
    mut root: ResMut<Folder>,
    folder_state: ResMut<FolderState>,
    folder_ui_section: Res<FolderUISection>,
    mut file_query: Query<&Interaction, (Changed<Interaction>, With<NewFileButton>)>,
    mut folder_query: Query<&Interaction, (Changed<Interaction>, With<NewFolderButton>)>,
) {
    for interaction in &mut file_query {
        if let Interaction::Pressed = *interaction {

            // ==== On File Creation ===== //
            if let Some(current_folder) = root.find_folder_mut(&folder_state.current_folder) {

                // ==== Generate Name ===== //

                let new_file_name = format!("file{}.txt", current_folder.files.len() + 1);

                // ==== Add file to Current Folder ===== //

                current_folder.add_file(File {
                    name: new_file_name.clone(),
                    content: String::new(),
                });

                // ==== Update UI ===== //

                update_folder_ui(&mut commands, folder_ui_section.0, current_folder, &theme);
            }
        }
    }

    for interaction in &mut folder_query {
        if let Interaction::Pressed = *interaction {

            // ==== On Folder Creation ===== //

        
            if let Some(current_folder) = root.find_folder_mut(&folder_state.current_folder) {
                let new_folder_name = format!("folder {}", current_folder.subfolders.len() + 1);

                current_folder.add_subfolder(Folder::new(&new_folder_name, Some(folder_state.current_folder.clone())));

                update_folder_ui(&mut commands, folder_ui_section.0, current_folder, &theme);
            }
        }
    }
}