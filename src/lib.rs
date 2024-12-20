#![allow(unused)]

mod home;
mod filemanager;


pub mod primitives { 
    pub mod button; 
    pub mod profile_photo;
}

pub mod theme { 
    pub mod icons; 
    pub mod color; 
    pub mod fonts; 
}

pub mod components {
    pub mod balance_display;
    pub mod navigator; 
    pub mod text_input;
    pub mod tip_button;
}

pub mod interface {
    pub mod bumper;
    pub mod header;
    pub mod interfaces;
}

use bevy::prelude::*;
use bevy_ui::prelude::*;
use bevy::input::mouse::MouseButton;


use theme::{
    color::Colors,
    fonts::setup_fonts
};

use bevy_simple_text_input::{TextInputPlugin, TextInputSystem};
use crate::primitives::button::button_system;

use crate::home::{OnHomeScreen, home_setup};
use crate::filemanager::{OnAddressScreen, address_setup};
use crate::components::text_input::focus;
use crate::theme::color::Display;

use crate::primitives::button::{
    CustomButton, 
    ButtonWidth, 
    ButtonComponent, 
    ButtonSize, 
    InteractiveState, 
    ButtonStyle, 
    primary_default,
};

use crate::theme::icons::Icon;

use bevy::window::PrimaryWindow;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Web5 File Manager".into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(TextInputPlugin)
        .add_systems(Update, focus.before(TextInputSystem))
        .insert_resource(ClearColor(Colors::tapa().shade1000)) 
        .add_plugins((menu_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
pub enum NavigateTo {
    Home,
    Address,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PageState {
    Home,
    Address,
    #[default]
    Disabled,
}
use crate::theme::fonts::FontResources;

pub fn menu_plugin(app: &mut App) {
    app
        .init_state::<PageState>()
        .add_systems(OnEnter(GameState::Menu), startup_setup)
        .add_systems(OnEnter(PageState::Home), home_setup)
        .add_systems(OnExit(PageState::Home), despawn_screen::<OnHomeScreen>)
        .add_systems(OnEnter(PageState::Address), address_setup)
        .add_systems(OnExit(PageState::Address), despawn_screen::<OnAddressScreen>)
        .add_systems(PreStartup, setup_fonts)
        .add_systems(Update, button_system)
        .add_systems(Update, context_menu)
        .add_systems(Update, (menu_action, button_system).run_if(in_state(GameState::Menu)));
}

pub fn startup_setup(mut menu_state: ResMut<NextState<PageState>>) {
    menu_state.set(PageState::Address);
}

pub fn menu_action(
    interaction_query: Query<
        (&Interaction, &NavigateTo),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<PageState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                NavigateTo::Home => menu_state.set(PageState::Home),
                NavigateTo::Address => menu_state.set(PageState::Address)
            }
        }
    }
}

#[derive(Component)]
pub struct ContextMenu;

fn context_menu(
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
                    ButtonComponent::spawn_button(child, &asset_server, &fonts, folder);
                    child.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(1.0),
                            ..default()
                        },
                        BackgroundColor(colors.outline_secondary),
                    ));
                    ButtonComponent::spawn_button(child, &asset_server, &fonts, file);
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
        NavigateTo::Address,
        JustifyContent::Start,
        true,
        false,
    )
}
