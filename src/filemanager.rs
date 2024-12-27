use bevy::prelude::*;
use bevy_ui::FocusPolicy;
use std::collections::HashMap;

use crate::theme::icons::Icon;
use crate::EXPAND;
use crate::theme::{color::Display, fonts::FontResources};

use bevy_simple_text_input::{
    TextInput,
    TextInputInactive,
    TextInputTextColor,
    TextInputTextFont,
    TextInputValue
};

#[derive(Component)]
pub struct SearchBar;
#[derive(Component)]
pub struct FolderName(pub String);
#[derive(Component)]
pub struct FileName(pub String);
#[derive(Default, Resource)]
pub struct RootNode(Option<Entity>);
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

/* -------------- Manager -------------- */

pub fn manager(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fonts: Res<FontResources>,
) {

    let colors = Display::new();
    let root = Folder::new("root", None);
    let mut folder_ui_section: Option<FolderUISection> = None;

    let interface = Node {
        width: EXPAND,
        height: EXPAND,
        align_items: AlignItems::Start,
        justify_content: JustifyContent::Start,
        flex_direction: FlexDirection::Row,
        ..default()
    };

    let page_node = Node {
        width: EXPAND,
        height: EXPAND,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        ..default()
    };

    let content = Node {
        width: EXPAND,
        height: EXPAND,
        max_width: Val::Px(512.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Start,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(24.0),
        ..default()
    };
    
    let root_node = commands.spawn(interface)
        .with_children(|parent| {
            parent.spawn((page_node, Interaction::None)).with_children(|parent| {

                // === Header === //

                parent.spawn(Node {
                    width: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Row,
                    padding: UiRect::all(Val::Px(24.0)),
                    ..default()
                }).with_children(|parent| {
                    parent.spawn((
                        Text::new("Web5 File Manager"),
                        TextFont {
                            font: fonts.style.heading.clone(),
                            font_size: fonts.size.h3,
                            ..default()
                        },
                        TextColor(colors.text_heading),
                    ));
                });

                // === Body === //
                
                parent.spawn(content).with_children(|parent| {

                    // === Text Input to Show current directory === //
                    
                    parent.spawn((
                        Node {
                            border: UiRect::all(Val::Px(1.0)),
                            height: Val::Px(48.0), 
                            width: Val::Percent(100.0),
                            align_items: AlignItems::Center, 
                            justify_content: JustifyContent::Start,
                            padding: UiRect::all(Val::Px(16.0)),
                            ..default()
                        },
                        TextInputTextFont(TextFont {
                            font:  fonts.style.text.clone(),
                            font_size: fonts.size.md,
                            ..default()
                        }),
                        BorderColor(colors.outline_secondary),
                        BackgroundColor(colors.bg_primary),
                        TextInputTextColor(TextColor(colors.text_primary)),
                        TextInputInactive(true),
                        TextInputValue("/root/".to_string()),
                        BorderRadius::all(Val::Px(8.0)),
                        FocusPolicy::Block,
                        TextInput,
                        SearchBar,
                    ));

                    // === Display all files and folders === //
                    let files_and_folders_node = parent.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Start,
                        width: Val::Percent(100.0),
                        ..default()
                    }).with_children(|parent| {
                        display_files_and_folders(parent, &root, &fonts, &asset_server);
                    }).id();

                    folder_ui_section = Some(FolderUISection(files_and_folders_node));
                });
            });
        }).id();

    if let Some(folder_ui_section) = folder_ui_section {
        commands.insert_resource(folder_ui_section);
    }

    commands.insert_resource(RootNode(Some(root_node)));
    commands.insert_resource(root);
    commands.insert_resource(FolderState::new());
}

/* -------------- Display Files And Folders -------------- */

pub fn display_files_and_folders(
    parent: &mut ChildBuilder,
    folder: &Folder,
    fonts: &Res<FontResources>,
    asset_server: &Res<AssetServer>,
) {

    // ==== Design Nodes ===== //

    let column_node = Node {
        margin: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Start,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(6.0),
        align_items: AlignItems::Start,
        ..default()
    };

    let row_node = Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Start,
        align_items: AlignItems::Center,
        height: Val::Auto,
        margin: UiRect::all(Val::Px(5.0)),
        ..default()
    };
    
    parent.spawn(Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Start,
        align_items: AlignItems::Center,
        flex_wrap: FlexWrap::Wrap,
        height: Val::Auto,
        width: Val::Percent(100.0),
        margin: UiRect::all(Val::Px(5.0)),
        ..default()
    }).with_children(|parent| {

        // ==== Display Back Folder ===== //

        if let Some(_parent_name) = &folder.parent_name {
            parent.spawn(row_node.clone())
            .insert(Button)
            .insert(FolderName(". .".to_string()))
            .with_children(|parent| {
                parent.spawn(column_node.clone())
                .with_children(|parent| {
                    object(parent, asset_server, fonts, ". .", Icon::Folder);
                });
            });
        }

        // ==== Display Files ===== //

        for name in folder.files.keys() {
            parent.spawn(row_node.clone())
            .insert(Button)
            .insert(FileName(name.clone()))
            .with_children(|parent| {
                parent.spawn(column_node.clone())
                .with_children(|parent| {
                    object(parent, asset_server, fonts, name, Icon::File);
                });
            });
        }

        // ==== Display Folders ===== //

        for name in folder.subfolders.keys() {
            parent.spawn(row_node.clone())
            .insert(Button)
            .insert(FolderName(name.clone()))
            .with_children(|parent| {
                parent.spawn(column_node.clone())
                .with_children(|parent| {
                    object(parent, asset_server, fonts, name, Icon::Folder);
                });
            });
        }
    });
}

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
