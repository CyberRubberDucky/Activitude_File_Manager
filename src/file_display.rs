use bevy::prelude::*;
use std::collections::HashMap;

use crate::components::popup::popup;

use crate::file_manager::SearchBar;
use crate::file_manager::display_files_and_folders;

use bevy_simple_text_input::TextInput;
use bevy_simple_text_input::TextInputValue;

use crate::Theme;

#[derive(Component)]
pub struct FolderName(pub String);
#[derive(Component)]
pub struct FileName(pub String);
#[derive(Resource)]
pub struct FolderUISection(pub Entity);

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub content: String, 
}

#[derive(Default, Resource)]
pub struct FolderState {
    pub current_folder: String,
    pub current_file_name: Option<String>,
}

impl FolderState {
    pub fn new() -> Self {
        FolderState {
            current_folder: "root".to_string(),
            current_file_name: None,
        }
    }
}


/* -------------- Display Files And Folders -------------- */

pub fn update_folder_ui(
    commands: &mut Commands,
    folder_ui_section: Entity,
    folder: &Folder,
    theme: &Res<Theme>,
) {
    commands.entity(folder_ui_section).despawn_descendants();
    commands.entity(folder_ui_section).with_children(|parent| {
        display_files_and_folders(parent, folder, theme);
    });
}

// ==== Folder or File Visual ===== //

pub fn object(
    parent: &mut ChildBuilder, 
    theme: &Res<Theme>,
    name: &str,
    icon: ImageNode,
) {

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
            icon,
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
                font: theme.fonts.style.text.clone(),
                font_size: theme.fonts.size.md,
                ..default()
            },
            TextColor(theme.colors.text_heading),
        ));
    });
}


#[derive(Debug, Clone, Resource)]
pub struct Folder {
    pub name: String,
    pub parent_name: Option<String>,
    pub files: HashMap<String, File>,
    pub subfolders: HashMap<String, Folder>,
}

impl Default for Folder {
    fn default() -> Self {
        Folder::new("root", None)
    }
}

impl Folder {
    pub fn new(name: &str, parent_name: Option<String>) -> Self {
        Self {
            name: name.to_string(),
            parent_name,
            files: HashMap::new(),
            subfolders: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, file: File) {
        self.files.insert(file.name.clone(), file);
    }

    pub fn add_subfolder(&mut self, folder: Folder) {
        self.subfolders.insert(folder.name.clone(), folder);
    }

    pub fn get_file(&self, name: &str) -> Option<&File> {
        self.files.get(name)
    }

    pub fn get_file_mut(&mut self, name: &str) -> Option<&mut File> {
        self.files.get_mut(name)
    }

    pub fn _remove_subfolder(&mut self, name: &str) -> Option<Folder> {
        self.subfolders.remove(name)
    }

    pub fn find_folder(&self, name: &str) -> Option<&Folder> {
        if self.name == name { return Some(self); }

        for subfolder in self.subfolders.values() {
            if let Some(found) = subfolder.find_folder(name) {
                return Some(found);
            }
        }

        None
    }

    pub fn find_folder_mut(&mut self, name: &str) -> Option<&mut Folder> {
        if self.name == name {
            return Some(self);
        }

        for subfolder in self.subfolders.values_mut() {
            if let Some(found) = subfolder.find_folder_mut(name) {
                return Some(found);
            }
        }

        None
    }

    pub fn get_path(&self, root: &Folder) -> String {
        if let Some(parent_name) = &self.parent_name {
            if let Some(parent_folder) = root.find_folder(parent_name) {
                return format!("{}/{}", parent_folder.get_path(root), self.name);
            }
        }
        self.name.clone()
    }
}



// ==== Folder/File Interaction System ===== //

pub fn file_manager_system(
    root: Res<Folder>,
    theme: Res<Theme>,
    folder_ui_section: Res<FolderUISection>, 
    mut commands: Commands,
    mut folder_state: ResMut<FolderState>,
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
                            update_folder_ui(&mut commands, folder_ui_section.0, parent_folder, &theme);
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
                    update_folder_ui(&mut commands, folder_ui_section.0, folder, &theme);
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
                    popup(&mut commands, &theme, &file.name, &file.content);
                }
            }
        }
    }
}
