use bevy::prelude::*;

use crate::Theme;
use crate::objects::Folder;
use crate::objects::File;

use crate::file_manager::UISection;
use crate::file_manager::FolderState;
use crate::file_manager::update_folder_ui;

use ramp_ds::components::button::Callback;

pub fn context_system(
    mut commands: Commands,
    theme: Res<Theme>,
    mut root: ResMut<Folder>,
    mut folder_state: ResMut<FolderState>,
    mut interaction_query: Query<
        (&Interaction, &Callback),
        (Changed<Interaction>, With<bevy::prelude::Button>),
    >,
    folder_ui_query: Query<(Entity, &Parent), With<UISection>>,
) {
    for (interaction, callback) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            let result = (callback.0)();

            if let Some(current_folder) = root.get(&folder_state.0) {
                match result.as_str() {
                    "Create File" => {
                        let file_name = format!("file{}.txt", current_folder.files.len() + 1);
                        current_folder.files.insert(file_name.clone(), File::new(file_name));
                    },
                    "Create Folder" => {
                        let folder_name = format!("folder {}", current_folder.subfolders.len() + 1);
                        current_folder.subfolders.insert(folder_name.clone(), Folder::new(&folder_name, Some(current_folder.name.clone())));
                    },
                    _ => continue,
                }

                update_folder_ui(&mut commands, &mut folder_state, &folder_ui_query, current_folder.clone(), &theme);
            }
        }
    }
}
