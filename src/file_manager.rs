use bevy::prelude::*;
use bevy_ui::FocusPolicy;


use crate::Folder;
use crate::EXPAND;

use crate::popup::TextEditor;
use ramp_ds::layout::{
    interface::{Interface, Page},
    header::Header,
    content::Content,
    bumper::Bumper,
    utils::Size
};

use crate::file_display::object;
use crate::file_display::FolderState;
use crate::file_display::FolderUISection;
use crate::file_display::FolderName;
use crate::file_display::FileName;

use crate::Theme;

use bevy_simple_text_input::{
    TextInput,
    TextInputInactive,
    TextInputTextColor,
    TextInputTextFont,
    TextInputSubmitEvent,
    TextInputValue
};

#[derive(Component)]
pub struct SearchBar;
#[derive(Default, Resource)]
pub struct RootNode(Option<Entity>);

pub fn manager(
    mut commands: Commands,
    theme: Res<Theme>,
) {
    let root = Folder::new("root", None);
    let mut folder_ui_section: Option<FolderUISection> = None;

    let mut content = Content::new(JustifyContent::Start);
    let header = Header::new("Web5 File Manager", Size::Large, None, None, false);
    let page = Page::new(header, content, None);
    let mut interface = Interface::new(false, page);
    let root_node = interface.spawn_under(&mut commands, &theme);

    content.add(&mut commands, |parent| {
        text_input(parent, &theme);
    });

    let ui_section_bundle = content.add(&mut commands, |parent| {
        parent.spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Start,
            width: Val::Percent(100.0),
            ..default()
        }).with_children(|parent| {
            display_files_and_folders(parent, &root, &theme);
        });
    });

    if let Some(ui_section_bundle) = ui_section_bundle {
        folder_ui_section = Some(FolderUISection(ui_section_bundle));
    }

    if let Some(folder_ui_section) = folder_ui_section {
        commands.insert_resource(folder_ui_section);
    }

    let root = Folder::new("root", None);
    commands.insert_resource(RootNode(Some(root_node)));
    commands.insert_resource(root);
    commands.insert_resource(FolderState::new());
}


pub fn display_files_and_folders(
    parent: &mut ChildBuilder,
    folder: &Folder,
    theme: &Theme,
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
                    object(parent, theme, ". .", theme.icons.get("folder"));
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
                    object(parent, theme, name, theme.icons.get("file"));
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
                    object(parent, theme, name, theme.icons.get("folder"));
                });
            });
        }
    });
}


pub fn text_input_system(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_input_query: Query<(Entity, &mut TextInputInactive, &mut BorderColor)>,
    mut events: EventReader<TextInputSubmitEvent>,
    mut editor_query: Query<&mut TextInputValue, With<TextEditor>>,
    theme: Res<Theme>,
) {

    // On Pressed
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for (entity, mut inactive, mut border_color) in &mut text_input_query {
                if entity == interaction_entity {
                    inactive.0 = false;
                    *border_color = theme.colors.outline.primary.into();
                } else {
                    inactive.0 = true;
                    *border_color = theme.colors.outline.secondary.into();
                }
            }
        }
    }

    // On Enter
    for event in events.read() {
        for mut text_input in &mut editor_query {
            text_input.0 = event.value.clone();
            text_input.0.push('\n');
        }
    }
    
}


pub fn text_input (parent: &mut ChildBuilder, theme: &Theme) {
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
            font: theme.fonts.style.text.clone(),
            font_size: theme.fonts.size.md,
            ..default()
        }),
        BorderColor(theme.colors.outline.secondary),
        BackgroundColor(theme.colors.background.primary),
        TextInputTextColor(TextColor(theme.colors.text.primary)),
        TextInputInactive(true),
        TextInputValue("/root/".to_string()),
        BorderRadius::all(Val::Px(8.0)),
        FocusPolicy::Block,
        TextInput,
        SearchBar,
    ));
}