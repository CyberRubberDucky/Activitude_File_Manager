use bevy::{prelude::*, ui::FocusPolicy};

use super::despawn_screen;

use crate::{
    menu_plugin,
    NavigateTo
};

use crate::theme::{
    color::Display,
    fonts::FontResources,
    icons::Icon,
};

use crate::interface::{
    header::{ header, Header },
    bumper::Bumper,
    interfaces::Interface
};

use crate::primitives::{
    button::{
        ButtonComponent,
        primary_default,
        secondary_default,
        button_system,
    },
};

use crate::components::{
    text_input::text_input,
    text_editor::text_editor
};

use bevy_simple_text_input::{
    TextInput, 
    TextInputTextFont,
    TextInputTextColor,
    TextInputPlaceholder,
    TextInputInactive,
};

use std::collections::HashMap;

#[derive(Component)]
pub struct OnFileManagerScreen;


#[derive(Component)]
pub struct FolderName(pub String);

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub content: String, 
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

#[derive(Default, Resource)]
pub struct FolderState {
    pub current_folder: String,
}

#[derive(Default, Resource)]
pub struct RootNode(pub Option<Entity>);

#[derive(Resource)]
pub struct FolderUISection(pub Entity);

pub fn manager(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    fonts: Res<FontResources>,
) {
    let colors = Display::new();
    let bumper = Bumper::new();
    let interface = Interface::new();

    let mut root = Folder::new("root", None);

    root.add_file(File {
        name: "file1.txt".to_string(),
        content: "Hello, world!".to_string(),
    });

    root.add_file(File {
        name: "file2.txt".to_string(),
        content: "Rust is awesome!".to_string(),
    });

    let mut subfolder = Folder::new("subfolder", Some("root".to_string()));

    subfolder.add_file(File {
        name: "file4.txt".to_string(),
        content: "Subfolder file!".to_string(),
    });

    root.add_subfolder(subfolder);

    root.add_file(File {
        name: "file3.txt".to_string(),
        content: "Rust is terrible!".to_string(),
    });

    let mut folder_ui_section: Option<FolderUISection> = None;

    let root_node = commands.spawn((
        interface.node,
        OnFileManagerScreen,
    )).with_children(|parent| {
        parent.spawn((interface.page_node, Interaction::None)).with_children(|parent| {
            header(parent, &fonts, &asset_server, Header::Home, "Web5 File Manager");
            parent.spawn(interface.content).with_children(|parent| { 
                text_input(parent, &fonts);
                
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
    commands.insert_resource(FolderState::default());
}

fn display_files_and_folders(
    parent: &mut ChildBuilder,
    folder: &Folder,
    fonts: &Res<FontResources>,
    asset_server: &Res<AssetServer>,
) {
    let colors = Display::new();

    let column_node = Node {
        margin: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(6.0),
        align_items: AlignItems::Center,
        ..default()
    };

    let row_node = Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Start,
        align_items: AlignItems::Center,
        width: Val::Percent(100.0),
        height: Val::Auto,
        margin: UiRect::all(Val::Px(5.0)),
        ..default()
    };

    if let Some(parent_name) = &folder.parent_name {
        parent.spawn(column_node.clone())
        .insert(Button)
        .insert(FolderName(". .".to_string()))
        .with_children(|button| {
            object(button, asset_server, fonts, ". .", "Folder");
        });
    }

    parent.spawn(row_node.clone())
    .with_children(|parent| {
        for (name, file) in &folder.files {
            parent.spawn(column_node.clone())
            .with_children(|button| {
                object(button, asset_server, fonts, name, "File");
            });
        }

        for (name, subfolder) in &folder.subfolders {
            parent
            .spawn(row_node.clone())
            .insert(Button)
            .insert(FolderName(name.clone()))
            .with_children(|parent| {
                parent.spawn(column_node.clone())
                .with_children(|parent| {
                    object(parent, asset_server, fonts, name, "Folder");
                });
            });
        }
    });
}

fn object (
    parent: &mut ChildBuilder, 
    asset_server: &Res<AssetServer>,
    fonts: &Res<FontResources>,
    name: &str,
    variant: &str,
) {
    let colors = Display::new();

    let icon = if variant == "Folder" {
        Icon::Folder
    } else {
        Icon::File
    };

    parent.spawn((
        Icon::new(icon, asset_server),
        Node {
            height: Val::Px(72.0),
            width: Val::Px(72.0),
            ..default()
        },
    ));

    parent.spawn((
        Text::new(name),
        TextFont {
            font: fonts.style.text.clone(),
            font_size: fonts.size.md,
            ..default()
        },
        TextColor(colors.text_heading),
    ));
}

fn folder_clicked(
    folder_state: &mut ResMut<FolderState>,
    folder_ui_section: Entity,
    commands: &mut Commands,
    folder_name: String,
    root: &Folder,
    fonts: &Res<FontResources>,
    asset_server: &Res<AssetServer>,
    mut text_input_query: Query<(Entity, &mut TextInputInactive, &mut BorderColor)>,
) {
    if folder_name == ". ." {
        if let Some(current_folder) = root.find_folder(&folder_state.current_folder) {
            if let Some(parent_name) = &current_folder.parent_name {
                folder_state.current_folder = parent_name.clone();
                if let Some(parent_folder) = root.find_folder(parent_name) {
                    update_folder_ui(commands, folder_ui_section, parent_folder, fonts, asset_server);
                    let path = parent_folder.get_path(root);

                }
            }
        }
    } else {
        folder_state.current_folder = folder_name.clone();
        if let Some(folder) = root.find_folder(&folder_name) {
            update_folder_ui(commands, folder_ui_section, folder, fonts, asset_server);
            let path = folder.get_path(root);
            println!("{}/", path);
        }
    }
}


pub fn focus(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_input_query: Query<(Entity, &mut TextInputInactive, &mut BorderColor)>,
) {
    let colors = Display::new();
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for (entity, mut inactive, mut border_color) in &mut text_input_query {
                if entity == interaction_entity {
                    inactive.0 = false;
                    *border_color = colors.outline_primary.into();
                } else {
                    inactive.0 = true;
                    *border_color = colors.outline_secondary.into();
                }
            }
        }
    }
}


fn update_folder_ui(
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

pub fn button_interaction_system(
    mut interaction_query: Query<(&Interaction, &FolderName), (Changed<Interaction>, With<Button>)>,
    mut folder_state: ResMut<FolderState>,
    root_node: Res<RootNode>,
    mut commands: Commands,
    root: Res<Folder>,
    fonts: Res<FontResources>,
    asset_server: Res<AssetServer>,
    folder_ui_section: Res<FolderUISection>, 
) {
    for (interaction, folder_name) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            folder_clicked(
                &mut folder_state,
                folder_ui_section.0, 
                &mut commands,
                folder_name.0.clone(),
                &root,
                &fonts,
                &asset_server,
            );
        }
    }
}
