use bevy::prelude::*;

use crate::Theme;
use crate::popup::popup;

use ramp_ds::prelude::*;
use ramp_ds::traits::Component;

use crate::objects::{
    Folder,
    FilesAndFolders,
};

#[derive(Default, Resource)]
pub struct RootNode(Option<Entity>);
#[derive(Default, Resource)]
pub struct FolderState(pub Folder, pub Option<String>);
#[derive(Component)]
pub struct UISection;

pub fn screen(
    mut commands: Commands,
    theme: Res<Theme>,
) {
    let root_folder = Folder::new("root", None);
    let root = Interface::new(
        false, 
        Page::new(
            Header::new("Web5 File Manager", Size::Large, None, None, false),
            Content(JustifyContent::Start, vec![
                Box::new(TextInput::new("/root/")),
                Box::new(FilesAndFolders(root_folder.clone())),
            ]),        
            None
        )
    ).spawn(&mut commands, &theme);
    
    commands.insert_resource(RootNode(Some(root)));
    commands.insert_resource(root_folder);
}

pub fn update_folder_ui(
    commands: &mut Commands,
    folder_state: &mut ResMut<FolderState>,
    folder_ui_query: &Query<(Entity, &Parent), With<UISection>>,
    folder: Folder,
    theme: &Res<Theme>,
) {
    folder_state.0 = folder.clone();
    for (ui_section_entity, parent) in folder_ui_query {
        let parent_entity = parent.get();
        commands.entity(ui_section_entity).despawn_recursive();
        commands.entity(parent_entity).with_children(|parent| {
            FilesAndFolders(folder.clone()).box_spawn(parent, theme);
        });
    }
}

pub fn file_manager(
    mut root: ResMut<Folder>,
    theme: Res<Theme>,
    mut commands: Commands,
    mut folder_state: ResMut<FolderState>,
    folder_ui_query: Query<(Entity, &Parent), With<UISection>>,
    mut text_input: Query<&mut bevy_simple_text_input::TextInputValue>,
    mut interaction_query: Query<(
        &Interaction,
        &IconTextButton,
    ), (Changed<Interaction>, With<bevy::prelude::Button>)>,
) {
    for (interaction, button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            let mut new_path = None;

            match button.0.as_str() {
                ". ." => {
                    if let Some(parent) = &folder_state.0.parent {
                        if let Some(parent_folder) = root.folder_from(parent) {
                            update_folder_ui(&mut commands, &mut folder_state, &folder_ui_query, parent_folder.clone(), &theme);
                            new_path = Some(parent_folder.get_path(&root));
                        }
                    }
                }
                _ if button.1 => {
                    if let Some(current_folder) = root.get(&folder_state.0) {
                        if let Some(subfolder) = current_folder.folder_from(&button.0) {
                            update_folder_ui(&mut commands, &mut folder_state, &folder_ui_query, subfolder.clone(), &theme);
                            new_path = Some(subfolder.get_path(&root));
                        }
                    }
                }
                _ => {
                    if let Some(current_folder) = root.get(&folder_state.0) {
                        if let Some(file) = current_folder.files.get(&button.0).cloned() {
                            folder_state.1 = Some(file.name.clone());
                            popup(&mut commands, &theme, &file.name, &file.content);
                        }
                    }
                }
            }

            if let Some(path) = new_path {
                for mut input in &mut text_input {
                    input.0 = path.clone();
                }
            }
        }
    }
}
