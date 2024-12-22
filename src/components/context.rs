use bevy::prelude::*;
use bevy_ui::prelude::*;
use bevy::input::mouse::MouseButton;

use bevy_simple_text_input::{TextInputPlugin, TextInputSystem};
use crate::interface::button::button_system;

use crate::filemanager::{OnFileManagerScreen, manager};
use crate::components::text_input::focus;
use crate::components::text_editor::listener;
use crate::theme::color::Display;

use crate::interface::button::{
    CustomButton, 
    ButtonWidth, 
    ButtonComponent, 
    ButtonSize, 
    InteractiveState, 
    ButtonStyle, 
    primary_default,
};


use crate::FontResources;
use crate::NavigateTo;
use crate::theme::icons::Icon;

use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct ContextMenu;
#[derive(Component)]
pub struct ContextButton;

pub fn context_menu(
    mut commands: Commands,
    fonts: Res<FontResources>,
    asset_server: Res<AssetServer>, 

    query_window: Query<&Window, With<PrimaryWindow>>,
    mouse_button: Res<ButtonInput<MouseButton>>,

    mut context_menu_query: Query<(Entity, &Node, &Children), With<ContextMenu>>,
    mut interaction_query: Query<&mut Node, Without<ContextMenu>>,
) {
    let window = query_window.single();
    let colors = Display::new();

    if let Some(cursor_position) = window.cursor_position() {
        if mouse_button.just_pressed(MouseButton::Right) {
            if context_menu_query.is_empty() {
                let (height, width) = (
                    cursor_position.y / window.height(),
                    cursor_position.x / window.width(),
                );

                let folder = context_button("Create Folder", InteractiveState::Default, Icon::Folder);
                let file = context_button("Create File", InteractiveState::Default, Icon::File);
                let delete = context_button("Delete", InteractiveState::Default, Icon::Exit);

                commands.spawn((
                    Node {
                        left: Val::Percent(width * 100.0),
                        top: Val::Percent(height * 100.0),
                        width: Val::Px(300.0),
                        border: UiRect::all(Val::Px(1.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BorderColor(colors.outline_secondary),
                    BackgroundColor(colors.bg_primary),
                    BorderRadius::all(Val::Px(8.0)),
                    ContextMenu,
                )).with_children(|child| {
                    ButtonComponent::spawn_button(child, &asset_server, &fonts, delete);
                    ButtonComponent::spawn_button(child, &asset_server, &fonts, folder);
                    child.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(1.0),
                            ..default()
                        },
                        BackgroundColor(colors.outline_secondary),
                    ));
                    child.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            ..default()
                        },
                        ContextButton
                    )).with_children(|parent| {
                        ButtonComponent::spawn_button(parent, &asset_server, &fonts, file);
                    });
                    
                });
            }
        }

        if mouse_button.just_pressed(MouseButton::Left) {
            for (entity, node, children) in context_menu_query.iter_mut() {
                let left = match node.left {
                    Val::Px(x) => x,
                    Val::Percent(p) => p * window.width(),
                    _ => 0.0, 
                };

                let top = match node.top {
                    Val::Px(y) => y,
                    Val::Percent(p) => p * window.height(),
                    _ => 0.0, 
                };

                let right = left + match node.width {
                    Val::Px(w) => w,
                    Val::Percent(p) => p * window.width(),
                    _ => 0.0,
                };

                let bottom = top + match node.height {
                    Val::Px(h) => h,
                    Val::Percent(p) => p * window.height(),
                    _ => 0.0,
                };

                let context_menu_rect = Rect {
                    min: Vec2::new(left, top),
                    max: Vec2::new(right, bottom),
                };

                if !contains(&context_menu_rect, cursor_position) {
                    for child in children.iter() {
                        commands.entity(*child).despawn_recursive();
                    }
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}

fn contains(rect: &Rect, point: Vec2) -> bool {
    point.x >= rect.min.x && point.x <= rect.max.x && point.y >= rect.min.y && point.y <= rect.max.y
}

fn context_button (label: &str, status: InteractiveState, icon: Icon) -> CustomButton {
    CustomButton::new(
        label,
        Some(icon),
        None,
        ButtonStyle::Ghost,
        ButtonWidth::Expand,
        ButtonSize::Medium,
        status,
        NavigateTo::None,
        JustifyContent::Start,
        true,
        false
    )
}
