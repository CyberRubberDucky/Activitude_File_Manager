use bevy::prelude::*;
use crate::theme::{color::Display, fonts::FontResources};
use crate::{FolderState, RootNode, Folder};
use crate::{folder::{FolderUISection, File}, manager_ui::display_files_and_folders};
use crate::components::text_input::text_input;

#[derive(Component)]
pub struct OnFileManagerScreen;

const EXPAND: Val = Val::Percent(100.0);

pub fn manager(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fonts: Res<FontResources>,
) {

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

    let node = Node {
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
    
    let root_node = commands.spawn(
        (node, OnFileManagerScreen)
    ).with_children(|parent| {
        parent.spawn((page_node, Interaction::None)).with_children(|parent| {
            large_header("Web5 File Manager", parent, &fonts);
            parent.spawn(content).with_children(|parent| {
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
    commands.insert_resource(FolderState::new());
}

pub fn large_header(
    title: &str,
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
) {
    let colors = Display::new();

    let node = Node {
        width: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Row,
        padding: UiRect::all(Val::Px(24.0)),
        ..default()
    };

    parent.spawn(node).with_children(|parent| {
        parent.spawn((
            Text::new(title),
            TextFont {
                font: fonts.style.heading.clone(),
                font_size: fonts.size.h3,
                ..default()
            },
            TextColor(colors.text_heading),
        ));
    });
}
