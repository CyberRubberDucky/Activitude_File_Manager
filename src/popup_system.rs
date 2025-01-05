
use bevy::prelude::*;
use bevy_simple_text_input::TextInputValue;

use ramp_ds::prelude::*;
use ramp_ds::components::button::Callback;

use crate::objects::Folder;
use crate::file_manager::FolderState;
use crate::file_manager::update_folder_ui;
use crate::file_manager::UISection;

use crate::Theme;
use crate::popup::Popup;

pub fn popup_system(
    mut commands: Commands,
    theme: Res<Theme>,
    mut root: ResMut<Folder>,
    mut folder_state: ResMut<FolderState>,
    folder_ui_query: Query<(Entity, &Parent), With<UISection>>,
    mut text_input_query: Query<&mut TextInputValue, With<TextEditor>>,
    mut popup_query: Query<(Entity, &Node, &Children), With<Popup>>,
    mut interaction_query: Query<
        (&Interaction, &Callback),
        (Changed<Interaction>, With<bevy::prelude::Button>),
    >,
) {
    for (interaction, callback) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            let result = (callback.0)();

            match result.as_str() {
                "Delete" => {
                    if let Some(current_folder) = root.get(&folder_state.0) {
                        if let Some(name) = folder_state.1.clone() {
                            current_folder.files.remove(&name);
                        }
                        folder_state.0 = current_folder.clone();
                    }
                }
                "Save" => {
                    if let Some(current_folder) = root.get(&folder_state.0) {
                        if let Some(file_name) = folder_state.1.clone() {
                            if let Some(file) = current_folder.files.get_mut(&file_name) {
                                if let Some(text_input) = text_input_query.iter_mut().next() {
                                    file.content = text_input.0.clone();
                                    folder_state.0 = current_folder.clone();
                                }
                            }
                        }
                    }
                }
                "Cancel" => {}
                _ => continue,
            }

            close_popup(&mut commands, &mut folder_state, &theme, &folder_ui_query, &mut popup_query);
        }
    }
}

pub fn close_popup(
    commands: &mut Commands,
    folder_state: &mut ResMut<FolderState>,
    theme: &Res<Theme>,
    folder_ui_query: &Query<(Entity, &Parent), With<UISection>>,
    popup_query: &mut Query<(Entity, &Node, &Children), With<Popup>>,
) {
    for (entity, _, children) in popup_query.iter_mut() {
        for child in children.iter() {
            commands.entity(*child).despawn_recursive();
        }
        commands.entity(entity).despawn_recursive();
    }
    update_folder_ui(commands, folder_state, folder_ui_query, folder_state.0.clone(), theme);
}
