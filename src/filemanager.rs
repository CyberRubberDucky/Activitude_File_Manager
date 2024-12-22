use bevy::{prelude::*, ui::FocusPolicy};

use super::despawn_screen;


use crate::theme::{
    color::Display,
    fonts::FontResources,
    icons::Icon,
};

use crate::interface::{
    header::{ header, Header },
    interfaces::Interface
};

use crate::FolderState;
use crate::RootNode;
use crate::Folder;
use crate::file_manager::folder::FolderUISection;
use crate::file_manager::manager_ui::display_files_and_folders;
use crate::file_manager::folder::File;

use crate::components::{
    text_input::{text_input, SearchBar},
    text_editor::text_editor
};

use bevy_simple_text_input::{
    TextInput, 
    TextInputTextFont,
    TextInputTextColor,
    TextInputPlaceholder,
    TextInputInactive,
    TextInputValue,
};

use std::collections::HashMap;

#[derive(Component)]
pub struct OnFileManagerScreen;


pub fn manager(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    fonts: Res<FontResources>,
) {
    let colors = Display::new();
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
