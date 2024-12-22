
use bevy::prelude::*;
use crate::theme::color::Display;
use crate::components::button::{CustomButton, ButtonWidth, ButtonComponent, ButtonSize, InteractiveState, ButtonStyle};
use crate::FontResources;
use crate::components::text_editor::text_editor;
use crate::components::context::NewFileButton;
use crate::components::context::NewFolderButton;
use bevy_simple_text_input::TextInputValue;
use crate::components::text_editor::TextEditor;

use crate::manager_ui::update_folder_ui;
use crate::folder::FolderUISection;

use crate::theme::icons::Icon;

use crate::folder::File;
use crate::Folder;
use crate::FolderState;

#[derive(Component)]
pub struct Popup;
#[derive(Component)]
pub struct SaveButton;
#[derive(Component)]
pub struct CancelButton;
#[derive(Component)]
pub struct DeleteButton;

pub fn popup(
    commands: &mut Commands,
    fonts: &Res<FontResources>,
    asset_server: &Res<AssetServer>,
    name: &str,
    content: &str,
    file: &File,
) {
    let colors = Display::new();

    // ==== Define Buttons ==== //

    let save = popup_button("Save", InteractiveState::Default, Icon::Save);
    let cancel = popup_button("Cancel", InteractiveState::Default, Icon::Exit);
    let delete = popup_button("Delete", InteractiveState::Default, Icon::Delete);

    // ==== Screen Container ==== //

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            position_type: PositionType::Absolute,
            ..default()
        },
        Popup,
    )).with_children(|parent| {

        // ==== Popup ==== //

        parent.spawn((
            Node {
                width: Val::Px(800.0),
                height: Val::Px(550.0),
                row_gap: Val::Px(16.0),
                border: UiRect::all(Val::Px(1.0)),
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                padding: UiRect {
                    left: Val::Px(48.0),
                    right: Val::Px(48.0),
                    top: Val::Px(16.0),
                    bottom: Val::Px(16.0),
                },
                ..default()
            },
            BorderColor(colors.outline_secondary),
            BackgroundColor(colors.bg_primary),
            BorderRadius::all(Val::Px(8.0)),
        )).with_children(|parent| {
            // ==== Header ==== //

            small_header(parent, fonts, name);

            // ==== Text Input ==== //

            text_editor(parent, fonts, content);

            // ==== Buttons ==== //

            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::End,
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    ..default()
                },
            )).with_children(|parent| {

                // ==== Delete Button ==== //

                parent.spawn((
                    Node::default(),
                    DeleteButton,
                )).with_children(|child| {
                    ButtonComponent::spawn_button(child, asset_server, fonts, delete);
                });

                // ==== Spacer ==== //

                parent.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        ..default()
                    },
                ));

                // ==== Cancel Button ==== //

                parent.spawn((
                    Node::default(),
                    CancelButton,
                )).with_children(|child| {
                    ButtonComponent::spawn_button(child, asset_server, fonts, cancel);
                });

                // ==== Save Button ==== //

                parent.spawn((
                    Node::default(),
                    SaveButton,
                )).with_children(|child| {
                    ButtonComponent::spawn_button(child, asset_server, fonts, save);
                });

            });
        });
    });
}

pub fn popup_b_system(
    mut commands: Commands,
    mut popup_query: Query<(Entity, &Node, &Children), With<Popup>>,
    mut interaction_query: Query<(&Interaction, &Parent), (Changed<Interaction>, With<Button>)>,
    s_query: Query<&SaveButton>,
    c_query: Query<&CancelButton>,
    delete_query: Query<&DeleteButton>, 
    mut query: Query<&mut TextInputValue, With<TextEditor>>,
    mut root: ResMut<Folder>, 
    folder_state: ResMut<FolderState>,
    folder_ui_section: Res<FolderUISection>, 
    fonts: Res<FontResources>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, parent) in &mut interaction_query {
        if let Interaction::Pressed = *interaction {
            if s_query.get(parent.get()).is_ok() || c_query.get(parent.get()).is_ok() || delete_query.get(parent.get()).is_ok() {
                if s_query.get(parent.get()).is_ok() {

                    // ==== Saving File ===== //

                    for mut text_input in &mut query {

                        // ==== Get Text Input ===== //

                        if let Some(current_folder) = root.find_folder_mut(&folder_state.current_folder) {
                            if let Some(file_name) = &folder_state.current_file_name {
                                if let Some(file) = current_folder.get_file_mut(file_name) {

                                    // ==== Set File Content To Text Input ===== //

                                    file.content = text_input.0.clone();
                                }
                            }
                        }
                    }
                } else if c_query.get(parent.get()).is_ok() {

                    // ==== Cancel (Do nothing) ===== //
                } else if delete_query.get(parent.get()).is_ok() {

                    // ==== Deleting File or Folder ===== //

                    if let Some(file_name) = &folder_state.current_file_name {
                        if let Some(current_folder) = root.find_folder_mut(&folder_state.current_folder) {
                            current_folder.files.remove(file_name);
                        }
                    } else {
                        let folder_name = folder_state.current_folder.clone();
                        if let Some(current_folder) = root.find_folder_mut(&folder_name) {
                            current_folder.subfolders.remove(&folder_name);
                        }
                    }
                }

                // ==== Close File Popup ===== //

                for (entity, _, children) in popup_query.iter_mut() {
                    for child in children.iter() {
                        commands.entity(*child).despawn_recursive();
                    }
                    commands.entity(entity).despawn_recursive();
                }

                // ==== Update UI  ===== //

                if let Some(current_folder) = root.find_folder_mut(&folder_state.current_folder) {
                    update_folder_ui(&mut commands, folder_ui_section.0, current_folder, &fonts, &asset_server);
                }
            }
        }
    }
}



fn popup_button(label: &str, status: InteractiveState, icon: Icon) -> CustomButton {
    CustomButton::new(
        label,
        Some(icon),
        None,
        ButtonStyle::Secondary,
        ButtonWidth::Hug,
        ButtonSize::Medium,
        status,
        JustifyContent::Center,
        true,
        false,
    )
}

pub fn small_header (
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
    title: &str, 
) {
    let colors = Display::new();

    let node = Node {
        width: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Row,
        padding: UiRect::all(Val::Px(12.0)),
        ..default()
    };

    parent.spawn(node).with_children(|parent| {
        parent.spawn((
            Text::new(title),
            TextFont {
                font: fonts.style.heading.clone(),
                font_size: fonts.size.h4,
                ..default()
            },
            TextColor(colors.text_heading),
        ));
    });
}