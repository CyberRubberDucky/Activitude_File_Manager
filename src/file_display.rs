use bevy::prelude::*;
use std::collections::HashMap;

use crate::theme::icons::Icon;
use crate::theme::color::Display;
use crate::theme::fonts::FontResources;

use crate::file_manager::display_files_and_folders;

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
    fonts: &Res<FontResources>,
    asset_server: &Res<AssetServer>,
) {
    commands.entity(folder_ui_section).despawn_descendants();
    commands.entity(folder_ui_section).with_children(|parent| {
        display_files_and_folders(parent, folder, fonts, asset_server);
    });
}

// ==== Folder or File Visual ===== //

pub fn object (
    parent: &mut ChildBuilder, 
    asset_server: &Res<AssetServer>,
    fonts: &Res<FontResources>,
    name: &str,
    icon: Icon,
) {

    let colors = Display::new();
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
            Icon::new(icon, asset_server),
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
                font: fonts.style.text.clone(),
                font_size: fonts.size.md,
                ..default()
            },
            TextColor(colors.text_heading),
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
