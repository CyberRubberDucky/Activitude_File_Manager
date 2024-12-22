use bevy::prelude::*;

use std::collections::HashMap;

#[derive(Component)]
pub struct FolderName(pub String);


#[derive(Component)]
pub struct FileName(pub String);

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub content: String, 
}

#[derive(Default, Resource)]
pub struct FolderState {
    pub current_folder: String,
}

impl FolderState {
    pub fn new() -> Self {
        FolderState {
            current_folder: "root".to_string(),
        }
    }
}

#[derive(Default, Resource)]
pub struct RootNode(pub Option<Entity>);

#[derive(Resource)]
pub struct FolderUISection(pub Entity);

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

    pub fn get_subfolder(&self, name: &str) -> Option<&Folder> {
        self.subfolders.get(name)
    }

    pub fn remove_file(&mut self, name: &str) -> Option<File> {
        self.files.remove(name)
    }

    pub fn remove_subfolder(&mut self, name: &str) -> Option<Folder> {
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

    pub fn get_path(&self, root: &Folder) -> String {
        if let Some(parent_name) = &self.parent_name {
            if let Some(parent_folder) = root.find_folder(parent_name) {
                return format!("{}/{}", parent_folder.get_path(root), self.name);
            }
        }
        self.name.clone()
    }
}
