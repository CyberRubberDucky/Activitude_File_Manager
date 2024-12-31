use bevy::prelude::*;
use bevy_ui::FocusPolicy;


use crate::Folder;
use crate::EXPAND;

use crate::popup::TextEditor;

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

/* -------------- Manager -------------- */

pub fn manager(
    mut commands: Commands,
    theme: Res<Theme>,
) {

    let root = Folder::new("root", None);
    let mut folder_ui_section: Option<FolderUISection> = None;
    
    let root_node = commands
        .spawn((
            Node {
                width: EXPAND,
                height: EXPAND,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            BackgroundColor(theme.colors.background.primary)
        )).with_children(|parent| {
            parent.spawn((Node {
                width: EXPAND,
                height: EXPAND,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            }, Interaction::None)).with_children(|parent| {

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
                            font: theme.fonts.style.heading.clone(),
                            font_size: theme.fonts.size.h3,
                            ..default()
                        },
                        TextColor(theme.colors.text.heading),
                    ));
                });

                // === Body === //
                
                parent.spawn(Node {
                    width: EXPAND,
                    height: EXPAND,
                    max_width: Val::Px(512.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(24.0),
                    ..default()
                }).with_children(|parent| {

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
                            font:  theme.fonts.style.text.clone(),
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

                    // === Display all files and folders === //
                    let files_and_folders_node = parent.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Start,
                        width: Val::Percent(100.0),
                        ..default()
                    }).with_children(|parent| {
                        display_files_and_folders(parent, &root, &theme);
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


pub fn display_files_and_folders(
    parent: &mut ChildBuilder,
    folder: &Folder,
    theme: &Res<Theme>,
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
