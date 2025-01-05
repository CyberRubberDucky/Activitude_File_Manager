use bevy::prelude::*;
use ramp_ds::prelude::*;
use ramp_ds::traits::Component;
use std::collections::BTreeMap;
use crate::Theme;
use crate::file_manager::UISection;

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    pub name: String,
    pub content: String, 
}

impl File {
    pub fn new(name: String) -> Self {
        Self {
            name,
            content: String::new(),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Resource)]
pub struct Folder {
    pub name: String,
    pub parent: Option<String>, 
    pub files: BTreeMap<String, File>,
    pub subfolders: BTreeMap<String, Folder>, 
}

impl Default for Folder {
    fn default() -> Self {
        Folder::new("root", None)
    }
}

impl Folder {
    pub fn new(name: &str, parent: Option<String>) -> Self {
        Self {
            name: name.to_string(),
            parent,
            files: BTreeMap::new(),
            subfolders: BTreeMap::new(),
        }
    }

    pub fn get(&mut self, folder: &Folder) -> Option<&mut Folder> {
        if self == folder {return Some(self)}
        for subfolder in self.subfolders.values_mut() {
            if subfolder == folder {
                return Some(subfolder);
            }
        }
        None
    }    

    pub fn folder_from(&self, name: &str) -> Option<Folder> {
        if self.name == name {return Some(self.clone())}
        for subfolder in self.subfolders.values() {
            if subfolder.name == name {
                return Some(subfolder.clone());
            }
        }
        None
    }

    pub fn get_path(&self, root: &Folder) -> String {
        if let Some(parent_name) = &self.parent {
            if let Some(parent_folder) = root.folder_from(parent_name) {
                return format!("/{}/{}/", parent_folder.get_path(root), self.name);
            }
        }
        self.name.clone()
    }
}

pub struct FilesAndFolders(pub Folder);

impl Component for FilesAndFolders {
    fn spawn(self: Box<Self>, parent: &mut ChildBuilder<'_>, theme: &Res<Theme>) {
        parent.spawn((Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Start,
            margin: UiRect::all(Val::Px(5.0)),
            flex_wrap: FlexWrap::Wrap,
            width: EXPAND,
            ..default()
        }, UISection)).with_children(|parent| {
            if self.0.parent.is_some() {
                IconTextButton::new(". .", true).box_spawn(parent, theme);
            }

            for name in self.0.files.keys() {
                IconTextButton::new(name, false).box_spawn(parent, theme);
            }

            for name in self.0.subfolders.keys() {
                IconTextButton::new(name, true).box_spawn(parent, theme);
            }
        });
    }
}